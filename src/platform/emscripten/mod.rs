pub mod ffi;

pub struct Platform {}

pub fn init() -> Result<Platform, String> {
    ffi::eval_js(include_str!(concat!(env!("OUT_DIR"), "/codevisual-lib.js")));
    ffi::call_js("CodeVisual.ffi.init_css",
                 include_str!(concat!(env!("OUT_DIR"), "/codevisual-lib.css")));
    ffi::call_js("CodeVisual.ffi.init_html",
                 include_str!(concat!(env!("OUT_DIR"), "/codevisual-lib.html")));
    Ok(Platform {})
}

impl Platform {
    pub fn run_main_loop<F: FnMut() -> bool>(&self, callback: F) {
        use std::cell::RefCell;
        use std::ptr::null_mut;
        use std::os::raw::c_void;
        thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));
        MAIN_LOOP_CALLBACK.with(|log| {
                                    *log.borrow_mut() = &callback as *const _ as *mut c_void;
                                });
        ffi::eval_js("CodeVisual.ffi.before_main_loop()");
        unsafe {
            ffi::emscripten_set_main_loop(wrapper::<F>, 0, 1);
        }
        unsafe extern "C" fn wrapper<F>()
            where F: FnMut() -> bool
        {
            MAIN_LOOP_CALLBACK.with(|z| {
                                        let closure = *z.borrow_mut() as *mut F;
                                        (*closure)();
                                    });
        }
    }
}