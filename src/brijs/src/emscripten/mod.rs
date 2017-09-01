use ::*;

mod browser;

pub use self::browser::*;

mod js;

pub use self::js::*;

pub fn wget<F: FnOnce(&str) + 'static>(url: &str, on_load: F) {
    let callback = Box::new(Box::new(on_load));
    unsafe {
        emscripten_async_wget_data(
            CString::new(url).unwrap().as_ptr(),
            Box::into_raw(callback) as *mut _,
            Some(on_load_wrapper::<F>),
            None,
        );
    }
    unsafe extern "C" fn on_load_wrapper<F: FnOnce(&str) + 'static>(
        callback: *mut c_void,
        data: *mut c_void,
        data_size: c_int,
    ) {
        let mut callback = Box::<Box<F>>::from_raw(callback as *mut _);
        let data = std::slice::from_raw_parts(data as *mut u8, data_size as usize);
        callback(std::str::from_utf8(data).unwrap());
    }
}
