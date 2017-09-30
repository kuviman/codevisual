extern crate codevisual;

use codevisual::prelude::*;
use codevisual::ugli;

struct Tutorial;

impl codevisual::Game for Tutorial {
    type Resources = ();
    fn new(_: &Rc<codevisual::Application>, _: ()) -> Self {
        Self {}
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
    }
}

fn main() {
    codevisual::run::<Tutorial>();
}
