#[cfg(target_os = "emscripten")]
#[path="emscripten/mod.rs"]
pub mod implementation;

pub use self::implementation::*;