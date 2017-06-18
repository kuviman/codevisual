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
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

            #[cfg(target_os = "emscripten")]
            {
                gl::TexImage2D(gl::TEXTURE_2D,
                               0,
                               gl::RGBA as GLint,
                               1,
                               1,
                               0,
                               gl::RGBA as GLenum,
                               gl::UNSIGNED_BYTE,
                               std::ptr::null());
                run_js!{
                    CodeVisual.internal.load_texture(path, &handle);
                }
            }
            #[cfg(not(target_os = "emscripten"))]
            {
                let image = ::image::open(path).unwrap().to_rgba();
                gl::TexImage2D(gl::TEXTURE_2D,
                               0,
                               gl::RGBA as GLint,
                               image.width() as GLsizei,
                               image.height() as GLsizei,
                               0,
                               gl::RGBA as GLenum,
                               gl::UNSIGNED_BYTE,
                               image.into_raw().as_ptr() as *const _);
            }

            Ok(Texture { handle })
        }
    }
    pub fn get_handle(&self) -> GLuint {
        self.handle
    }
}