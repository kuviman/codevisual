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

pub use std::rc::Rc;
pub use std::cell::{Cell, RefCell, Ref};
pub use std::marker::PhantomData;