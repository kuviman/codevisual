#[macro_use]
extern crate codevisual;
#[macro_use]
extern crate ugli;

use codevisual::prelude::*;

struct Demo {
    font: Rc<codevisual::Font>,
    time: f64,
}

impl codevisual::Game for Demo {
    fn new(app: &Rc<codevisual::App>) -> Self {
        let context = app.ugli_context();

        Demo {
            font: codevisual::Font::default(context),
            time: 0.0,
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
            &format!("time passed: {} seconds", self.time as i32),
            vec2(0.0, 0.0),
            32.0,
            Color::WHITE,
        );
    }
    fn update(&mut self, delta_time: f64) {
        self.time += delta_time;
    }
}

fn main() {
    codevisual::run::<Demo>();
}
