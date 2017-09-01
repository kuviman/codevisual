#[macro_use]
extern crate codevisual;

pub ( crate ) use codevisual::prelude::*;
pub ( crate ) use codevisual::ugli;
#[cfg(target_os = "emscripten")]
pub ( crate ) use codevisual::brijs;

#[cfg(target_os = "emscripten")]
extern crate codewars2017_web;

struct CodeWars2017 {
    app: Rc<codevisual::Application>,
}

impl codevisual::Game for CodeWars2017 {
    type Resources = ();

    fn get_title() -> String {
        String::from("CodeWars 2017")
    }

    fn new(app: Rc<codevisual::Application>, resources: Self::Resources) -> Self {
        Self {
            app: app.clone()
        }
    }

    fn update(&mut self, delta_time: f64) {}

    fn draw(&mut self) {
        let mut framebuffer = ugli::default_framebuffer(self.app.ugli_context());
        let framebuffer = &mut framebuffer;
        ugli::clear(framebuffer, Some(Color::rgb(1.0, 1.0, 1.0)), Some(0.0));
    }

    fn handle_event(&mut self, event: codevisual::Event) {}
}

fn main() {
    #[cfg(target_os = "emscripten")]
    {
        brijs::run_script(codewars2017_web::JS_SOURCE);
    }
    codevisual::run::<CodeWars2017>()
}