use ::*;

pub struct Timer {
    #[cfg(target_os = "emscripten")] start_time: f64,
    #[cfg(not(target_os = "emscripten"))] start: std::time::Instant,
}

#[cfg(target_os = "emscripten")]
extern "C" {
    fn emscripten_get_now() -> c_double;
}

#[cfg(not(target_os = "emscripten"))]
fn to_secs(duration: std::time::Duration) -> f64 {
    duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1e9
}

impl Timer {
    pub fn new() -> Self {
        #[cfg(target_os = "emscripten")]
        return Self {
            start_time: unsafe { emscripten_get_now() } as f64 / 1000.0,
        };
        #[cfg(not(target_os = "emscripten"))]
        return Self {
            start: std::time::Instant::now(),
        };
    }
    pub fn elapsed(&self) -> f64 {
        #[cfg(target_os = "emscripten")]
        return unsafe { emscripten_get_now() } as f64 / 1000.0 - self.start_time;
        #[cfg(not(target_os = "emscripten"))]
        return to_secs(self.start.elapsed());
    }
    pub fn tick(&mut self) -> f64 {
        #[cfg(target_os = "emscripten")]
        return {
            let now = unsafe { emscripten_get_now() } as f64 / 1000.0;
            let delta = now - self.start_time;
            self.start_time = now;
            delta
        };
        #[cfg(not(target_os = "emscripten"))]
        return {
            let now = std::time::Instant::now();
            let delta = to_secs(now.duration_since(self.start));
            self.start = now;
            delta
        };
    }
}
