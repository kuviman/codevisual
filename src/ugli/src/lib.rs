#[macro_use]
extern crate vpl;
use vpl::*;

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

mod framebuffer;
pub use framebuffer::*;

mod draw;
pub use draw::*;

mod vertex;
pub use vertex::*;

mod uniform;
pub use uniform::*;

fn check_gl_error() {
    assert_eq!(unsafe { gl::GetError() }, gl::NO_ERROR);
}
