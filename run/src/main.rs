#[macro_use]
extern crate codevisual;

pub ( crate ) use codevisual::prelude::*;
pub ( crate ) use codevisual::ugli;

mod unit;

pub ( crate ) use unit::*;

struct Run {
    app: Rc<codevisual::Application>,
    player: Unit,
}

impl codevisual::Game for Run {
    type Resources = ();

    fn get_title() -> String {
        String::from("RUN")
    }

    fn new(app: &Rc<codevisual::Application>, resources: Self::Resources) -> Self {
        Self {
            app: app.clone(),
            player: Unit::new(),
        }
    }

    fn update(&mut self, delta_time: f64) {}

    fn draw(&mut self) {
        let mut framebuffer = self.app.ugli_context().default_framebuffer();
        let framebuffer = &mut framebuffer;
        ugli::clear(framebuffer, Some(Color::BLACK), None);
    }

    fn handle_event(&mut self, event: codevisual::Event) {}
}

fn main() {
    codevisual::run::<Run>();
}