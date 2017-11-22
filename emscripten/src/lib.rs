#![deny(warnings)]

extern crate emscripten_sys;
extern crate prelude;

pub(crate) use emscripten_sys::*;
pub(crate) use prelude::*;

mod emscripten;
mod html5;

pub use emscripten::*;
pub use html5::*;

pub fn get_proc_address(name: &str) -> *const c_void {
    let name = CString::new(name).unwrap();
    unsafe {
        emscripten_GetProcAddress(name.as_ptr())
    }
}