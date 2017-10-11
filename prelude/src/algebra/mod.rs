use ::*;

mod vec;

pub use self::vec::*;

mod mat;

pub use self::mat::*;

mod rect;

pub use self::rect::*;

pub fn min_max<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    min_max(a, b).0
}

pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    min_max(a, b).1
}