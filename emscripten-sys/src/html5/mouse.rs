use ::*;

pub const EMSCRIPTEN_EVENT_CLICK: EM_EVENT_TYPE = 4;
pub const EMSCRIPTEN_EVENT_MOUSEDOWN: EM_EVENT_TYPE = 5;
pub const EMSCRIPTEN_EVENT_MOUSEUP: EM_EVENT_TYPE = 6;
pub const EMSCRIPTEN_EVENT_DBLCLICK: EM_EVENT_TYPE = 7;
pub const EMSCRIPTEN_EVENT_MOUSEMOVE: EM_EVENT_TYPE = 8;
pub const EMSCRIPTEN_EVENT_MOUSEENTER: EM_EVENT_TYPE = 33;
pub const EMSCRIPTEN_EVENT_MOUSELEAVE: EM_EVENT_TYPE = 34;

#[repr(C)]
#[derive(Debug)]
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

pub type em_mouse_callback_func = Option<unsafe extern "C" fn(
    eventType: EM_EVENT_TYPE,
    mouseEvent: *const EmscriptenMouseEvent,
    userData: *mut c_void) -> EM_BOOL>;

extern "C" {
    pub fn emscripten_set_click_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EM_BOOL,
        callback: em_mouse_callback_func) -> EMSCRIPTEN_RESULT;
    pub fn emscripten_set_mousedown_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EM_BOOL,
        callback: em_mouse_callback_func) -> EMSCRIPTEN_RESULT;
    pub fn emscripten_set_mouseup_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EM_BOOL,
        callback: em_mouse_callback_func) -> EMSCRIPTEN_RESULT;
    pub fn emscripten_set_dblclick_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EM_BOOL,
        callback: em_mouse_callback_func) -> EMSCRIPTEN_RESULT;
    pub fn emscripten_set_mousemove_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EM_BOOL,
        callback: em_mouse_callback_func) -> EMSCRIPTEN_RESULT;
    pub fn emscripten_set_mouseenter_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EM_BOOL,
        callback: em_mouse_callback_func) -> EMSCRIPTEN_RESULT;
    pub fn emscripten_set_mouseleave_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EM_BOOL,
        callback: em_mouse_callback_func) -> EMSCRIPTEN_RESULT;
    pub fn emscripten_get_mouse_status(mouseState: *mut EmscriptenMouseEvent) -> EMSCRIPTEN_RESULT;
}