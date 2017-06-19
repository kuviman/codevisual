use ::*;

pub fn create_gl_context() -> Result<(), String> {
    unsafe {
        let mut attributes: EmscriptenWebGLContextAttributes = std::mem::uninitialized();
        emscripten_webgl_init_context_attributes(&mut attributes);
        let context = emscripten_webgl_create_context(std::ptr::null(), &attributes);
        if context <= 0 {
            return Err(String::from("Could not create WebGL context"));
        }
        emscripten_webgl_make_context_current(context);
    }
    Ok(())
}