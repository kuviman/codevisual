use ::*;

mod parameters;
pub use self::parameters::*;

use framebuffer::attachment;

pub enum DrawMode {
    Points,
    Lines,
    LineStrip,
    LineLoop,
    Triangles,
    TriangleStrip,
    TriangleFan,
}

pub fn clear<FR, FC>(
    framebuffer: &mut Framebuffer<FR, attachment::HasAccess, FC>,
    color: Option<Color>,
    depth: Option<f32>,
) where
    FR: attachment::Access,
    FC: attachment::Color<ReadAccess = FR, WriteAccess = attachment::HasAccess>,
{
    framebuffer.fbo.bind();
    let mut flags = 0;
    if let Some(color) = color {
        flags |= gl::COLOR_BUFFER_BIT;
        unsafe {
            gl::ClearColor(
                color.red as GLfloat,
                color.green as GLfloat,
                color.blue as GLfloat,
                color.alpha as GLfloat,
            );
        }
    }
    if let Some(depth) = depth {
        flags |= gl::DEPTH_BUFFER_BIT;
        unsafe {
            gl::ClearDepthf(depth as GLfloat);
        }
    }
    unsafe {
        gl::Clear(flags);
    }
}

pub fn draw<FR, FC, V, U>(
    framebuffer: &mut Framebuffer<FR, attachment::HasAccess, FC>,
    program: &Program,
    mode: DrawMode,
    vertices: &V,
    uniforms: &U,
    draw_parameters: &DrawParameters,
) where
    FR: attachment::Access,
    FC: attachment::Color<ReadAccess = FR, WriteAccess = attachment::HasAccess>,
    V: VertexDataSource,
    U: UniformStorage,
{
    framebuffer.fbo.bind();
    unsafe {
        let size = framebuffer.get_size();
        gl::Viewport(0, 0, size.x as GLsizei, size.y as GLsizei);
    }
    draw_parameters.apply();
    program.bind();
    uniforms.walk_uniforms(&mut UC {
        program,
        texture_count: 0,
    });

    #[cfg(not(target_os = "emscripten"))]
    let vao = VAO::new();
    #[cfg(not(target_os = "emscripten"))] vao.bind();

    let mut vertex_count = None;
    let mut instance_count = None;
    vertices.walk_data(VDC {
        program,
        vertex_count: &mut vertex_count,
        instance_count: &mut instance_count,
    });
    let vertex_count = vertex_count.unwrap();
    if vertex_count == 0 {
        return;
    }
    let gl_mode = match mode {
        DrawMode::Points => gl::POINTS,
        DrawMode::Lines => {
            assert!(vertex_count % 2 == 0);
            gl::LINES
        }
        DrawMode::LineStrip => {
            assert!(vertex_count >= 2);
            gl::LINE_STRIP
        }
        DrawMode::LineLoop => {
            assert!(vertex_count >= 3);
            gl::LINE_LOOP
        }
        DrawMode::Triangles => {
            assert!(vertex_count % 3 == 0);
            gl::TRIANGLES
        }
        DrawMode::TriangleStrip => {
            assert!(vertex_count >= 3);
            gl::TRIANGLE_STRIP
        }
        DrawMode::TriangleFan => {
            assert!(vertex_count >= 3);
            gl::TRIANGLE_FAN
        }
    };

    if let Some(instance_count) = instance_count {
        if instance_count == 0 {
            return;
        }
        unsafe {
            gl::DrawArraysInstanced(
                gl_mode,
                0,
                vertex_count as GLsizei,
                instance_count as GLsizei,
            );
        }
    } else {
        unsafe {
            gl::DrawArrays(gl_mode, 0, vertex_count as GLsizei);
        }
    }
    check_gl_error();

    struct UC<'a> {
        program: &'a Program,
        texture_count: usize,
    }
    impl<'a> UniformConsumer for UC<'a> {
        fn consume<U: Uniform>(&mut self, name: &str, uniform: &U) {
            let location = unsafe {
                gl::GetUniformLocation(
                    self.program.handle,
                    std::ffi::CString::new(name).unwrap().as_ptr(),
                )
            };
            if location >= 0 {
                uniform.apply(UniformLocation {
                    location,
                    texture_count: &mut self.texture_count,
                });
            }
        }
    }

    struct VAO {
        handle: GLuint,
    }
    impl VAO {
        fn new() -> Self {
            Self {
                handle: unsafe {
                    let mut handle: GLuint = std::mem::uninitialized();
                    gl::GenVertexArrays(1, &mut handle);
                    handle
                },
            }
        }
        fn bind(&self) {
            unsafe {
                gl::BindVertexArray(self.handle);
            }
        }
    }
    impl Drop for VAO {
        fn drop(&mut self) {
            unsafe {
                gl::DeleteVertexArrays(1, &self.handle);
            }
        }
    }

    struct VDC<'a> {
        program: &'a Program,
        vertex_count: &'a mut Option<usize>,
        instance_count: &'a mut Option<usize>,
    }
    impl<'a> VertexDataConsumer for VDC<'a> {
        fn consume<D>(&mut self, data: &VertexBufferSlice<D>, divisor: Option<usize>)
        where
            D: VertexData,
        {
            if let Some(divisor) = divisor {
                let instance_count = data.len() * divisor;
                if let Some(current_instance_count) = *self.instance_count {
                    assert_eq!(current_instance_count, instance_count);
                } else {
                    *self.instance_count = Some(instance_count);
                }
            } else {
                if let Some(current_vertex_count) = *self.vertex_count {
                    assert_eq!(current_vertex_count, data.len());
                } else {
                    *self.vertex_count = Some(data.len());
                }
            }
            let sample: D = unsafe { std::mem::uninitialized() };
            data.buffer.bind();
            sample.walk_attributes(VAC {
                sample: &sample,
                divisor,
                program: self.program,
                offset: data.range.start * std::mem::size_of::<D>(),
            });
            std::mem::forget(sample);
            struct VAC<'a, D: VertexData + 'a> {
                offset: usize,
                sample: &'a D,
                divisor: Option<usize>,
                program: &'a Program,
            }
            impl<'a, D: VertexData> VertexAttributeConsumer for VAC<'a, D> {
                fn consume<A: VertexAttribute>(&mut self, name: &str, attribute: &A) {
                    let location = unsafe {
                        gl::GetAttribLocation(
                            self.program.handle,
                            std::ffi::CString::new(name).unwrap().as_ptr(),
                        )
                    };
                    if location == -1 {
                        return;
                    }
                    let location = location as GLuint;
                    let gl_type = A::get_gl_type();
                    let offset = self.offset + attribute as *const _ as usize -
                        self.sample as *const _ as usize;
                    unsafe {
                        gl::EnableVertexAttribArray(location);
                        gl::VertexAttribPointer(
                            location,
                            gl_type.gl_size,
                            gl_type.gl_type,
                            gl::FALSE,
                            std::mem::size_of::<D>() as GLsizei,
                            offset as *const GLvoid,
                        );
                        if let Some(divisor) = self.divisor {
                            gl::VertexAttribDivisor(location, divisor as GLuint);
                        } else {
                            gl::VertexAttribDivisor(location, 0);
                        }
                    }
                }
            }
        }
    }
}