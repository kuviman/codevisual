use std;
use std::os::raw::{c_int, c_char, c_void, c_double, c_long, c_ushort};
use std::ffi::CString;
use serde_json;

pub fn eval_js(code: &str) -> i32 {
    let code = CString::new(code).unwrap();
    unsafe { emscripten_run_script_int(code.as_ptr()) }
}

pub fn call_js<T: Into<serde_json::Value>>(fun: &str, json: T) -> i32 {
    eval_js(format!("{0}({1})", fun, json.into().to_string()).as_str())
}

pub fn get_proc_address(name: &str) -> *const c_void {
    let name = CString::new(name).unwrap();
    unsafe { emscripten_GetProcAddress(name.into_raw() as *const _) as *const _ }
}

pub fn create_gl_context() -> Result<(), ::Error> {
    unsafe {
        let mut attributes: EmscriptenWebGLContextAttributes = std::mem::uninitialized();
        emscripten_webgl_init_context_attributes(&mut attributes);
        let context = emscripten_webgl_create_context(std::ptr::null(), &attributes);
        if context <= 0 {
            return Err(::Error::from("Could not create WebGL context"));
        }
        emscripten_webgl_make_context_current(context);
    }
    Ok(())
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct EmscriptenWebGLContextAttributes {
    pub alpha: c_int,
    pub depth: c_int,
    pub stencil: c_int,
    pub antialias: c_int,
    pub premultipliedAlpha: c_int,
    pub preserveDrawingBuffer: c_int,
    pub preferLowPowerToHighPerformance: c_int,
    pub failIfMajorPerformanceCaveat: c_int,
    pub majorVersion: c_int,
    pub minorVersion: c_int,
    pub enableExtensionsByDefault: c_int,
}

#[allow(non_camel_case_types)]
pub type EM_BOOL = c_int;

#[allow(non_camel_case_types)]
pub type EMSCRIPTEN_RESULT = c_int;

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EmscriptenMouseEvent {
    pub timestamp: c_double,
    pub screenX: c_long,
    pub screenY: c_long,
    pub clientX: c_long,
    pub clientY: c_long,
    pub ctrlKey: EM_BOOL,
    pub shiftKey: EM_BOOL,
    pub altKey: EM_BOOL,
    pub metaKey: EM_BOOL,
    pub button: c_ushort,
    pub buttons: c_ushort,
    pub movementX: c_long,
    pub movementY: c_long,
    pub targetX: c_long,
    pub targetY: c_long,
    pub canvasX: c_long,
    pub canvasY: c_long,
    pub padding: c_long,
}

#[allow(non_camel_case_types)]
pub type em_mouse_callback_func = extern "C" fn(eventType: c_int,
                                                mouseEvent: *const EmscriptenMouseEvent,
                                                userData: *mut c_void)
                                                -> EM_BOOL;

extern "C" {
    pub fn emscripten_run_script_int(s: *const c_char) -> c_int;
    pub fn emscripten_pause_main_loop();
    pub fn emscripten_cancel_main_loop();
    pub fn emscripten_set_main_loop(m: unsafe extern "C" fn(), fps: c_int, infinite: c_int);
    pub fn emscripten_GetProcAddress(name: *const c_char) -> *const c_void;
    pub fn emscripten_webgl_init_context_attributes(attributes: *mut EmscriptenWebGLContextAttributes);
    pub fn emscripten_webgl_create_context(target: *const c_char,
                                           attributes: *const EmscriptenWebGLContextAttributes)
                                           -> c_int;
    pub fn emscripten_webgl_make_context_current(context: c_int) -> c_int;
    pub fn emscripten_get_canvas_size(width: *mut c_int,
                                      height: *mut c_int,
                                      is_fullscreen: *mut c_int);

    pub fn emscripten_set_mousedown_callback(target: *const c_char,
                                             userData: *mut c_void,
                                             useCapture: EM_BOOL,
                                             callback: em_mouse_callback_func)
                                             -> EMSCRIPTEN_RESULT;
    pub fn emscripten_set_mouseup_callback(target: *const c_char,
                                           userData: *mut c_void,
                                           useCapture: EM_BOOL,
                                           callback: em_mouse_callback_func)
                                           -> EMSCRIPTEN_RESULT;
}