#![deny(warnings)]

pub extern crate rand;

#[cfg(target_os = "emscripten")]
pub fn thread_rng() -> Box<rand::Rng> {
    extern "C" {
        fn emscripten_random() -> c_float;
    }
    struct EmscriptenRng;
    impl rand::Rng for EmscriptenRng {
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
pub use rand::thread_rng;

pub fn random<R: rand::Rand>() -> R {
    R::rand(&mut thread_rng())
}

pub extern crate num;

pub use num::{Float, Num, Integer, clamp};

mod color;

pub use self::color::*;

mod algebra;

pub use self::algebra::*;

mod range;

pub use self::range::*;

pub use std::rc::Rc;
pub use std::cell::{Cell, RefCell, Ref, RefMut};
pub use std::marker::PhantomData;
pub use std::error::Error;
pub use std::os::raw::{c_int, c_float, c_double, c_short, c_ushort, c_long, c_ulong, c_char,
                       c_void};
pub use std::ffi::CString;
pub use std::ops::{Deref, DerefMut, Range, RangeFrom, RangeTo, RangeFull};
pub use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
pub use std::ops::{Index, IndexMut};
pub use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
pub use std::fmt::{Debug, Display, Formatter};
pub use std::sync::{Arc, Mutex, RwLock};
pub use std::cmp::{min, max, Ord};

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
