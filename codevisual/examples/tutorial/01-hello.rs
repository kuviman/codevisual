extern crate codevisual;

use codevisual::prelude::*;
use codevisual::ugli;

struct Tutorial {
    font: Rc<codevisual::Font>,
}

impl codevisual::Game for Tutorial {
    type Resources = ();
    fn new(app: &Rc<codevisual::Application>, _: ()) -> Self {
        Self { font: codevisual::Font::default(app.ugli_context()) }
    }
    fn get_title() -> String {
        String::from("CodeVisual Tutorial 01 - Hello, world!")
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(
            framebuffer,
            Some(Color::rgb(0.8, 0.8, 1.0)),
            None,
        );
        let framebuffer_size = framebuffer.get_size();
        let center = vec2(framebuffer_size.x as f32, framebuffer_size.y as f32) / 2.0;
        const HELLO: &str = "Hello, World! :)";
        const TEXT_SIZE: f32 = 64.0;
        let pos = center - vec2(self.font.measure(HELLO, TEXT_SIZE).unwrap().width() / 2.0, 0.0);
        self.font.draw(framebuffer, HELLO, pos, TEXT_SIZE, Color::BLACK);
    }
}

fn main() {
    codevisual::run::<Tutorial>();
}
