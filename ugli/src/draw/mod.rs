use ::*;

mod parameters;

pub use self::parameters::*;

pub enum DrawMode {
    Points,
    Lines {
        line_width: f32,
    },
    LineStrip {
        line_width: f32,
    },
    LineLoop {
        line_width: f32,
    },
    Triangles,
    TriangleStrip,
    TriangleFan,
}

pub fn clear(framebuffer: &mut Framebuffer,
             color: Option<Color>,
             depth: Option<f32>) {
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
            gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
        }
    }
    if let Some(depth) = depth {
        flags |= gl::DEPTH_BUFFER_BIT;
        unsafe {
            gl::ClearDepthf(depth as GLfloat);
            gl::DepthMask(gl::TRUE);
        }
    }
    unsafe {
        gl::Clear(flags);
    }
}

pub fn draw<V, U, DP>(framebuffer: &mut Framebuffer,
                      program: &Program,
                      mode: DrawMode,
                      vertices: V,
                      uniforms: U,
                      draw_parameters: DP)
    where V: VertexDataSource,
          U: Uniforms,
          DP: std::borrow::Borrow<DrawParameters> {
    framebuffer.fbo.bind();
    let draw_parameters = draw_parameters.borrow();
    draw_parameters.apply(framebuffer.get_size());
    program.bind();
    let uniforms = (SingleUniform::new("u_framebuffer_size", framebuffer.get_size()), uniforms);
    unsafe { UNIFORM_TEXTURE_COUNT = 0; }
    uniforms.walk_uniforms(&mut UC {
        program,
    });

    #[cfg(not(target_os = "emscripten"))]
    let vao = VAO::new();
    #[cfg(not(target_os = "emscripten"))]
        vao.bind();

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
        DrawMode::Lines { line_width } => {
            unsafe { gl::LineWidth(line_width as GLfloat); }
            assert!(vertex_count % 2 == 0);
            gl::LINES
        }
        DrawMode::LineStrip { line_width } => {
            unsafe { gl::LineWidth(line_width as GLfloat); }
            assert!(vertex_count >= 2);
            gl::LINE_STRIP
        }
        DrawMode::LineLoop { line_width } => {
            unsafe { gl::LineWidth(line_width as GLfloat); }
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
    }
    impl<'a> UniformConsumer for UC<'a> {
        fn consume<U: Uniform>(&mut self, name: &str, uniform: &U) {
            if let Some(uniform_info) = self.program.uniforms.get(name) {
                uniform.apply(uniform_info);
            }
            uniform.walk_extra(name, self);
        }
    }

    #[cfg(not(target_os = "emscripten"))]
    struct VAO {
        handle: GLuint,
    }

    #[cfg(not(target_os = "emscripten"))]
    impl VAO {
        fn new() -> Self {
            Self {
                handle: unsafe {
                    let mut handle: GLuint = mem::uninitialized();
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

    #[cfg(not(target_os = "emscripten"))]
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
        fn consume<'b, D: Vertex + 'b, T: IntoVertexBufferSlice<'b, D>>(&mut self, data: T, divisor: Option<usize>) {
            let data = data.into_slice();
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
            let sample: D = unsafe { mem::uninitialized() };
            data.buffer.bind();
            sample.walk_attributes(VAC {
                sample: &sample,
                divisor,
                program: self.program,
                offset: data.range.start * mem::size_of::<D>(),
            });
            mem::forget(sample);
            struct VAC<'a, D: Vertex + 'a> {
                offset: usize,
                sample: &'a D,
                divisor: Option<usize>,
                program: &'a Program,
            }
            impl<'a, D: Vertex> VertexAttributeConsumer for VAC<'a, D> {
                fn consume<A: VertexAttribute>(&mut self, name: &str, attribute: &A) {
                    if let Some(attribute_info) = self.program.attributes.get(name) {
                        let offset = self.offset + attribute as *const _ as usize -
                            self.sample as *const _ as usize;
                        unsafe {
                            gl::EnableVertexAttribArray(attribute_info.location);
                            gl::VertexAttribPointer(
                                attribute_info.location,
                                A::SIZE as GLint,
                                A::TYPE as GLenum,
                                gl::FALSE,
                                mem::size_of::<D>() as GLsizei,
                                offset as *const GLvoid,
                            );
                            if let Some(divisor) = self.divisor {
                                gl::VertexAttribDivisor(attribute_info.location, divisor as GLuint);
                            } else {
                                gl::VertexAttribDivisor(attribute_info.location, 0);
                            }
                        }
                    }
                }
            }
        }
    }
}
