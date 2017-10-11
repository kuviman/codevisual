#![deny(warnings)]

#[doc(hidden)]
pub extern crate rand;
#[doc(hidden)]
pub extern crate num;

#[doc(no_inline)]
pub use std::rc::Rc;
#[doc(no_inline)]
pub use std::cell::{Cell, RefCell, Ref, RefMut};
#[doc(no_inline)]
pub use std::marker::PhantomData;
#[doc(no_inline)]
pub use std::error::Error;
#[doc(no_inline)]
pub use std::os::raw::{c_int, c_float, c_double, c_short, c_ushort, c_long, c_ulong, c_char,
                       c_void};
#[doc(no_inline)]
pub use std::ffi::CString;
#[doc(no_inline)]
pub use std::ops::{Deref, DerefMut, Range, RangeFrom, RangeTo, RangeFull};
#[doc(no_inline)]
pub use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
#[doc(no_inline)]
pub use std::ops::{Index, IndexMut};
#[doc(no_inline)]
pub use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
#[doc(no_inline)]
pub use std::fmt::{Debug, Display, Formatter};
#[doc(no_inline)]
pub use std::sync::{Arc, Mutex, RwLock};
#[doc(no_inline)]
pub use std::cmp::{Eq, PartialEq, Ord, PartialOrd};
#[doc(no_inline)]
pub use std::mem;
#[doc(no_inline)]
pub use std::thread;

#[doc(no_inline)]
pub use num::{Float, Num, Integer, clamp};

mod color;
mod algebra;
mod range;
mod timer;
mod atomic;

pub use color::*;
pub use algebra::*;
pub use range::*;
pub use timer::*;
pub use atomic::*;

pub use rand::Rng;

#[cfg(target_os = "emscripten")]
pub fn thread_rng() -> Box<Rng> {
    extern "C" {
        fn emscripten_random() -> c_float;
    }
    struct EmscriptenRng;
    impl Rng for EmscriptenRng {
        fn next_u32(&mut self) -> u32 {
            unsafe { (emscripten_random() as f64 * std::u32::MAX as f64) as u32 }
        }
        fn next_f32(&mut self) -> f32 {
            unsafe { emscripten_random() as f32 }
        }
        fn next_f64(&mut self) -> f64 {
            unsafe { emscripten_random() as f64 }
        }
    }
    Box::new(EmscriptenRng)
}

#[cfg(not(target_os = "emscripten"))]
#[doc(no_inline)]
pub use rand::thread_rng;

pub fn random<R: rand::Rand>() -> R {
    R::rand(&mut thread_rng())
}

#[macro_export]
macro_rules! display_error_description {
    ($name: ident) => {
        impl ::std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "{}", ::std::error::Error::description(self))
            }
        }
    }
}
