extern crate codevisual;
extern crate ugli;

use codevisual::prelude::*;

struct EventsExample;

impl codevisual::Game for EventsExample {
    fn new(_: &Rc<codevisual::App>) -> Self {
        Self {}
    }
    fn title() -> String {
        String::from("CodeVisual Example - Events")
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::rgb(0.2, 1.0, 0.2)), None);
    }
    fn handle_event(&mut self, event: codevisual::Event) {
        println!("{:?}", event);
    }
}

fn main() {
    codevisual::run::<EventsExample>();
}
