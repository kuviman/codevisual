use ::*;

pub type em_callback_func = Option<unsafe extern "C" fn()>;
pub type em_arg_callback_func = Option<unsafe extern "C" fn(*mut c_void)>;
pub type em_str_callback_func = Option<unsafe extern "C" fn(*const c_char)>;

extern "C" {
    pub fn emscripten_run_script(script: *const c_char);
    pub fn emscripten_run_script_int(script: *const c_char) -> c_int;
    pub fn emscripten_run_script_string(script: *const c_char) -> *mut c_char;
    pub fn emscripten_async_run_script(script: *const c_char, millis: c_int);
    pub fn emscripten_async_load_script(
        script: *const c_char,
        onload: em_callback_func,
        onerror: em_callback_func);
}