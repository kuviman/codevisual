#[macro_use]
extern crate codevisual;

pub ( crate ) use codevisual::prelude::*;
pub ( crate ) use codevisual::ugli;

mod unit;

pub use unit::*;

mod static_map;

pub use static_map::*;

shader_library! {
    ShaderLib {
        "global" => include_str!("global.glsl"),
    }
}

type Material<U = (), D = ()> = codevisual::Material<ShaderLib, U, D>;

struct Run {
    app: Rc<codevisual::Application>,
    player: Unit,
    static_map: StaticMap,
}

impl codevisual::Game for Run {
    type Resources = ();

    fn get_title() -> String {
        String::from("RUN")
    }

    fn new(app: &Rc<codevisual::Application>, resources: Self::Resources) -> Self {
        app.window().set_cursor_position({
            let pos = app.window().get_size() / 2;
            vec2(pos.x as f64, pos.y as f64)
        });
        app.window().grab_cursor();
        Self {
            app: app.clone(),
            player: Unit::new(),
            static_map: StaticMap::new(app),
        }
    }

    fn update(&mut self, delta_time: f64) {}

    fn draw(&mut self) {
        let mut framebuffer = self.app.ugli_context().default_framebuffer();
        let framebuffer = &mut framebuffer;
        ugli::clear(framebuffer, Some(Color::BLACK), Some(1.0));
        let projection_matrix = Mat4::perspective(
            std::f32::consts::PI / 2.0,
            framebuffer.get_size().x as f32 / framebuffer.get_size().y as f32,
            1e-1, 1e3);
        let uniforms = uniforms! {
            u_projection_matrix: projection_matrix,
            u_eye_matrix: self.player.eye_matrix(),
        };
        self.static_map.draw(framebuffer, &uniforms);
    }

    fn handle_event(&mut self, event: codevisual::Event) {
        match event {
            codevisual::Event::MouseMove { position } => {
                let center = self.app.window().get_size() / 2;
                let dv = position - vec2(center.x as f64, center.y as f64);
                let dv = vec2(dv.x as f32, dv.y as f32);
                const SENSIVITY: f32 = 5e-3;
                self.player.rotate_head(dv * SENSIVITY);
                self.app.window().set_cursor_position(vec2(center.x as f64, center.y as f64));
            }
            _ => {}
        }
    }
}

fn main() {
    codevisual::run::<Run>();
}