#[macro_use]
extern crate codevisual;
extern crate codevisual_debug_overlay;
#[macro_use]
extern crate ugli;

use codevisual::prelude::*;

struct Demo {
    context: Rc<codevisual::Context>,
}

impl codevisual::App for Demo {
    fn new(context: &Rc<codevisual::Context>) -> Self {
        Demo {
            context: context.clone(),
        }
    }
    fn title() -> String {
        String::from("CodeVisual Demo")
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
        let center = vec2(framebuffer.get_size().x as _, framebuffer.get_size().y as _) / 2.0;
        self.context.default_font().draw_aligned(
            framebuffer,
            "CodeVisual Demo",
            center,
            0.5,
            32.0,
            Color::WHITE,
        );
    }
}

fn main() {
    codevisual_debug_overlay::run::<Demo>();
}
