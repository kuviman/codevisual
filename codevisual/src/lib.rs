#![deny(warnings)]

extern crate color;
#[cfg(target_os = "emscripten")]
extern crate emscripten;
#[macro_use]
extern crate failure;
extern crate geom;
#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
extern crate glutin;
#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
extern crate image;
#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
extern crate num_cpus;
extern crate prelude as external_prelude;
#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
extern crate rodio;
extern crate rusttype;
extern crate serde;
#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
#[macro_use]
extern crate stdweb;
#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
extern crate threadpool;
extern crate timer;
#[macro_use]
extern crate ugli;

pub(crate) use failure::Error;

pub mod prelude {
    pub use external_prelude::*;
    pub use geom::*;
    pub use color::*;
    pub use timer::*;
}

pub(crate) use prelude::*;

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
