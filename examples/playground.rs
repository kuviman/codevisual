extern crate codevisual;
#[macro_use]
extern crate codevisual_derive;

use codevisual::common::*;
use codevisual::draw;

#[derive(Vertex, Debug, Copy, Clone)]
struct Vertex {
    a_v: Vec3<f32>,
    a_vt: Vec2<f32>,
    a_n: Vec3<f32>,
}

#[derive(Vertex, Debug, Copy, Clone)]
struct Instance {
    i_start_pos: Vec2<f32>,
    i_speed: Vec2<f32>,
    i_start_time: f32,
    i_color: Color,
    i_size: f32,
    i_angle: f32,
    i_start_angle: f32,
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
    geometry: draw::InstancedGeometry<Instance, draw::PlainGeometry<Vertex>>,
    shader: draw::Shader,
    uniforms: Uniforms,
    draw_count: usize,
    actions_per_tick: usize,
    start_drag: Option<Vec2<i32>>,
    camera_distance: f32,
    pos: Vec2<f32>,
    time_scale: f32,
}

const MIN_CAMERA_DIST: f32 = 6.0;
const MAX_CAMERA_DIST: f32 = 2000.0;
const COUNT: usize = 10000;
const MAX_SIZE: f32 = 1.5;
const MIN_SIZE: f32 = 0.5;
const MAP_SIZE: f32 = 1000.0;
const ACTION_TICK: f32 = 0.016666;
const SPEED: f32 = 15.0;

impl Test {
    fn new() -> Self {
        let mut instances = Vec::new();
        for _ in 0..COUNT {
            instances.push(Instance {
                               i_start_pos: vec2(random::<f32>() * 2.0 - 1.0,
                                                 random::<f32>() * 2.0 - 1.0) *
                                            MAP_SIZE,
                               i_speed: vec2(0.0, 0.0),
                               i_start_time: 0.0,
                               i_size: random::<f32>() * (MAX_SIZE - MIN_SIZE) + MAX_SIZE,
                               i_color: Color::rgb(1.0, random::<f32>(), 0.0),
                               i_angle: 0.0,
                               i_start_angle: 0.0,
                           });
        }
        let texture = codevisual::draw::Texture::load("assets/car.png").unwrap();
        let vertices = {
            let mut v = Vec::new();
            let mut n = Vec::new();
            let mut vt = Vec::new();
            let mut result = Vec::new();
            for line in include_str!("public/assets/car.obj").lines() {
                let line: &str = line;
                if line.starts_with("v ") {
                    let mut parts = line.split_whitespace();
                    parts.next();
                    let x: f32 = parts.next().unwrap().parse().unwrap();
                    let y: f32 = parts.next().unwrap().parse().unwrap();
                    let z: f32 = parts.next().unwrap().parse().unwrap();
                    v.push(vec3(x, z, y));
                } else if line.starts_with("vn") {
                    let mut parts = line.split_whitespace();
                    parts.next();
                    let x: f32 = parts.next().unwrap().parse().unwrap();
                    let y: f32 = parts.next().unwrap().parse().unwrap();
                    let z: f32 = parts.next().unwrap().parse().unwrap();
                    n.push(vec3(x, z, y));

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
                        // let normal = Vec3::cross(cur[i - 1].a_v - cur[0].a_v,
                        //                          cur[i].a_v - cur[0].a_v);
                        // result.push(Vertex {
                        //                 a_n: normal,
                        //                 ..cur[0]
                        //             });
                        // result.push(Vertex {
                        //                 a_n: normal,
                        //                 ..cur[i - 1]
                        //             });
                        // result.push(Vertex {
                        //                 a_n: normal,
                        //                 ..cur[i]
                        //             });
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
            },
            next_action: 0.0,
            draw_count: COUNT,
            actions_per_tick: 1000,
            start_drag: None,
            camera_distance: MAX_CAMERA_DIST / 2.0,
            pos: vec2(0.0, 0.0),
            time_scale: 1.0,
        }
    }
}

use draw::Target as DrawTarget;

impl codevisual::Game for Test {
    fn update(&mut self, mut delta_time: f32) {
        delta_time *= self.time_scale;
        self.current_time += delta_time;
        self.next_action -= delta_time;
        while self.next_action < 0.0 {
            self.next_action += ACTION_TICK;
            for _ in 0..self.actions_per_tick {
                let i = random::<usize>() % self.geometry.get_instance_data().len();
                let ref mut cur = *self.geometry.get_instance_data_mut().index_mut(i);
                let mut target = cur.i_start_pos +
                                 vec2(random::<f32>() * 2.0 - 1.0, random::<f32>() * 2.0 - 1.0) *
                                 MAP_SIZE;
                target.x = target.x.min(MAP_SIZE).max(-MAP_SIZE);
                target.y = target.y.min(MAP_SIZE).max(-MAP_SIZE);
                let cur_pos = cur.i_start_pos +
                              cur.i_speed * (self.current_time - cur.i_start_time);
                cur.i_start_pos = cur_pos;
                cur.i_speed = (target - cur_pos).normalize() * SPEED;
                let current_angle = {
                    let mut diff = cur.i_angle - cur.i_start_angle;
                    let PI = std::f32::consts::PI;
                    if diff < -PI {
                        diff += 2.0 * PI;
                    }
                    if diff > PI {
                        diff -= 2.0 * PI;
                    }
                    let passed_time = self.current_time - cur.i_start_time;
                    const W: f32 = 10.0;
                    cur.i_start_angle + diff.max(-W * passed_time).min(W * passed_time)
                };
                cur.i_start_time = self.current_time;
                let target_angle = f32::atan2(cur.i_speed.y, cur.i_speed.x);
                cur.i_start_angle = current_angle;
                cur.i_angle = target_angle;
            }
        }
    }
    fn render<T: DrawTarget>(&mut self, target: &mut T) {
        target.clear(Color::rgb(0.0, 0.0, 0.0));
        self.uniforms.u_time = self.current_time;
        self.uniforms.u_matrix = {
            let (w, h) = codevisual::Application::get_instance().get_size();
            Mat4::perspective(std::f32::consts::PI / 4.0,
                              w as f32 / h as f32,
                              1.0,
                              100000.0) *
            Mat4::translate(vec3(self.pos.x, self.pos.y, -self.camera_distance))
        };
        target.draw(&self.geometry.slice(0..self.draw_count),
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
                    self.pos += vec2((x - prev_x) as f32, -(y - prev_y) as f32) /
                                codevisual::Application::get_instance().get_size().1 as f32 *
                                self.camera_distance;
                    self.start_drag = Some(vec2(x, y));
                }
            }
            MouseUp { button: codevisual::MouseButton::Left, .. } => self.start_drag = None,
            Wheel { delta } => {
                self.camera_distance = (self.camera_distance * f32::exp(delta as f32 / 1000.0))
                    .min(MAX_CAMERA_DIST)
                    .max(MIN_CAMERA_DIST)
            }
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
                                                            default_value: test.draw_count as i32,
                                                            setter: &mut |new_value| {
        println!("Drawing {} instances", new_value);
        test.draw_count = new_value as usize;
    },
                                                        });
    codevisual::Application::get_instance().add_setting(codevisual::Setting::I32 {
                                                            name: "Actions per tick",
                                                            min_value: 0,
                                                            max_value: 1000,
                                                            default_value: test.actions_per_tick as
                                                                           i32,
                                                            setter: &mut |new_value| {
        test.actions_per_tick = new_value as usize;
    },
                                                        });
    codevisual::Application::get_instance().add_setting(codevisual::Setting::I32 {
                                                            name: "Time scale",
                                                            min_value: 0,
                                                            max_value: 200,
                                                            default_value: 100,
                                                            setter: &mut |new_value| {
                                                                             test.time_scale =
                                                                                 new_value as f32 /
                                                                                 100.0;
                                                                         },
                                                        });
    codevisual::run(&mut test);
}