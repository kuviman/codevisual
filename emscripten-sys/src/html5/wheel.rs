use ::*;

pub const EMSCRIPTEN_EVENT_WHEEL: EM_EVENT_TYPE = 9;

pub type EM_DELTA_MODE = c_ulong;

pub const DOM_DELTA_PIXEL: EM_DELTA_MODE = 0x00;
pub const DOM_DELTA_LINE: EM_DELTA_MODE = 0x01;
pub const DOM_DELTA_PAGE: EM_DELTA_MODE = 0x02;

#[repr(C)]
#[derive(Debug)]
pub struct EmscriptenWheelEvent {
    pub mouse: EmscriptenMouseEvent,
    pub deltaX: c_double,
    pub deltaY: c_double,
    pub deltaZ: c_double,
    pub deltaMode: EM_DELTA_MODE,
}

pub type em_wheel_callback_func = Option<unsafe extern "C" fn(
    eventType: EM_EVENT_TYPE,
    wheelEvent: *const EmscriptenWheelEvent,
    userData: *mut c_void) -> EM_BOOL>;

extern "C" {
    pub fn emscripten_set_wheel_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EM_BOOL,
        callback: em_wheel_callback_func) -> EMSCRIPTEN_RESULT;
}