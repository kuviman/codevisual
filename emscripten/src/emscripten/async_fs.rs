use ::*;

pub fn async_wget_data<F>(url: &str, callback: F)
where
    F: FnOnce(Result<&[u8], ()>) + 'static,
{
    let url = CString::new(url).unwrap();
    unsafe {
        emscripten_async_wget_data(
            url.as_ptr(),
            Box::into_raw(Box::new(callback)) as *mut _,
            Some(on_load::<F>),
            Some(on_error::<F>),
        );
    }
    unsafe extern "C" fn on_load<F>(callback: *mut c_void, data: *mut c_void, data_size: c_int)
    where
        F: FnOnce(Result<&[u8], ()>) + 'static,
    {
        let callback = Box::<F>::from_raw(callback as *mut _);
        let data = std::slice::from_raw_parts(data as *mut u8, data_size as usize);
        callback(Ok(data));
    }
    unsafe extern "C" fn on_error<F>(callback: *mut c_void)
    where
        F: FnOnce(Result<&[u8], ()>) + 'static,
    {
        let callback = Box::<F>::from_raw(callback as *mut _);
        callback(Err(()));
    }
}
