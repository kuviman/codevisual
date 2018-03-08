#[macro_use]
extern crate codevisual;
#[macro_use]
extern crate ugli;

use codevisual::prelude::*;

struct Demo {
    font: Rc<codevisual::Font>,
    time: f64,
    frames: usize,
    fps: usize,
}

impl codevisual::Game for Demo {
    fn new(app: &Rc<codevisual::App>) -> Self {
        Demo {
            font: codevisual::Font::default(app),
            time: 0.0,
            frames: 0,
            fps: 0,
        }
    }
    fn title() -> String {
        String::from("CodeVisual Demo")
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
        let center = vec2(framebuffer.get_size().x as _, framebuffer.get_size().y as _) / 2.0;
        self.font.draw_aligned(
            framebuffer,
            "CodeVisual Demo",
            center,
            0.5,
            32.0,
            Color::WHITE,
        );
        self.font.draw(
            framebuffer,
            &format!("FPS: {}", self.fps),
            vec2(10.0, 10.0),
            16.0,
            Color::WHITE,
        );
    }
    fn update(&mut self, delta_time: f64) {
        self.time += delta_time;
        self.frames += 1;
        if self.time > 1.0 {
            self.fps = (self.frames as f64 / self.time) as _;
            self.time = 0.0;
            self.frames = 0;
        }
    }
}

fn main() {
    codevisual::run::<Demo>();
}
