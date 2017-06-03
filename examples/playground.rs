pub extern crate codevisual;
extern crate rand;

use codevisual::common::*;
use codevisual::draw;

struct Vertex {
    a_pos: Vec2,
    a_color: Color,
}

struct Instance {
    i_pos: Vec2,
    i_speed: f32,
}

impl draw::vertex::Data for Vertex {
    fn walk_attributes<F: draw::vertex::AttributeConsumer>(&self, f: &mut F) {
        f.consume("a_pos", &self.a_pos);
        f.consume("a_color", &self.a_color);
    }
}

impl draw::vertex::Data for Instance {
    fn walk_attributes<F: draw::vertex::AttributeConsumer>(&self, f: &mut F) {
        f.consume("i_pos", &self.i_pos);
        f.consume("i_speed", &self.i_speed);
    }
}

impl Vertex {
    fn new(x: f32, y: f32, c: f32) -> Self {
        Self {
            a_pos: Vec2::new(x, y),
            a_color: Color::rgb(c, c / 2.0, 0.0),
        }
    }
}

struct Uniforms {
    u_time: f32,
}

impl draw::uniform::Data for Uniforms {
    fn walk<F: draw::uniform::ValueConsumer>(&self, f: &mut F) {
        f.consume("u_time", &self.u_time);
    }
}

struct Test {
    current_time: f32,
    geometry: draw::Geometry<Vertex, Instance>,
    shader: draw::Shader,
    uniforms: Uniforms,
}

impl Test {
    fn new() -> Self {
        let r = 1e-2;
        let mut instances = Vec::new();
        for _ in 0..100000 {
            instances.push(Instance {
                               i_pos: Vec2::new(rand::random::<f32>() * 2.0 - 1.0,
                                                rand::random::<f32>() * 2.0 - 1.0),
                               i_speed: rand::random::<f32>() * 4.0 - 2.0,
                           });
        }
        Self {
            current_time: 0.0,
            shader: codevisual::draw::Shader::compile(include_str!("vertex.glsl"),
                                                      include_str!("fragment.glsl"))
                    .unwrap(),
            geometry: codevisual::draw::Geometry::new_instanced(draw::geometry::Mode::TriangleFan,
                                                                &[Vertex::new(-r, -r, 0.0),
                                                                  Vertex::new(r, -r, 0.25),
                                                                  Vertex::new(r, r, 0.5),
                                                                  Vertex::new(-r, r, 0.75)],
                                                                &instances)
                    .unwrap(),
            uniforms: Uniforms { u_time: 0.0 },
        }
    }
}

use draw::Target as DrawTarget;

impl codevisual::Game for Test {
    fn update(&mut self, delta_time: f32) {
        self.current_time += delta_time;
    }
    fn render<T: DrawTarget>(&mut self, target: &mut T) {
        target.clear(Color::rgb(self.current_time.fract(), 0.8, 1.0));
        self.uniforms.u_time = self.current_time;
        target.draw(&self.geometry, &self.shader, &self.uniforms);
    }
}

fn main() {
    codevisual::run(Test::new());

    // Hack (WTF??)
    unsafe {
        codevisual::platform::ffi::emscripten_GetProcAddress(std::ffi::CString::new("abacaba")
                                                                 .unwrap()
                                                                 .into_raw() as
                                                             *const _);
    }
}