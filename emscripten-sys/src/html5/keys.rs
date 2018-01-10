use ::*;

pub const EMSCRIPTEN_EVENT_KEYPRESS: EM_EVENT_TYPE = 1;
pub const EMSCRIPTEN_EVENT_KEYDOWN: EM_EVENT_TYPE = 2;
pub const EMSCRIPTEN_EVENT_KEYUP: EM_EVENT_TYPE = 3;

pub type DOM_KEY_LOCATION = c_ulong;

pub const DOM_KEY_LOCATION_STANDARD: DOM_KEY_LOCATION = 0x00;
pub const DOM_KEY_LOCATION_LEFT: DOM_KEY_LOCATION = 0x01;
pub const DOM_KEY_LOCATION_RIGHT: DOM_KEY_LOCATION = 0x02;
pub const DOM_KEY_LOCATION_NUMPAD: DOM_KEY_LOCATION = 0x03;

#[repr(C)]
#[derive(Debug)]
pub struct EmscriptenKeyboardEvent {
    pub key: [EM_UTF8; 32],
    pub code: [EM_UTF8; 32],
    pub location: DOM_KEY_LOCATION,
    pub ctrlKey: EM_BOOL,
    pub shiftKey: EM_BOOL,
    pub altKey: EM_BOOL,
    pub metaKey: EM_BOOL,
    pub repeat: EM_BOOL,
    pub locale: [EM_UTF8; 32],
    pub charValue: [EM_UTF8; 32],
    pub charCode: c_ulong,
    pub keyCode: c_ulong,
    pub which: c_ulong,
}

pub type em_key_callback_func = Option<
    unsafe extern "C" fn(
        eventType: EM_EVENT_TYPE,
        keyEvent: *const EmscriptenKeyboardEvent,
        userData: *mut c_void,
    ) -> EM_BOOL,
>;

extern "C" {
    pub fn emscripten_set_keypress_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EM_BOOL,
        callback: em_key_callback_func,
    ) -> EMSCRIPTEN_RESULT;
    pub fn emscripten_set_keydown_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EM_BOOL,
        callback: em_key_callback_func,
    ) -> EMSCRIPTEN_RESULT;
    pub fn emscripten_set_keyup_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EM_BOOL,
        callback: em_key_callback_func,
    ) -> EMSCRIPTEN_RESULT;
}
