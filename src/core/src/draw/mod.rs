use gl;
use gl::types::*;
use std;

pub mod vertex;
pub mod uniform;

pub mod geometry;
pub use self::geometry::{Geometry, PlainGeometry, InstancedGeometry};

pub mod shader;
pub use self::shader::*;

pub mod texture;
pub use self::texture::*;

use commons::*;

pub trait Target {
    fn clear(&mut self, color: Color);
    fn draw<G: Geometry, U: uniform::Data>(&mut self,
                                           geometry: &G,
                                           shader: &Shader,
                                           uniforms: &U);
}

unsafe fn prepare_vertex_attributes<B>(shader: &Shader, data: &B, attrib_divisor: GLuint)
    where B: vertex::BufferView
{
    if data.as_slice().len() == 0 {
        return;
    }
    gl::BindBuffer(gl::ARRAY_BUFFER, data.get_original_buffer().handle);
    struct AttributeWalker<'a, D: 'a + vertex::Data> {
        shader: &'a Shader,
        offset: usize,
        data: &'a D,
        attrib_divisor: GLuint,
    }
    impl<'a, D: vertex::Data> vertex::AttributeConsumer for AttributeWalker<'a, D> {
        fn consume<A: vertex::Attribute>(&mut self, name: &str, value: &A) {
            unsafe {
                let location =
                    gl::GetAttribLocation(self.shader.handle,
                                          std::ffi::CString::new(name).unwrap().as_ptr()); // TODO: cache
                if location == -1 {
                    return;
                }
                let location = location as GLuint;
                gl::EnableVertexAttribArray(location);
                let offset = self.offset + value as *const _ as usize -
                             self.data as *const _ as usize;
                gl::VertexAttribPointer(location,
                                        A::get_gl_size(),
                                        A::get_gl_type(),
                                        gl::FALSE,
                                        std::mem::size_of::<D>() as GLsizei,
                                        offset as *const GLvoid);
                gl::VertexAttribDivisor(location, self.attrib_divisor);
            }
        }
    }
    let original_data = vertex::BufferView::as_slice(data.get_original_buffer());
    let data = data.as_slice();
    <B::Data as vertex::Data>::walk_attributes(&data[0],
                                               &mut AttributeWalker {
                                                        shader,
                                                        offset: data.as_ptr() as usize -
                                                                original_data.as_ptr() as usize,
                                                        data: &data[0],
                                                        attrib_divisor,
                                                    });
}

unsafe fn prepare_geometry_attributes<G: Geometry>(shader: &Shader, geometry: &G) {
    struct DataWalker<'a> {
        shader: &'a Shader,
        attrib_divisor: GLuint,
    }
    impl<'a> vertex::DataConsumer for DataWalker<'a> {
        fn consume<B: vertex::BufferView>(&mut self, data: &B) {
            unsafe {
                prepare_vertex_attributes(self.shader, data, self.attrib_divisor);
                if self.attrib_divisor == 0 {
                    self.attrib_divisor = 1;
                } else {
                    self.attrib_divisor *= data.as_slice().len() as GLuint;
                }
            }
        }
    }
    geometry.walk_data(&mut DataWalker {
                                shader,
                                attrib_divisor: 0,
                            });
}

fn apply_uniforms<U: uniform::Data>(shader: &Shader, uniforms: &U) {
    use std::marker::PhantomData;
    struct Walker<U: uniform::Data>(GLuint, usize, PhantomData<U>);
    impl<U: uniform::Data> uniform::ValueConsumer for Walker<U> {
        fn consume<V: uniform::Value>(&mut self, name: &str, value: &V) {
            unsafe {
                let location =
                    gl::GetUniformLocation(self.0, std::ffi::CString::new(name).unwrap().as_ptr());
                if location >= 0 {
                    value.apply(location, &mut self.1);
                }
            }
        }
    }
    let mut walker = Walker::<U>(shader.handle, 0, PhantomData);
    uniforms.walk(&mut walker);
}

pub struct Screen;

impl Target for Screen {
    fn clear(&mut self, color: Color) {
        unsafe {
            gl::ClearColor(color.red, color.green, color.blue, color.alpha);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
    fn draw<G: Geometry, U: uniform::Data>(&mut self,
                                           geometry: &G,
                                           shader: &Shader,
                                           uniforms: &U) {
        use draw::geometry::Mode::*;
        unsafe {
            gl::UseProgram(shader.handle);
            let mut vao: GLuint = std::mem::uninitialized();
            gl::GenVertexArrays(1, &mut vao as *mut _);
            gl::BindVertexArray(vao);
            prepare_geometry_attributes(shader, geometry);
            apply_uniforms(shader, uniforms);
            let gl_mode = match geometry.get_mode() {
                Points => gl::POINTS,
                Lines => gl::LINES,
                LineStrip => gl::LINE_STRIP,
                Triangles => gl::TRIANGLES,
                TriangleStrip => gl::TRIANGLE_STRIP,
                TriangleFan => gl::TRIANGLE_FAN,
            };

            struct CounterWalker {
                instance_count: GLsizei,
                vertex_count: Option<GLsizei>,
            }
            impl vertex::DataConsumer for CounterWalker {
                fn consume<B: vertex::BufferView>(&mut self, data: &B) {
                    if let None = self.vertex_count {
                        self.vertex_count = Some(data.as_slice().len() as GLsizei);
                    } else {
                        self.instance_count *= data.as_slice().len() as GLsizei;
                    }
                }
            }
            let (vertex_count, instance_count) = {
                let mut counter_walker = CounterWalker {
                    instance_count: 1,
                    vertex_count: None,
                };
                geometry.walk_data(&mut counter_walker);
                (counter_walker.vertex_count.unwrap(), counter_walker.instance_count)
            };
            gl::DrawArraysInstanced(gl_mode, 0, vertex_count, instance_count);
            gl::DeleteVertexArrays(1, &vao);
        }
    }
}