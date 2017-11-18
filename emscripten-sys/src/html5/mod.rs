use ::*;

mod keys;
mod mouse;
mod wheel;
mod touch;
mod webgl;
mod css;

pub use self::keys::*;
pub use self::mouse::*;
pub use self::wheel::*;
pub use self::touch::*;
pub use self::webgl::*;
pub use self::css::*;

pub type EM_BOOL = c_int;

pub const EM_TRUE: EM_BOOL = 1;
pub const EM_FALSE: EM_BOOL = 0;

pub type EM_UTF8 = c_char;

pub type EMSCRIPTEN_RESULT = c_int;

pub const EMSCRIPTEN_RESULT_SUCCESS: EMSCRIPTEN_RESULT = 0;
pub const EMSCRIPTEN_RESULT_DEFERRED: EMSCRIPTEN_RESULT = 1;
pub const EMSCRIPTEN_RESULT_NOT_SUPPORTED: EMSCRIPTEN_RESULT = -1;
pub const EMSCRIPTEN_RESULT_FAILED_NOT_DEFERRED: EMSCRIPTEN_RESULT = -2;
pub const EMSCRIPTEN_RESULT_INVALID_TARGET: EMSCRIPTEN_RESULT = -3;
pub const EMSCRIPTEN_RESULT_UNKNOWN_TARGET: EMSCRIPTEN_RESULT = -4;
pub const EMSCRIPTEN_RESULT_INVALID_PARAM: EMSCRIPTEN_RESULT = -5;
pub const EMSCRIPTEN_RESULT_FAILED: EMSCRIPTEN_RESULT = -6;
pub const EMSCRIPTEN_RESULT_NO_DATA: EMSCRIPTEN_RESULT = -7;
pub const EMSCRIPTEN_RESULT_TIMED_OUT: EMSCRIPTEN_RESULT = -8;

pub type EM_EVENT_TYPE = c_int;