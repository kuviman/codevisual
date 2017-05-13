use std::os::raw::{c_int, c_char};
use std::ffi::CString;
use serde_json;

pub fn eval_js(code: &str) -> i32 {
    let code = CString::new(code).unwrap();
    unsafe { emscripten_run_script_int(code.as_ptr()) }
}

pub fn call_js<T: Into<serde_json::Value>>(fun: &str, json: T) -> i32 {
    eval_js(format!("{0}({1})", fun, json.into().to_string()).as_str())
}

extern "C" {
    pub fn emscripten_run_script_int(s: *const c_char) -> c_int;
    pub fn emscripten_pause_main_loop();
    pub fn emscripten_cancel_main_loop();
    pub fn emscripten_set_main_loop(m: unsafe extern "C" fn(), fps: c_int, infinite: c_int);
}