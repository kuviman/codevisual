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

            #[cfg(target_os = "emscripten")]
            {
                use serde_json;
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
                let mut args = serde_json::Value::Object(serde_json::Map::new());
                args["path"] = serde_json::Value::String(String::from(path));
                args["texture_handle"] =
                    serde_json::Value::Number(serde_json::Number::from_f64(handle as f64).unwrap());
                ::emscripten::run_script(&format!("CodeVisual.internal.load_texture({})", args));
            }

            Ok(Texture { handle })
        }
    }
    pub fn get_handle(&self) -> GLuint {
        self.handle
    }
}