#![deny(warnings)]

extern crate prelude;
extern crate gl;
#[cfg(target_os = "emscripten")]
extern crate webby;
#[cfg(not(target_os = "emscripten"))]
extern crate image;
#[cfg(not(target_os = "emscripten"))]
extern crate glutin;
#[allow(unused_imports)]
#[macro_use]
extern crate ugli_derive;

pub use ugli_derive::*;

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

pub use context::*;
pub use shader::*;
pub use program::*;
pub use texture::*;
pub use renderbuffer::*;
pub use framebuffer::*;
pub use draw::*;
pub use vertex::*;
pub use uniform::*;

pub unsafe trait Pixel {
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

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct DepthComponent(GLfloat);

unsafe impl Pixel for DepthComponent {
    fn possible_texture(context: &Context) -> bool {
        #![allow(unused_variables)]

        #[cfg(target_os = "emscripten")]
            return context.webgl_context.enable_extension("WEBGL_depth_texture");

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

fn gl_bool(b: bool) -> GLboolean {
    if b { gl::TRUE } else { gl::FALSE }
}

fn check_gl_error() {
    // TODO: text instead of just code
    assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR, "OpenGL error");
}

#[macro_export]
macro_rules! uniforms {
    () => {
        ()
    };
    ($name:ident : $value:expr) => {
        $crate::SingleUniform::new(stringify!($name), $value)
    };
    ($name:ident : $value:expr, $($names:ident : $values:expr),+) => {
        (uniforms!($name : $value), uniforms!($($names : $values),+))
    };
    ($($name:ident : $value:expr),*,) => {
        uniforms!($($name : $value),*)
    }
}