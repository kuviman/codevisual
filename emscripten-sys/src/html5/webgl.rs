use ::*;

pub const EMSCRIPTEN_EVENT_WEBGLCONTEXTLOST: EM_EVENT_TYPE = 31;
pub const EMSCRIPTEN_EVENT_WEBGLCONTEXTRESTORED: EM_EVENT_TYPE = 32;

pub type EMSCRIPTEN_WEBGL_CONTEXT_HANDLE = c_int;

#[repr(C)]
#[derive(Debug)]
pub struct EmscriptenWebGLContextAttributes {
    pub alpha: EM_BOOL,
    pub depth: EM_BOOL,
    pub stencil: EM_BOOL,
    pub antialias: EM_BOOL,
    pub premultipliedAlpha: EM_BOOL,
    pub preserveDrawingBuffer: EM_BOOL,
    pub preferLowPowerToHighPerformance: EM_BOOL,
    pub failIfMajorPerformanceCaveat: EM_BOOL,
    pub majorVersion: c_int,
    pub minorVersion: c_int,
    pub enableExtensionsByDefault: EM_BOOL,
}

pub type em_webgl_context_callback = Option<
    unsafe extern "C" fn(eventType: EM_EVENT_TYPE, reserved: *const c_void, userData: *mut c_void)
        -> EM_BOOL,
>;

extern "C" {
    pub fn emscripten_set_webglcontextlost_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EM_BOOL,
        callback: em_webgl_context_callback,
    ) -> EMSCRIPTEN_RESULT;
    pub fn emscripten_set_webglcontextrestored_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EM_BOOL,
        callback: em_webgl_context_callback,
    ) -> EMSCRIPTEN_RESULT;
    pub fn emscripten_is_webgl_context_lost(target: *const c_char) -> EM_BOOL;
    pub fn emscripten_webgl_init_context_attributes(
        attributes: *mut EmscriptenWebGLContextAttributes,
    );
    pub fn emscripten_webgl_create_context(
        target: *const c_char,
        attributes: *const EmscriptenWebGLContextAttributes,
    ) -> EMSCRIPTEN_WEBGL_CONTEXT_HANDLE;
    pub fn emscripten_webgl_make_context_current(
        context: EMSCRIPTEN_WEBGL_CONTEXT_HANDLE,
    ) -> EMSCRIPTEN_RESULT;
    pub fn emscripten_webgl_get_current_context() -> EMSCRIPTEN_WEBGL_CONTEXT_HANDLE;
    pub fn emscripten_webgl_get_drawing_buffer_size(
        context: EMSCRIPTEN_WEBGL_CONTEXT_HANDLE,
        width: *mut c_int,
        height: *mut c_int,
    ) -> EMSCRIPTEN_RESULT;
    pub fn emscripten_webgl_destroy_context(
        context: EMSCRIPTEN_WEBGL_CONTEXT_HANDLE,
    ) -> EMSCRIPTEN_RESULT;
    pub fn emscripten_webgl_enable_extension(
        context: EMSCRIPTEN_WEBGL_CONTEXT_HANDLE,
        extension: *const c_char,
    ) -> EM_BOOL;
}
