#[cfg(not(target_os = "emscripten"))]
#[path = "impl_not_emscripten.rs"]
mod implementation;

#[cfg(target_os = "emscripten")]
#[path = "impl_emscripten.rs"]
mod implementation;

pub use self::implementation::*;