use std;
use std::os::raw::c_void;
use std::ffi::CString;

pub fn random() -> f64 {
    unsafe { ::emscripten_sys::emscripten_random() as f64 }
}

pub fn get_now() -> f64 {
    unsafe { ::emscripten_sys::emscripten_get_now() / 1000.0 as f64 }
}

pub fn get_proc_address(name: &str) -> *const std::os::raw::c_void {
    unsafe {
        ::emscripten_sys::emscripten_GetProcAddress(CString::new(name)
                                      .expect("Could not convert name to C string")
                                      .as_ptr()) as *const _
    }
}

pub fn run_script(script: &str) {
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
    let callback = Box::new(Box::new(callback));
    unsafe {
        ::emscripten_sys::emscripten_set_main_loop_arg(Some(wrapper::<F>),
                                                       Box::into_raw(callback) as *mut _,
                                                       0,
                                                       1);
    }
    unsafe extern "C" fn wrapper<F>(arg: *mut c_void)
        where F: FnMut()
    {
        let mut callback = Box::<Box<F>>::from_raw(arg as *mut _);
        callback();
        std::mem::forget(callback);
    }
}