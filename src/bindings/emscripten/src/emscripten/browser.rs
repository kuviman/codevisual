use ::*;

pub fn get_canvas_size() -> (u32, u32) {
    unsafe {
        let mut width: c_int = std::mem::uninitialized();
        let mut height: c_int = std::mem::uninitialized();
        let mut is_fullscreen: c_int = std::mem::uninitialized();
        emscripten_get_canvas_size(&mut width, &mut height, &mut is_fullscreen);
        (width as u32, height as u32)
    }
}

pub fn set_main_loop<F: FnMut()>(callback: F) {
    let callback = Box::new(Box::new(callback));
    unsafe {
        emscripten_set_main_loop_arg(Some(wrapper::<F>), Box::into_raw(callback) as *mut _, 0, 1);

        // TODO: this is a hack. Emscripten (or rust?) optimizes emscripten_GetProcAddress out without this.
        emscripten_GetProcAddress(std::ptr::null());
    }
    unsafe extern "C" fn wrapper<F>(arg: *mut c_void)
        where F: FnMut()
    {
        let mut callback = Box::<Box<F>>::from_raw(arg as *mut _);
        callback();
        std::mem::forget(callback);
    }
}

pub fn random() -> f64 {
    unsafe { ::emscripten_sys::emscripten_random() as f64 }
}

pub fn get_now() -> f64 {
    unsafe { ::emscripten_sys::emscripten_get_now() as f64 / 1000.0 }
}