use gl;
use gl::types::*;
use std;

pub mod vertex;
pub mod uniform;

pub mod geometry;
pub use self::geometry::Geometry;

pub mod shader;
pub use self::shader::*;

use common::*;

#[allow(dead_code)]
fn check_gl_error() {
    unsafe {
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            panic!("OpenGL error: {}", error);
        }
    }
}

pub trait Target {
    fn clear(&mut self, color: Color);
    fn draw<V: vertex::Data, U: uniform::Data>(&mut self,
                                               geometry: &Geometry<V>,
                                               shader: &Shader,
                                               uniforms: &U);
}

unsafe fn prepare_attributes<V: vertex::Data>(shader: &Shader) {
    use std::marker::PhantomData;
    struct Walker<V: vertex::Data>(GLuint, usize, PhantomData<V>);
    impl<V: vertex::Data> vertex::AttributeConsumer for Walker<V> {
        fn consume<A: vertex::Attribute>(&mut self, name: &str, value: &A) {
            unsafe {
                let location =
                    gl::GetAttribLocation(self.0, std::ffi::CString::new(name).unwrap().as_ptr()) as
                    GLuint; // TODO: cache
                gl::EnableVertexAttribArray(location);
                let offset = value as *const _ as usize - self.1;
                gl::VertexAttribPointer(location,
                                        A::get_gl_size(),
                                        A::get_gl_type(),
                                        gl::FALSE,
                                        std::mem::size_of::<V>() as GLsizei,
                                        offset as *const GLvoid);
            }
        }
    }
    let fake_value = std::mem::uninitialized();
    let mut walker = Walker::<V>(shader.handle, &fake_value as *const _ as usize, PhantomData);
    V::walk_attributes(&fake_value, &mut walker);
    std::mem::forget(fake_value);
}

fn apply_uniforms<U: uniform::Data>(shader: &Shader, uniforms: &U) {
    use std::marker::PhantomData;
    struct Walker<U: uniform::Data>(GLuint, usize, PhantomData<U>);
    impl<U: uniform::Data> uniform::ValueConsumer for Walker<U> {
        fn consume<V: uniform::Value>(&mut self, name: &str, value: &V) {
            unsafe {
                let location =
                    gl::GetUniformLocation(self.0, std::ffi::CString::new(name).unwrap().as_ptr());
                value.apply(location, &mut self.1);
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
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
    fn draw<V: vertex::Data, U: uniform::Data>(&mut self,
                                               geometry: &Geometry<V>,
                                               shader: &Shader,
                                               uniforms: &U) {
        use draw::geometry::Mode::*;
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, geometry.get_handle());
            gl::UseProgram(shader.handle);
            prepare_attributes::<V>(shader);
            apply_uniforms(shader, uniforms);
            gl::DrawArrays(match geometry.mode {
                               Points => gl::POINTS,
                               Lines => gl::LINES,
                               LineStrip => gl::LINE_STRIP,
                               Triangles => gl::TRIANGLES,
                               TriangleStrip => gl::TRIANGLE_STRIP,
                               TriangleFan => gl::TRIANGLE_FAN,
                           },
                           0,
                           geometry.len() as GLsizei);
        }
    }
}