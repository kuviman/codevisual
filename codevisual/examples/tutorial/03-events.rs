extern crate codevisual;
extern crate ugli;

use codevisual::prelude::*;

struct Tutorial;

impl codevisual::Game for Tutorial {
    type Resources = ();
    fn new(_: &Rc<codevisual::App>, _: ()) -> Self {
        Self {}
    }
    fn get_title() -> String {
        String::from("CodeVisual Tutorial 03 - Events")
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(
            framebuffer,
            Some(Color::rgb(0.2, 1.0, 0.2)),
            None,
        );
    }
    fn handle_event(&mut self, event: codevisual::Event) {
        println!("{:?}", event);
    }
}

fn main() {
    codevisual::run::<Tutorial>();
}
