#![deny(warnings)]

#[macro_use]
extern crate prelude;

pub ( crate ) use prelude::*;

extern crate gl;

use gl::types::*;

#[cfg(not(target_os = "emscripten"))]
extern crate image;

mod context;

pub use context::*;

mod shader;

pub use shader::*;

mod program;

pub use program::*;

mod texture;

pub use texture::*;

mod renderbuffer;

pub use renderbuffer::*;

mod framebuffer;

pub use framebuffer::*;

mod draw;

pub use draw::*;

mod vertex;

pub use vertex::*;

mod uniform;

pub use uniform::*;

mod quad;

pub use quad::*;

mod cube;

pub use cube::*;

mod private {
    pub trait Sealed {}
}

pub ( crate ) use private::*;

pub trait Pixel: Sealed {
    fn possible_texture(context: &Context) -> bool;
    const GL_TEXTURE_FORMAT: GLenum;
    const GL_FRAMEBUFFER_FORMAT: GLenum;
    const GL_TEXTURE_TYPE: GLenum;
}

impl Pixel for Color {
    fn possible_texture(_: &Context) -> bool { true }
    const GL_TEXTURE_FORMAT: GLenum = gl::RGBA;
    const GL_FRAMEBUFFER_FORMAT: GLenum = gl::RGBA;
    const GL_TEXTURE_TYPE: GLenum = gl::UNSIGNED_BYTE;
}

impl Sealed for Color {}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct DepthComponent(GLfloat);

impl Pixel for DepthComponent {
    fn possible_texture(context: &Context) -> bool {
        #![allow(unused_variables)]

        #[cfg(target_os = "emscripten")]
        return unsafe {
            type EmscriptenContext = c_int;
            extern "C" {
                fn emscripten_webgl_get_current_context() -> EmscriptenContext;
                fn emscripten_webgl_enable_extension(context: EmscriptenContext, name: *const c_char) -> c_int;
            }
            emscripten_webgl_enable_extension(
                emscripten_webgl_get_current_context(),
                CString::new("WEBGL_depth_texture").unwrap().as_ptr()
            ) != 0
        };

        #[cfg(not(target_os = "emscripten"))]
        return true; // TODO: maybe not always available?
    }

    const GL_TEXTURE_FORMAT: GLenum = gl::DEPTH_COMPONENT;
    #[cfg(not(target_os = "emscripten"))]
    const GL_FRAMEBUFFER_FORMAT: GLenum = gl::DEPTH_COMPONENT;
    #[cfg(target_os = "emscripten")]
    const GL_FRAMEBUFFER_FORMAT: GLenum = gl::DEPTH_COMPONENT16;
    const GL_TEXTURE_TYPE: GLenum = gl::UNSIGNED_INT;
}

impl Sealed for DepthComponent {}

pub fn check_gl_error() {
    assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR, "OpenGL error");
}