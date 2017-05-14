use serde_json;
use gl;
use std;

pub mod ffi;

pub struct Platform {}

pub fn panic_hook(info: &std::panic::PanicInfo) {
    use std::string::ToString;
    let mut json_info = serde_json::Value::Object(serde_json::Map::new());
    if let Some(location) = info.location() {
        let mut json_location = serde_json::Value::Object(serde_json::Map::new());
        json_location["file"] = serde_json::Value::String(location.file().to_string());
        json_location["line"] = serde_json::Value::String(location.line().to_string());
        json_info["location"] = json_location;
    }
    if let Some(error) = info.payload().downcast_ref::<String>() {
        json_info["error"] = serde_json::Value::String(error.clone());
    } else if let Some(error) = info.payload().downcast_ref::<&str>() {
        json_info["error"] = serde_json::Value::String(error.to_string());
    } else {
        json_info["error"] = serde_json::Value::String(String::from("Something went wrong"));
    }
    ffi::call_js("CodeVisual.ffi.error", json_info);
}

pub fn init() -> Result<Platform, String> {
    ffi::eval_js(include_str!(concat!(env!("OUT_DIR"), "/codevisual-lib.js")));
    ffi::call_js("CodeVisual.ffi.init_css",
                 include_str!(concat!(env!("OUT_DIR"), "/codevisual-lib.css")));
    ffi::call_js("CodeVisual.ffi.init_html",
                 include_str!(concat!(env!("OUT_DIR"), "/codevisual-lib.html")));
    try!(ffi::create_gl_context());
    gl::load_with(ffi::get_proc_address);
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
                                        ffi::eval_js("CodeVisual.stats.update()");
                                    });
        }
    }
}