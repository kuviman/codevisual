pub extern crate codevisual;

use codevisual::common::*;
use codevisual::draw;

struct Test {
    current_time: f32,
}

impl Test {
    fn new() -> Self {
        Self { current_time: 0.0 }
    }
}

impl codevisual::Game for Test {
    fn update(&mut self, delta_time: f32) {
        self.current_time += delta_time;
    }
    fn render(&mut self) -> Vec<draw::Command> {
        vec![draw::Command::Clear { color: Color::rgb(self.current_time.fract(), 0.8, 1.0) }]
    }
}

fn main() {
    codevisual::run(Test::new());

    // Hack (WTF??)
    unsafe {
        codevisual::platform::ffi::emscripten_GetProcAddress(std::ffi::CString::new("abacaba")
                                                                 .unwrap()
                                                                 .into_raw() as
                                                             *const _);
    }
}