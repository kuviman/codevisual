use std;
use gl::types::*;
use gl;

pub struct Texture {
    handle: GLuint,
}

impl Texture {
    pub fn load(path: &str) -> Result<Self, ::Error> {
        ::Application::get_instance();
        unsafe {
            let mut handle: GLuint = std::mem::uninitialized();
            gl::GenTextures(1, &mut handle);
            gl::BindTexture(gl::TEXTURE_2D, handle);
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGBA as GLint,
                           1,
                           1,
                           0,
                           gl::RGBA as GLenum,
                           gl::UNSIGNED_BYTE,
                           std::ptr::null());

            #[cfg(target_os = "emscripten")]
            {
                run_js!{
                    CodeVisual.internal.load_texture(path, &handle);
                }
            }

            Ok(Texture { handle })
        }
    }
    pub fn get_handle(&self) -> GLuint {
        self.handle
    }
}