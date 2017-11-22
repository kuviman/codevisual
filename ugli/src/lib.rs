#![deny(warnings)]

#[macro_use]
extern crate prelude;

#[doc(hidden)]
extern crate gl;

#[cfg(target_os = "emscripten")]
extern crate webby;

#[cfg(not(target_os = "emscripten"))]
#[doc(hidden)]
extern crate image;

#[cfg(not(target_os = "emscripten"))]
extern crate glutin;

pub(crate) use prelude::*;
pub(crate) use gl::types::*;
#[cfg(target_os = "emscripten")]
pub(crate) use webby::emscripten;

mod context;
mod shader;
mod program;
mod texture;
mod renderbuffer;
mod framebuffer;
mod draw;
mod vertex;
mod uniform;
mod quad;
mod cube;

pub use context::*;
pub use shader::*;
pub use program::*;
pub use texture::*;
pub use renderbuffer::*;
pub use framebuffer::*;
pub use draw::*;
pub use vertex::*;
pub use uniform::*;
pub use quad::*;
pub use cube::*;

mod private {
    pub trait Sealed {}
}

pub(crate) use private::*;

pub unsafe trait Pixel: Sealed {
    fn possible_texture(context: &Context) -> bool;
    const GL_TEXTURE_FORMAT: GLenum;
    const GL_FRAMEBUFFER_FORMAT: GLenum;
    const GL_TEXTURE_TYPE: GLenum;
}

unsafe impl Pixel for Color {
    fn possible_texture(_: &Context) -> bool { true }
    const GL_TEXTURE_FORMAT: GLenum = gl::RGBA;
    const GL_FRAMEBUFFER_FORMAT: GLenum = gl::RGBA;
    const GL_TEXTURE_TYPE: GLenum = gl::UNSIGNED_BYTE;
}

impl Sealed for Color {}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct DepthComponent(GLfloat);

unsafe impl Pixel for DepthComponent {
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

fn check_gl_error() {
    assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR, "OpenGL error");
}

pub fn sync() {
    check_gl_error();
    unsafe {
        gl::Finish();
    }
}