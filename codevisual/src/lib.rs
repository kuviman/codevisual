#![deny(warnings)]

#[cfg(target_os = "emscripten")]
extern crate emscripten;
#[macro_use]
extern crate failure;
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
extern crate color;
extern crate geom;
extern crate prelude;
extern crate timer;

pub(crate) use prelude::*;
pub(crate) use color::*;
pub(crate) use geom::*;
pub(crate) use timer::*;
pub(crate) use failure::Error;

mod app;
mod game;
mod window;
mod font;
mod shader_lib;

pub use self::app::*;
pub use self::game::*;
pub use self::window::*;
pub use self::font::*;
pub use self::shader_lib::*;
