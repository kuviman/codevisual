#[cfg(target_os = "emscripten")]
extern crate emscripten;

mod color;
pub use self::color::*;

mod random;
pub use self::random::*;

mod algebra;
pub use self::algebra::*;

mod range;
pub use self::range::*;