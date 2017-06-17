extern crate codevisual;

use codevisual::common::*;
use codevisual::draw;

#[derive(Debug, Copy, Clone)]
struct Vertex {
    a_v: Vec3<f32>,
    a_vt: Vec2<f32>,
    a_n: Vec3<f32>,
}

impl draw::vertex::Data for Vertex {
    fn walk_attributes<F: draw::vertex::AttributeConsumer>(&self, f: &mut F) {
        f.consume("a_v", &self.a_v);
        f.consume("a_n", &self.a_n);
        f.consume("a_vt", &self.a_vt);
    }
}

struct Instance {
    i_start_pos: Vec2<f32>,
    i_speed: Vec2<f32>,
    i_start_time: f32,
    i_color: Color,
    i_size: f32,
    i_angle: f32,
}

impl draw::vertex::Data for Instance {
    fn walk_attributes<F: draw::vertex::AttributeConsumer>(&self, f: &mut F) {
        f.consume("i_start_pos", &self.i_start_pos);
        f.consume("i_speed", &self.i_speed);
        f.consume("i_start_time", &self.i_start_time);
        f.consume("i_color", &self.i_color);
        f.consume("i_size", &self.i_size);
        f.consume("i_angle", &self.i_angle);
    }
}

struct Uniforms {
    u_time: f32,
    u_matrix: Mat4<f32>,
    u_texture: codevisual::draw::Texture,
    u_scale: f32,
    u_pos: Vec2<f32>,
}

impl draw::uniform::Data for Uniforms {
    fn walk<F: draw::uniform::ValueConsumer>(&self, f: &mut F) {
        f.consume("u_time", &self.u_time);
        f.consume("u_matrix", &self.u_matrix);
        f.consume("u_texture", &self.u_texture);
        f.consume("u_scale", &self.u_scale);
        f.consume("u_pos", &self.u_pos);
    }
}

struct Test {
    current_time: f32,
    next_action: f32,
    geometry: draw::InstancedGeometry<Instance, draw::PlainGeometry<Vertex>>,
    shader: draw::Shader,
    uniforms: Uniforms,
    draw_count: i32,
    actions_per_tick: i32,
    start_drag: Option<Vec2<i32>>,
}

const COUNT: usize = 10000;
const SLOW_DOWN: f32 = 20.0;
const MAX_SIZE: f32 = 0.0003;
const MIN_SIZE: f32 = 0.0001;
const ACTION_TICK: f32 = 0.016666;

impl Test {
    fn new() -> Self {
        let mut instances = Vec::new();
        for _ in 0..COUNT {
            instances.push(Instance {
                               i_start_pos: vec2(0.0, 0.0),
                               i_speed: vec2(0.0, 0.0),
                               i_start_time: 0.0,
                               i_size: random::<f32>() * (MAX_SIZE - MIN_SIZE) + MAX_SIZE,
                               i_color: Color::rgb(1.0, random::<f32>(), 0.0),
                               i_angle: 0.0,
                           });
        }
        let texture = codevisual::draw::Texture::load("assets/BUGGY_BLUE.png").unwrap();
        let vertices = {
            let mut v = Vec::new();
            let mut n = Vec::new();
            let mut vt = Vec::new();
            let mut result = Vec::new();
            for line in include_str!("public/assets/BUGGY.obj").lines() {
                let line: &str = line;
                if line.starts_with("v ") {
                    let mut parts = line.split_whitespace();
                    parts.next();
                    let x: f32 = parts.next().unwrap().parse().unwrap();
                    let y: f32 = parts.next().unwrap().parse().unwrap();
                    let z: f32 = parts.next().unwrap().parse().unwrap();
                    v.push(vec3(x, z, y) / 1000.0);
                } else if line.starts_with("vn") {
                    let mut parts = line.split_whitespace();
                    parts.next();
                    let x: f32 = parts.next().unwrap().parse().unwrap();
                    let y: f32 = parts.next().unwrap().parse().unwrap();
                    let z: f32 = parts.next().unwrap().parse().unwrap();
                    n.push(vec3(x, y, z));

                } else if line.starts_with("vt") {
                    let mut parts = line.split_whitespace();
                    parts.next();
                    let x: f32 = parts.next().unwrap().parse().unwrap();
                    let y: f32 = parts.next().unwrap().parse().unwrap();
                    vt.push(vec2(x, 1.0 - y));
                } else if line.starts_with("f") {
                    let mut parts = line.split_whitespace();
                    parts.next();
                    let to_vertex = |s: &str| {
                        let mut parts = s.split("/");
                        let i_v: usize = parts.next().unwrap().parse().unwrap();
                        let i_vt: usize = parts.next().unwrap().parse().unwrap();
                        let i_n: usize = parts.next().unwrap().parse().unwrap();
                        Vertex {
                            a_v: v[i_v - 1],
                            a_n: n[i_n - 1],
                            a_vt: vt[i_vt - 1],
                        }
                    };
                    let mut cur = Vec::new();
                    while let Some(s) = parts.next() {
                        cur.push(to_vertex(s));
                    }
                    for i in 2..cur.len() {
                        result.push(cur[0]);
                        result.push(cur[i - 1]);
                        result.push(cur[i]);
                    }
                }
            }
            result
        };
        println!("Vertex count = {}", vertices.len());
        Self {
            current_time: 0.0,
            shader: codevisual::draw::Shader::compile(include_str!("vertex.glsl"),
                                                      include_str!("fragment.glsl"))
                    .unwrap(),
            geometry: draw::InstancedGeometry::new(std::rc::Rc::new(draw::PlainGeometry::new(draw::geometry::Mode::Triangles, vertices)),
                                                   instances),
            uniforms: Uniforms {
                u_time: 0.0,
                u_matrix: Mat4::identity(),
                u_texture: texture,
                u_scale: 1.0,
                u_pos: vec2(0.0, 0.0),
            },
            next_action: 0.0,
            draw_count: 10,
            actions_per_tick: 1000,
            start_drag: None,
        }
    }
}

use draw::Target as DrawTarget;

impl codevisual::Game for Test {
    fn update(&mut self, mut delta_time: f32) {
        delta_time /= self.uniforms.u_scale;
        self.current_time += delta_time;
        self.next_action -= delta_time;
        while self.next_action < 0.0 {
            self.next_action += ACTION_TICK;
            for _ in 0..self.actions_per_tick {
                let i = random::<usize>() % self.geometry.get_instance_data().len();
                let ref mut cur = *self.geometry.get_instance_data_mut().index_mut(i);
                let target = vec2(random::<f32>() * 2.0 - 1.0, random::<f32>() * 2.0 - 1.0);
                let cur_pos = cur.i_start_pos +
                              cur.i_speed * (self.current_time - cur.i_start_time);
                cur.i_start_pos = cur_pos;
                cur.i_speed = (target - cur_pos).normalize() / SLOW_DOWN;
                cur.i_start_time = self.current_time;
                cur.i_angle = f32::atan2(cur.i_speed.y, cur.i_speed.x);
            }
        }
    }
    fn render<T: DrawTarget>(&mut self, target: &mut T) {
        target.clear(Color::rgb(0.0, 0.0, 0.0));
        self.uniforms.u_time = self.current_time;
        self.uniforms.u_matrix = {
            let (w, h) = codevisual::Application::get_instance().get_size();
            Mat4::perspective(std::f32::consts::PI / 2.0, w as f32 / h as f32, 0.1, 1000.0)
        };
        target.draw(&self.geometry.slice(0..self.draw_count as usize),
                    &self.shader,
                    &self.uniforms);
    }
    fn handle_event(&mut self, event: codevisual::Event) {
        use codevisual::Event::*;
        match event {
            MouseDown {
                x,
                y,
                button: codevisual::MouseButton::Left,
            } => self.start_drag = Some(vec2(x, y)),
            MouseMove { x, y } => {
                if let Some(Vec2 {
                                x: prev_x,
                                y: prev_y,
                            }) = self.start_drag {
                    self.uniforms.u_pos += vec2((x - prev_x) as f32, -(y - prev_y) as f32) /
                                           1000.0 /
                                           self.uniforms.u_scale;
                    self.start_drag = Some(vec2(x, y));
                }
            }
            MouseUp { button: codevisual::MouseButton::Left, .. } => self.start_drag = None,
            Wheel { delta } => self.uniforms.u_scale *= f32::exp(-delta as f32 / 1000.0),
            _ => (),
        }
    }
}

fn main() {
    let mut test = Test::new();
    codevisual::Application::get_instance().add_setting(codevisual::Setting::I32 {
                                                            name: "Count",
                                                            min_value: 1,
                                                            max_value: COUNT as i32,
                                                            default_value: test.draw_count,
                                                            setter: &mut |new_value| {
        println!("Drawing {} instances", new_value);
        test.draw_count = new_value;
    },
                                                        });
    codevisual::Application::get_instance().add_setting(codevisual::Setting::I32 {
                                                            name: "Actions per tick",
                                                            min_value: 0,
                                                            max_value: 1000,
                                                            default_value: test.actions_per_tick,
                                                            setter: &mut |new_value| {
        test.actions_per_tick = new_value;
    },
                                                        });
    codevisual::run(&mut test);
}