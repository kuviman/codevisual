use ::*;

pub type em_async_wget_onload_func = Option<unsafe extern "C" fn(*mut c_void, *mut c_void, c_int)>;
pub type em_async_wget2_onload_func = Option<unsafe extern "C" fn(*mut c_void, *const c_char)>;
pub type em_async_wget2_onstatus_func = Option<unsafe extern "C" fn(*mut c_void, c_int)>;
pub type em_async_wget2_data_onload_func =
    Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_uint)>;
pub type em_async_wget2_data_onerror_func =
    Option<unsafe extern "C" fn(*mut c_void, c_int, *const c_char)>;
pub type em_async_wget2_data_onprogress_func =
    Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>;
pub type em_run_preload_plugins_data_onload_func =
    Option<unsafe extern "C" fn(*mut c_void, *const c_char)>;

extern "C" {
    pub fn emscripten_wget(url: *const c_char, file: *const c_char);
    pub fn emscripten_async_wget(
        url: *const c_char,
        file: *const c_char,
        onload: em_str_callback_func,
        onerror: em_str_callback_func,
    );
    pub fn emscripten_async_wget_data(
        url: *const c_char,
        arg: *mut c_void,
        onload: em_async_wget_onload_func,
        onerror: em_arg_callback_func,
    );
    pub fn emscripten_async_wget2(
        url: *const c_char,
        file: *const c_char,
        requesttype: *const c_char,
        param: *const c_char,
        arg: *mut c_void,
        onload: em_async_wget2_onload_func,
        onerror: em_async_wget2_onstatus_func,
        onprogress: em_async_wget2_onstatus_func,
    ) -> c_int;
    pub fn emscripten_async_wget2_data(
        url: *const c_char,
        requesttype: *const c_char,
        param: *const c_char,
        arg: *mut c_void,
        free: c_int,
        onload: em_async_wget2_data_onload_func,
        onerror: em_async_wget2_data_onerror_func,
        onprogress: em_async_wget2_data_onprogress_func,
    ) -> c_int;
    pub fn emscripten_async_wget2_abort(handle: c_int);
    pub fn emscripten_run_preload_plugins_data(
        data: *mut c_char,
        size: c_int,
        suffix: *const c_char,
        arg: *mut c_void,
        onload: em_run_preload_plugins_data_onload_func,
        onerror: em_arg_callback_func,
    );
}
