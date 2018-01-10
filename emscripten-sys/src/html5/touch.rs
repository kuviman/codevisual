use ::*;

pub const EMSCRIPTEN_EVENT_TOUCHSTART: EM_EVENT_TYPE = 22;
pub const EMSCRIPTEN_EVENT_TOUCHEND: EM_EVENT_TYPE = 23;
pub const EMSCRIPTEN_EVENT_TOUCHMOVE: EM_EVENT_TYPE = 24;
pub const EMSCRIPTEN_EVENT_TOUCHCANCEL: EM_EVENT_TYPE = 25;

#[repr(C)]
#[derive(Debug)]
pub struct EmscriptenTouchPoint {
    pub identifier: c_long,
    pub screenX: c_long,
    pub screenY: c_long,
    pub clientX: c_long,
    pub clientY: c_long,
    pub pageX: c_long,
    pub pageY: c_long,
    pub isChanged: EM_BOOL,
    pub onTarget: EM_BOOL,
    pub targetX: c_long,
    pub targetY: c_long,
    pub canvasX: c_long,
    pub canvasY: c_long,
}

#[repr(C)]
#[derive(Debug)]
pub struct EmscriptenTouchEvent {
    pub numTouches: c_int,
    pub ctrlKey: EM_BOOL,
    pub shiftKey: EM_BOOL,
    pub altKey: EM_BOOL,
    pub metaKey: EM_BOOL,
    pub touches: [EmscriptenTouchPoint; 32],
}

pub type em_touch_callback_func = Option<
    unsafe extern "C" fn(
        eventType: EM_EVENT_TYPE,
        touchEvent: *const EmscriptenTouchEvent,
        userData: *mut c_void,
    ) -> EM_BOOL,
>;

extern "C" {
    pub fn emscripten_set_touchstart_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EM_BOOL,
        callback: em_touch_callback_func,
    ) -> EMSCRIPTEN_RESULT;
    pub fn emscripten_set_touchend_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EM_BOOL,
        callback: em_touch_callback_func,
    ) -> EMSCRIPTEN_RESULT;
    pub fn emscripten_set_touchmove_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EM_BOOL,
        callback: em_touch_callback_func,
    ) -> EMSCRIPTEN_RESULT;
    pub fn emscripten_set_touchcancel_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EM_BOOL,
        callback: em_touch_callback_func,
    ) -> EMSCRIPTEN_RESULT;
}
