use gl;
use gl::types::*;
use std;

pub mod geometry;
pub use self::geometry::*;

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

pub enum Command<'a, 'b> {
    Clear { color: Color },
    Object {
        geometry: &'a GeometryBuffer,
        shader: &'b Shader,
    },
}

unsafe fn prepare_attributes(shader: GLuint, attributes: &[VertexAttribute]) {
    let mut stride: GLsizei = 0;
    for attribute in attributes {
        stride += attribute.raw_size;
    }
    for attribute in attributes {
        let location = gl::GetAttribLocation(shader, attribute.name.as_ptr()) as GLuint; // TODO: cache
        gl::EnableVertexAttribArray(location);
        gl::VertexAttribPointer(location,
                                attribute.size,
                                attribute.gl_type,
                                attribute.normalized,
                                stride,
                                std::ptr::null());
    }
}

pub fn immediate(command: Command) {
    match command {
        Command::Clear { color } => unsafe {
            gl::ClearColor(color.red, color.green, color.blue, color.alpha);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        },
        Command::Object { geometry, shader } => unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, geometry.handle);
            gl::UseProgram(shader.handle);
            prepare_attributes(shader.handle, &geometry.attributes);
            gl::DrawArrays(geometry.mode, 0, geometry.element_count);
        },
    }
}