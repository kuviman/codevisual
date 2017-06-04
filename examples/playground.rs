pub extern crate codevisual;
extern crate rand;

use codevisual::common::*;
use codevisual::draw;

struct Vertex {
    a_pos: Vec2<f32>,
}

impl draw::vertex::Data for Vertex {
    fn walk_attributes<F: draw::vertex::AttributeConsumer>(&self, f: &mut F) {
        f.consume("a_pos", &self.a_pos);
    }
}

struct Instance {
    i_start_pos: Vec2<f32>,
    i_speed: Vec2<f32>,
    i_start_time: f32,
    i_color: Color,
    i_size: f32,
}

impl draw::vertex::Data for Instance {
    fn walk_attributes<F: draw::vertex::AttributeConsumer>(&self, f: &mut F) {
        f.consume("i_start_pos", &self.i_start_pos);
        f.consume("i_speed", &self.i_speed);
        f.consume("i_start_time", &self.i_start_time);
        f.consume("i_color", &self.i_color);
        f.consume("i_size", &self.i_size);
    }
}

struct Uniforms {
    u_time: f32,
    u_matrix: Mat4<f32>,
    u_texture: codevisual::draw::Texture,
}

impl draw::uniform::Data for Uniforms {
    fn walk<F: draw::uniform::ValueConsumer>(&self, f: &mut F) {
        f.consume("u_time", &self.u_time);
        f.consume("u_matrix", &self.u_matrix);
        f.consume("u_texture", &self.u_texture);
    }
}

struct Test {
    current_time: f32,
    next_action: f32,
    geometry: draw::Geometry<Vertex, Instance>,
    shader: draw::Shader,
    uniforms: Uniforms,
    instances: Vec<Instance>,
}

const COUNT: usize = 10000;
const SLOW_DOWN: f32 = 20.0;
const MAX_SIZE: f32 = 0.003;
const MIN_SIZE: f32 = 0.001;
const ACTION_TICK: f32 = 0.01666666;

impl Test {
    fn new() -> Self {
        let mut instances = Vec::new();
        for _ in 0..COUNT {
            let start_pos = Vec2::new(rand::random::<f32>() * 2.0 - 1.0,
                                      rand::random::<f32>() * 2.0 - 1.0);
            instances.push(Instance {
                               i_start_pos: start_pos,
                               i_speed: Vec2::new(0.0, 0.0),
                               i_start_time: 0.0,
                               i_size: rand::random::<f32>() * (MAX_SIZE - MIN_SIZE) + MAX_SIZE,
                               i_color: Color::rgb(1.0, rand::random::<f32>(), 0.0),
                           });
        }
        let texture = codevisual::draw::Texture::load("textures/test.png").unwrap();
        Self {
            current_time: 0.0,
            shader: codevisual::draw::Shader::compile(include_str!("vertex.glsl"),
                                                      include_str!("fragment.glsl"))
                    .unwrap(),
            geometry: codevisual::draw::Geometry::new_instanced(draw::geometry::Mode::TriangleFan,
                                                                &[Vertex {
                                                                      a_pos: Vec2::new(-1.0, -1.0),
                                                                  },
                                                                  Vertex {
                                                                      a_pos: Vec2::new(-1.0, 1.0),
                                                                  },
                                                                  Vertex {
                                                                      a_pos: Vec2::new(1.0, 1.0),
                                                                  },
                                                                  Vertex {
                                                                      a_pos: Vec2::new(1.0, -1.0),
                                                                  }],
                                                                &instances)
                    .unwrap(),
            uniforms: Uniforms {
                u_time: 0.0,
                u_matrix: Mat4::identity(),
                u_texture: texture,
            },
            instances,
            next_action: 0.0,
        }
    }
}

use draw::Target as DrawTarget;

impl codevisual::Game for Test {
    fn update(&mut self, delta_time: f32) {
        self.current_time += delta_time;
        self.next_action -= delta_time;
        while self.next_action < 0.0 {
            self.next_action += ACTION_TICK;
            for _ in 0..1000 {
                let i = rand::random::<usize>() % self.instances.len();
                let cur = &mut self.instances[i];
                let target = Vec2::new(rand::random::<f32>() * 2.0 - 1.0,
                                       rand::random::<f32>() * 2.0 - 1.0);
                let cur_pos = cur.i_start_pos +
                              cur.i_speed * (self.current_time - cur.i_start_time);
                cur.i_start_pos = cur_pos;
                cur.i_speed = (target - cur_pos).normalize() / SLOW_DOWN;
                cur.i_start_time = self.current_time;
                self.geometry.set_instance(i, cur);
            }
        }
    }
    fn render<T: DrawTarget>(&mut self, target: &mut T) {
        target.clear(Color::rgb(0.0, 0.0, 0.0));
        self.uniforms.u_time = self.current_time;
        self.uniforms.u_matrix = {
            let (w, h) = codevisual::Application::get_instance().get_size();
            perspective(Deg(90.0), w as f32 / h as f32, 0.1, 1000.0)
        };
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