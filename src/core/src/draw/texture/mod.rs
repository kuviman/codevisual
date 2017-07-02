use std;
use std::error::Error;
use gl::types::*;
use gl;
use commons::*;

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

pub struct TextureResource {
    texture: Rc<Texture>,
    loaded: Rc<Cell<bool>>,
}

impl std::ops::Deref for TextureResource {
    type Target = Rc<Texture>;
    fn deref(&self) -> &Self::Target {
        assert!(self.loaded.get());
        &self.texture
    }
}

impl Texture {
    pub fn load(loader: &::ResourceLoader, path: &str) -> TextureResource {
        unsafe {
            let mut handle: GLuint = std::mem::uninitialized();
            gl::GenTextures(1, &mut handle);
            gl::BindTexture(gl::TEXTURE_2D, handle);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);

            let loaded = Rc::new(Cell::new(false));

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
                let loaded = loaded.clone();
                loader.resource_count.set(loader.resource_count.get() + 1);
                let loaded_resource_count = loader.loaded_resource_count.clone();
                run_js!{
                    CodeVisual.internal.load_texture(path, &handle, ::emscripten::Callback::new(move |_: ()| {
                        loaded.set(true);
                        loaded_resource_count.set(loaded_resource_count.get() + 1);
                    }));
                }
            }
            #[cfg(not(target_os = "emscripten"))]
            {
                let image = match ::image::open(path) {
                    Ok(image) => image.to_rgba(),
                    Err(e) => {
                        panic!(TextureError { description: String::from(Error::description(&e)) });
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
                loaded.set(true);
            }

            TextureResource {
                texture: Rc::new(Texture { handle }),
                loaded,
            }
        }
    }
    pub fn get_handle(&self) -> GLuint {
        self.handle
    }
}