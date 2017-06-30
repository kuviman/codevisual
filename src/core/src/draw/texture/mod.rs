use std;
use std::error::Error;
use gl::types::*;
use gl;

pub struct Texture {
    handle: GLuint,
}

#[derive(Debug)]
pub struct TextureError {
    description: String,
}

impl Error for TextureError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl std::fmt::Display for TextureError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Error::description(self))
    }
}

impl Texture {
    pub fn load(_: &::Application, path: &str) -> Result<Self, TextureError> {
        unsafe {
            let mut handle: GLuint = std::mem::uninitialized();
            gl::GenTextures(1, &mut handle);
            gl::BindTexture(gl::TEXTURE_2D, handle);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);

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
                let image = match ::image::open(path) {
                    Ok(image) => image.to_rgba(),
                    Err(e) => {
                        return Err(TextureError {
                                       description: String::from(Error::description(&e)),
                                   })
                    }
                };
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