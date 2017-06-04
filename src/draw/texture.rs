use std;
use gl::types::*;
use gl;

pub struct Texture {
    handle: GLuint,
}

impl Texture {
    pub fn load(path: &str) -> Result<Self, ::Error> {
        unsafe {
            let app = ::Application::get_instance();
            let mut handle: GLuint = std::mem::uninitialized();
            gl::GenTextures(1, &mut handle);
            app.platform.load_texture(path, handle)?;
            Ok(Texture { handle })
        }
    }
    pub fn get_handle(&self) -> GLuint {
        self.handle
    }
}