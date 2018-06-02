extern crate codevisual;
#[macro_use]
extern crate ugli;

use codevisual::prelude::*;

pub struct App<T: codevisual::App> {
    context: Rc<codevisual::Context>,
    inner: T,
    time: f64,
    frames: usize,
    fps: usize,
    last_event: Option<codevisual::Event>,
}

impl<T: codevisual::App> codevisual::App for App<T> {
    fn title() -> String {
        T::title() + " (debug overlay enabled)"
    }
    fn new(context: &Rc<codevisual::Context>) -> Self {
        Self {
            inner: T::new(context),
            context: context.clone(),
            time: 0.0,
            frames: 0,
            fps: 0,
            last_event: None,
        }
    }
    fn update(&mut self, delta_time: f64) {
        self.inner.update(delta_time);
        self.time += delta_time;
        self.frames += 1;
        if self.time > 1.0 {
            self.fps = (self.frames as f64 / self.time) as _;
            self.time = 0.0;
            self.frames = 0;
        }
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.inner.draw(framebuffer);
        self.context.default_font().draw(
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
            self.context.default_font().draw_aligned(
                framebuffer,
                &format!("last event: {:?}", event),
                pos,
                1.0,
                16.0,
                Color::WHITE,
            );
            pos.y -= 16.0;
        }
        self.context.default_font().draw_aligned(
            framebuffer,
            &format!("mouse pos: {:?}", self.context.window().mouse_pos()),
            pos,
            1.0,
            16.0,
            Color::WHITE,
        );
        pos.y -= 16.0;
        self.context.default_font().draw_aligned(
            framebuffer,
            &format!("pressed keys: {:?}", self.context.window().pressed_keys()),
            pos,
            1.0,
            16.0,
            Color::WHITE,
        );
    }
    fn handle_event(&mut self, event: codevisual::Event) {
        self.last_event = Some(event.clone());
        self.inner.handle_event(event);
    }
}

pub fn run<T: codevisual::App>() {
    codevisual::Context::run::<App<T>>();
}
