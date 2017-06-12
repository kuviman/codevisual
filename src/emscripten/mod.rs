use std;

pub fn get_proc_address(name: &str) -> *const std::os::raw::c_void {
    unsafe {
        ::emscripten_sys::emscripten_GetProcAddress(std::ffi::CString::new(name)
                                      .expect("Could not convert name to C string")
                                      .as_ptr()) as *const _
    }
}

pub fn run_script(script: &str) {
    use std::ffi::CString;
    unsafe {
        ::emscripten_sys::emscripten_run_script(CString::new(script)
                                                    .expect("Could not convert script to C string",)
                                                    .as_ptr());
    }
}

pub fn get_canvas_size() -> (u32, u32) {
    use std::os::raw::c_int;
    unsafe {
        let mut width: c_int = std::mem::uninitialized();
        let mut height: c_int = std::mem::uninitialized();
        let mut is_fullscreen: c_int = std::mem::uninitialized();
        ::emscripten_sys::emscripten_get_canvas_size(&mut width, &mut height, &mut is_fullscreen);
        (width as u32, height as u32)
    }
}

pub fn create_gl_context() -> Result<(), ::Error> {
    unsafe {
        let mut attributes: ::emscripten_sys::EmscriptenWebGLContextAttributes =
            std::mem::uninitialized();
        ::emscripten_sys::emscripten_webgl_init_context_attributes(&mut attributes);
        let context = ::emscripten_sys::emscripten_webgl_create_context(std::ptr::null(),
                                                                        &attributes);
        if context <= 0 {
            return Err(::Error::from("Could not create WebGL context"));
        }
        ::emscripten_sys::emscripten_webgl_make_context_current(context);
    }
    Ok(())
}

pub fn set_main_loop<F: FnMut()>(callback: F) {
    use std::cell::RefCell;
    use std::ptr::null_mut;
    use std::os::raw::c_void;

    thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));
    MAIN_LOOP_CALLBACK.with(|log| { *log.borrow_mut() = &callback as *const _ as *mut c_void; });

    run_script("CodeVisual.ffi.before_main_loop()");
    unsafe {
        ::emscripten_sys::emscripten_set_main_loop(Some(wrapper::<F>), 0, 1);
    }

    unsafe extern "C" fn wrapper<F>()
        where F: FnMut()
    {
        MAIN_LOOP_CALLBACK.with(|z| {
                                    let closure = *z.borrow_mut() as *mut F;
                                    (*closure)();
                                    run_script("CodeVisual.stats.update()");
                                });
    }
}