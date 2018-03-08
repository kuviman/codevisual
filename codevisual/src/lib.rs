#![deny(warnings)]

#[cfg(target_os = "emscripten")]
extern crate emscripten;
#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
extern crate glutin;
#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
extern crate image;
#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
extern crate num_cpus;
#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
extern crate rodio;
extern crate rusttype;
extern crate serde;
#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
#[macro_use]
extern crate stdweb;
#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
extern crate threadpool;
#[macro_use]
extern crate ugli;
pub extern crate prelude;

pub(crate) use prelude::*;

mod app;
mod window;
mod font;
mod shader_lib;

pub use self::app::*;
pub use self::window::*;
pub use self::font::*;
pub use self::shader_lib::*;
