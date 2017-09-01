extern crate emscripten_sys;
extern crate serde;
extern crate serde_json;
extern crate ugli;
extern crate vpl;

pub ( crate ) use vpl::*;

mod emscripten;

pub use emscripten::*;

mod html5;

pub use html5::*;

use emscripten_sys::*;

#[allow(non_camel_case_types)]
type EM_BOOL = c_int;

const EM_TRUE: EM_BOOL = 1;

const EMSCRIPTEN_FETCH_LOAD_TO_MEMORY: u32 = 1;
const EMSCRIPTEN_FETCH_STREAM_DATA: u32 = 2;

const CANVAS_SELECTOR: &[c_char] = b"#canvas\0";
const USE_CAPTURE: EM_BOOL = 1;

pub fn get_proc_address(name: &str) -> *const c_void {
    let name = CString::new(name).expect("Could not convert name to C string");
    unsafe { ::emscripten_sys::emscripten_GetProcAddress(name.as_ptr()) as *const _ }
}
