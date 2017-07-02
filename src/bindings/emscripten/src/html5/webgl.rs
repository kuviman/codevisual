use ::*;

#[derive(Debug)]
pub struct GLContextCreationError;

impl std::error::Error for GLContextCreationError {
    fn description(&self) -> &str {
        "WebGL context could not be created"
    }
}

impl std::fmt::Display for GLContextCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", std::error::Error::description(self))
    }
}

pub fn create_gl_context() -> Result<(), GLContextCreationError> {
    unsafe {
        let mut attributes: EmscriptenWebGLContextAttributes = std::mem::uninitialized();
        emscripten_webgl_init_context_attributes(&mut attributes);
        let context = emscripten_webgl_create_context(std::ptr::null(), &attributes);
        if context <= 0 {
            return Err(GLContextCreationError);
        }
        emscripten_webgl_make_context_current(context);
    }
    Ok(())
}