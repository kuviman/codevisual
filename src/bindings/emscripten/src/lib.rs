extern crate emscripten_sys;
extern crate serde;
extern crate serde_json;

mod emscripten;
pub use emscripten::*;

mod html5;
pub use html5::*;

use emscripten_sys::*;
use std::os::raw::{c_double, c_char, c_ushort, c_int, c_long, c_ulong, c_void};
use std::ffi::CString;

#[allow(non_camel_case_types)]
type EM_BOOL = c_int;
const EM_TRUE: EM_BOOL = 1;

const CANVAS_SELECTOR: &[c_char] = b"#canvas\0";
const USE_CAPTURE: EM_BOOL = 1;

pub fn get_proc_address(name: &str) -> *const c_void {
    let name = CString::new(name).expect("Could not convert name to C string");
    unsafe { ::emscripten_sys::emscripten_GetProcAddress(name.as_ptr()) as *const _ }
}

fn into_canvas_pos(x: c_long, y: c_long) -> (f64, f64) {
    let (x, y) = (x as f64, y as f64);
    let (css_width, css_height) = unsafe {
        let mut css_width: c_double = std::mem::uninitialized();
        let mut css_height: c_double = std::mem::uninitialized();
        emscripten_get_element_css_size(std::ptr::null(), &mut css_width, &mut css_height);
        (css_width as f64, css_height as f64)
    };
    let (width, height) = get_canvas_size();
    let (width, height) = (width as f64, height as f64);
    (x * width / css_width, y * height / css_height)
}