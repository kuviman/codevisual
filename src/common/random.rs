pub trait Random {
    fn get_random() -> Self;
}

impl Random for f64 {
    fn get_random() -> f64 {
        #[cfg(target_os = "emscripten")]
        ::emscripten::random()
    }
}

impl Random for f32 {
    fn get_random() -> f32 {
        <f64 as Random>::get_random() as f32
    }
}

impl Random for usize {
    fn get_random() -> usize {
        (<f64 as Random>::get_random() * usize::max_value() as f64) as usize
    }
}

pub fn random<R: Random>() -> R {
    R::get_random()
}