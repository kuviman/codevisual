use ::*;

extern "C" {
    pub fn emscripten_set_main_loop(
        func: em_callback_func,
        fps: c_int,
        simulate_infinite_loop: c_int,
    );
    pub fn emscripten_set_main_loop_arg(
        func: em_arg_callback_func,
        arg: *mut c_void,
        fps: c_int,
        simulate_infinite_loop: c_int,
    );
    pub fn emscripten_push_main_loop_blocker(func: em_arg_callback_func, arg: *mut c_void);
    pub fn emscripten_push_uncounted_main_loop_blocker(
        func: em_arg_callback_func,
        arg: *mut c_void,
    );
    pub fn emscripten_pause_main_loop();
    pub fn emscripten_resume_main_loop();
    pub fn emscripten_cancel_main_loop();
    pub fn emscripten_set_main_loop_timing(mode: c_int, value: c_int) -> c_int;
    pub fn emscripten_get_main_loop_timing(mode: *mut c_int, value: *mut c_int);
    pub fn emscripten_set_main_loop_expected_blockers(num: c_int);
    pub fn emscripten_async_call(func: em_arg_callback_func, arg: *mut c_void, millis: c_int);
    pub fn emscripten_exit_with_live_runtime();
    pub fn emscripten_force_exit(status: c_int);
    pub fn emscripten_get_device_pixel_ratio() -> c_double;
    pub fn emscripten_hide_mouse();
    pub fn emscripten_set_canvas_size(width: c_int, height: c_int);
    pub fn emscripten_get_canvas_size(
        width: *mut c_int,
        height: *mut c_int,
        isFullscreen: *mut c_int,
    );
    pub fn emscripten_get_now() -> c_double;
    pub fn emscripten_random() -> c_float;
}
