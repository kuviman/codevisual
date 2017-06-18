#[cfg(not(target_os = "emscripten"))]
extern crate rand;

use std;

pub trait Random {
    fn get_random() -> Self;
}

impl Random for f64 {
    #[cfg(target_os = "emscripten")]
    fn get_random() -> f64 {
        ::emscripten::random()
    }
    #[cfg(not(target_os = "emscripten"))]
    fn get_random() -> f64 {
        self::rand::random()
    }
}

impl Random for f32 {
    fn get_random() -> f32 {
        <f64 as Random>::get_random() as f32
    }
}

pub fn random<R: Random>() -> R {
    R::get_random()
}

pub fn random_range(range: std::ops::Range<usize>) -> usize {
    range.start + (random::<f64>() * (range.end - range.start) as f64) as usize
}