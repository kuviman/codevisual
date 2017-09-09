#![allow(unused_variables)]

extern crate codevisual;

use codevisual::prelude::*;
use codevisual::ugli;

struct Tutorial {
    app: Rc<codevisual::Application>,
}

impl codevisual::Game for Tutorial {
    type Resources = ();
    fn new(app: &Rc<codevisual::Application>, resources: ()) -> Self {
        Tutorial { app: app.clone() }
    }
    fn get_title() -> String {
        String::from("CodeVisual Tutorial 01 - Hello, world!")
    }
    fn update(&mut self, delta_time: f64) {}
    fn draw(&mut self) {
        ugli::clear(
            &mut self.app.ugli_context().default_framebuffer(),
            Some(Color::rgb(0.8, 0.8, 1.0)),
            None,
        );
    }
    fn handle_event(&mut self, event: codevisual::Event) {}
}

fn main() {
    codevisual::run::<Tutorial>();
}
