#![deny(warnings)]

pub extern crate num;
pub extern crate owning_ref;
pub extern crate rand;

pub use std::rc::Rc;
pub use std::cell::{Cell, Ref, RefCell, RefMut};
pub use std::marker::PhantomData;
pub use std::os::raw::{c_char, c_double, c_float, c_int, c_long, c_short, c_ulong, c_ushort,
                       c_void};
pub use std::ffi::{CStr, CString};
pub use std::borrow::Cow;
pub use std::ops::{Deref, DerefMut, Range, RangeFrom, RangeFull, RangeTo};
pub use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
pub use std::ops::{Index, IndexMut};
pub use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
pub use std::fmt::{Debug, Display, Formatter};
pub use std::sync::{Arc, Mutex};
pub use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
pub use std::mem;
pub use std::thread;

pub use num::{clamp, Float, Integer, Num};

pub use owning_ref::{OwningHandle, OwningRef, OwningRefMut};

mod color;
mod algebra;
mod range;
mod timer;
mod atomic;
mod stable_fn;

pub use color::*;
pub use algebra::*;
pub use range::*;
pub use timer::*;
pub use atomic::*;
pub use stable_fn::*;

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
pub use rand::thread_rng;

pub fn random<R: rand::Rand>() -> R {
    R::rand(&mut thread_rng())
}

pub fn default<T: Default>() -> T {
    T::default()
}
