#[macro_use]
extern crate codevisual;
#[macro_use]
extern crate ugli;

use codevisual::prelude::*;

struct Demo {
    app: Rc<codevisual::Context>,
    time: f64,
    frames: usize,
    fps: usize,
    last_event: Option<codevisual::Event>,
}

impl codevisual::App for Demo {
    fn new(app: &Rc<codevisual::Context>) -> Self {
        Demo {
            app: app.clone(),
            time: 0.0,
            frames: 0,
            fps: 0,
            last_event: None,
        }
    }
    fn title() -> String {
        String::from("CodeVisual Demo")
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
        let center = vec2(framebuffer.get_size().x as _, framebuffer.get_size().y as _) / 2.0;
        self.app.default_font().draw_aligned(
            framebuffer,
            "CodeVisual Demo",
            center,
            0.5,
            32.0,
            Color::WHITE,
        );
        self.app.default_font().draw(
            framebuffer,
            &format!("FPS: {}", self.fps),
            vec2(10.0, 10.0),
            16.0,
            Color::WHITE,
        );
        let mut pos = vec2(
            framebuffer.get_size().x as f32 - 10.0,
            framebuffer.get_size().y as f32 - 26.0,
        );
        if let Some(ref event) = self.last_event {
            self.app.default_font().draw_aligned(
                framebuffer,
                &format!("last event: {:?}", event),
                pos,
                1.0,
                16.0,
                Color::WHITE,
            );
            pos.y -= 16.0;
        }
        self.app.default_font().draw_aligned(
            framebuffer,
            &format!("mouse pos: {:?}", self.app.window().mouse_pos()),
            pos,
            1.0,
            16.0,
            Color::WHITE,
        );
        pos.y -= 16.0;
        self.app.default_font().draw_aligned(
            framebuffer,
            &format!("pressed keys: {:?}", self.app.window().pressed_keys()),
            pos,
            1.0,
            16.0,
            Color::WHITE,
        );
    }
    fn update(&mut self, delta_time: f64) {
        self.time += delta_time;
        self.frames += 1;
        if self.time > 1.0 {
            self.fps = (self.frames as f64 / self.time) as _;
            self.time = 0.0;
            self.frames = 0;
        }
    }
    fn handle_event(&mut self, event: codevisual::Event) {
        self.last_event = Some(event);
    }
}

fn main() {
    codevisual::Context::run::<Demo>();
}
