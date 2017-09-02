use std::ops::{Range, RangeTo, RangeFrom, RangeFull};

/// Temporary re-implemenation of RangeArgument from stdlib while
/// waiting for it to become stable
pub trait RangeArgument<T> {
    fn start(&self) -> Option<&T>;
    fn end(&self) -> Option<&T>;
}
impl<T> RangeArgument<T> for RangeFull {
    fn start(&self) -> Option<&T> {
        None
    }
    fn end(&self) -> Option<&T> {
        None
    }
}
impl<T> RangeArgument<T> for RangeFrom<T> {
    fn start(&self) -> Option<&T> {
        Some(&self.start)
    }
    fn end(&self) -> Option<&T> {
        None
    }
}
impl<T> RangeArgument<T> for RangeTo<T> {
    fn start(&self) -> Option<&T> {
        None
    }
    fn end(&self) -> Option<&T> {
        Some(&self.end)
    }
}
impl<T> RangeArgument<T> for Range<T> {
    fn start(&self) -> Option<&T> {
        Some(&self.start)
    }
    fn end(&self) -> Option<&T> {
        Some(&self.end)
    }
}

pub struct CustomRange<T> {
    start: Option<T>,
    end: Option<T>,
}

impl<T> RangeArgument<T> for CustomRange<T> {
    fn start(&self) -> Option<&T> {
        self.start.as_ref()
    }
    fn end(&self) -> Option<&T> {
        self.end.as_ref()
    }
}

impl<T: Copy> CustomRange<T> {
    pub fn from<R: RangeArgument<T>>(range: &R) -> Self {
        Self {
            start: range.start().map(|v| *v),
            end: range.end().map(|v| *v),
        }
    }
}

pub fn get_slice<T, R: RangeArgument<usize>>(array: &[T], range: R) -> &[T] {
    match range.start() {
        Some(&start) => {
            match range.end() {
                Some(&end) => &array[start..end],
                None => &array[start..],
            }
        }
        None => {
            match range.end() {
                Some(&end) => &array[..end],
                None => &array[..],
            }
        }
    }
}
