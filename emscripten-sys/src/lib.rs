#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod emscripten;
mod html5;

pub use emscripten::*;
pub use html5::*;

pub(crate) use std::os::raw::{c_char, c_double, c_float, c_int, c_long, c_uint, c_ulong, c_ushort,
                              c_void};

extern "C" {
    pub fn emscripten_GetProcAddress(name: *const c_char) -> *const c_void;
}
