#[cfg(target_os = "emscripten")]
#[path="emscripten/mod.rs"]
mod platform;

pub use self::platform::*;