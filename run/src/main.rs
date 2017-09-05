#[macro_use]
extern crate codevisual;

pub ( crate ) use codevisual::prelude::*;
pub ( crate ) use codevisual::ugli;

struct Run {
    app: Rc<codevisual::Application>,
}

impl codevisual::Game for Run {
    type Resources = ();

    fn get_title() -> String {
        String::from("RUN")
    }

    fn new(app: &Rc<codevisual::Application>, resources: Self::Resources) -> Self {
        Self {
            app: app.clone(),
        }
    }

    fn update(&mut self, delta_time: f64) {}

    fn draw(&mut self) {
        let mut framebuffer = ugli::default_framebuffer(self.app.ugli_context());
        let framebuffer = &mut framebuffer;
        ugli::clear(framebuffer, Some(Color::BLACK), None);
    }

    fn handle_event(&mut self, event: codevisual::Event) {}
}

fn main() {
    codevisual::run::<Run>();
}