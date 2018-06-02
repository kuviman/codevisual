#[macro_use]
extern crate codevisual;
extern crate codevisual_debug_overlay;
#[macro_use]
extern crate ugli;

use codevisual::prelude::*;

struct Demo {
    context: Rc<codevisual::Context>,
}

impl Demo {
    fn new(context: &Rc<codevisual::Context>) -> Self {
        Demo {
            context: context.clone(),
        }
    }
}

impl codevisual::App for Demo {
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
    let context = Rc::new(codevisual::Context::new("CodeVisual Demo"));
    let app = Demo::new(&context);
    let app = codevisual_debug_overlay::App::new(&context, app);
    codevisual::run(context, app);
}
