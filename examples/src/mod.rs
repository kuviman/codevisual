use codevisual;
use codevisual::commons::*;
use codevisual::draw;

use std;
use std::rc::Rc;
use std::cell::RefCell;

const MIN_CAMERA_DIST: f32 = 6.0;
const MAX_CAMERA_DIST: f32 = 2000.0;
const COUNT: usize = 10000;
const MAX_SIZE: f32 = 1.5;
const MIN_SIZE: f32 = 0.5;
const MAP_SIZE: f32 = 1000.0;
const ACTION_TICK: f32 = 0.016666;
const SPEED: f32 = 15.0;

#[derive(Vertex, Debug, Copy, Clone)]
struct Vertex {
    a_v: Vec3<f32>,
    a_vt: Vec2<f32>,
    a_n: Vec3<f32>,
}

#[derive(Vertex, Debug, Copy, Clone)]
struct GroundVertex {
    a_pos: Vec3<f32>,
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

#[derive(Uniforms)]
struct Uniforms {
    u_time: f32,
    u_matrix: Mat4<f32>,
    u_texture: codevisual::draw::Texture,
    u_grass_texture: codevisual::draw::Texture,
    u_dirt_texture: codevisual::draw::Texture,
    u_map_texture: codevisual::draw::Texture,
}

struct Settings {
    actions_per_tick: usize,
    time_scale: f32,
    draw_count: usize,
}

pub struct Test {
    app: Rc<codevisual::Application>,
    current_time: f32,
    next_action: f32,
    geometry: draw::InstancedGeometry<Instance, draw::PlainGeometry<Vertex>>,
    ground_geometry: draw::PlainGeometry<GroundVertex>,
    shader: draw::Shader,
    ground_shader: draw::Shader,
    uniforms: Uniforms,
    start_drag: Option<Vec2>,
    camera_distance: f32,
    pos: Vec2<f32>,
    settings: Rc<RefCell<Settings>>,
    prev_zoom_touchdist: f32,
}

impl Test {
    fn create_settings(app: &codevisual::Application) -> Rc<RefCell<Settings>> {
        let settings = Rc::new(RefCell::new(Settings {
                                                time_scale: 1.0,
                                                actions_per_tick: 1000,
                                                draw_count: COUNT,
                                            }));
        {
            let settings = settings.clone();
            app.add_setting(codevisual::I32Setting {
                                name: String::from("Count"),
                                min_value: 1,
                                max_value: COUNT as i32,
                                default_value: {
                                    let borrow = settings.borrow();
                                    borrow.draw_count as i32
                                },
                                setter: move |new_value| {
                                    println!("Drawing {} instances", new_value);
                                    settings.borrow_mut().draw_count = new_value as usize;
                                },
                            });
        }
        {
            let settings = settings.clone();
            app.add_setting(codevisual::I32Setting {
                                name: String::from("Actions per tick"),
                                min_value: 0,
                                max_value: 1000,
                                default_value: {
                                    let borrow = settings.borrow();
                                    borrow.actions_per_tick as i32
                                },
                                setter: move |new_value| {
                                    settings.borrow_mut().actions_per_tick = new_value as usize;
                                },
                            });
        }
        {
            let settings = settings.clone();
            app.add_setting(codevisual::I32Setting {
                                name: String::from("Time scale"),
                                min_value: 0,
                                max_value: 200,
                                default_value: 100,
                                setter: move |new_value| {
                                    settings.borrow_mut().time_scale = new_value as f32 / 100.0;
                                },
                            });
        }
        settings
    }
}

use self::draw::Target as DrawTarget;

impl codevisual::Game for Test {
    fn new(app: Rc<codevisual::Application>) -> Self {
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
        let vertices = {
            let mut v = Vec::new();
            let mut n = Vec::new();
            let mut vt = Vec::new();
            let mut result = Vec::new();
            for line in include_str!("../public/assets/car.obj").lines() {
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
                    }
                }
            }
            result
        };
        println!("Vertex count = {}", vertices.len());
        app.set_cursor_type(codevisual::CursorType::Pointer);
        let shader = draw::Shader::compile(&app,
                                           include_str!("shaders/model/vertex.glsl"),
                                           include_str!("shaders/model/fragment.glsl"))
                .unwrap();
        let ground_shader = draw::Shader::compile(&app,
                                                  include_str!("shaders/ground/vertex.glsl"),
                                                  include_str!("shaders/ground/fragment.glsl"))
                .unwrap();
        let uniforms = Uniforms {
            u_time: 0.0,
            u_matrix: Mat4::identity(),
            u_texture: codevisual::draw::Texture::load(&app, "assets/car.png").unwrap(),
            u_grass_texture: codevisual::draw::Texture::load(&app, "assets/grass.png").unwrap(),
            u_dirt_texture: codevisual::draw::Texture::load(&app, "assets/dirt.png").unwrap(),
            u_map_texture: codevisual::draw::Texture::load(&app, "assets/map.png").unwrap(),
        };
        let geometry = draw::InstancedGeometry::new(&app, std::rc::Rc::new(draw::PlainGeometry::new(&app, draw::geometry::Mode::Triangles, vertices)),
                                                   instances);
        let ground_geometry =
            draw::PlainGeometry::new(&app,
                                     draw::geometry::Mode::TriangleFan,
                                     vec![GroundVertex { a_pos: vec3(-MAP_SIZE, -MAP_SIZE, 0.0) },
                                          GroundVertex { a_pos: vec3(MAP_SIZE, -MAP_SIZE, 0.0) },
                                          GroundVertex { a_pos: vec3(MAP_SIZE, MAP_SIZE, 0.0) },
                                          GroundVertex { a_pos: vec3(-MAP_SIZE, MAP_SIZE, 0.0) }]);
        let settings = Self::create_settings(&app);
        Self {
            app,
            current_time: 0.0,
            shader,
            ground_shader,
            geometry,
            ground_geometry,
            uniforms,
            next_action: 0.0,
            start_drag: None,
            camera_distance: MAX_CAMERA_DIST / 2.0,
            pos: vec2(0.0, 0.0),
            settings,
            prev_zoom_touchdist: 0.0,
        }
    }
    fn update(&mut self, mut delta_time: f32) {
        delta_time *= self.settings.borrow().time_scale;
        self.current_time += delta_time;
        self.next_action -= delta_time;
        while self.next_action < 0.0 {
            self.next_action += ACTION_TICK;
            for _ in 0..self.settings.borrow().actions_per_tick {
                let i = random_range(0..self.geometry.get_instance_data().len());
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
                    const PI: f32 = std::f32::consts::PI;
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
        target.clear(Color::rgb(1.0, 1.0, 1.0));
        self.uniforms.u_time = self.current_time;
        self.uniforms.u_matrix = {
            let (w, h) = self.app.get_size();
            Mat4::perspective(std::f32::consts::PI / 4.0,
                              w as f32 / h as f32,
                              1.0,
                              100000.0) *
            Mat4::translate(vec3(self.pos.x, self.pos.y, -self.camera_distance))
        };
        target.draw(&self.geometry.slice(0..self.settings.borrow().draw_count),
                    &self.shader,
                    &self.uniforms);
        target.draw(&self.ground_geometry, &self.ground_shader, &self.uniforms);
    }
    fn handle_event(&mut self, event: codevisual::Event) {
        use codevisual::Event::*;
        println!("{:?}", event);
        match event {
            MouseDown {
                x,
                y,
                button: codevisual::MouseButton::Left,
            } => {
                self.app.set_cursor_type(codevisual::CursorType::Drag);
                self.start_drag = Some(vec2(x, y));
            }
            MouseMove { x, y } => {
                if let Some(Vec2 {
                                x: prev_x,
                                y: prev_y,
                            }) = self.start_drag {
                    self.pos += vec2((x - prev_x) as f32, -(y - prev_y) as f32) /
                                self.app.get_size().1 as f32 *
                                self.camera_distance;
                    self.start_drag = Some(vec2(x, y));
                }
            }
            MouseUp { button: codevisual::MouseButton::Left, .. } => {
                self.app.set_cursor_type(codevisual::CursorType::Pointer);
                self.start_drag = None;
            }
            TouchStart { touches } => {
                if touches.len() == 1 {
                    self.start_drag = Some(touches[0].position);
                }
                if touches.len() == 2 {
                    self.prev_zoom_touchdist = (touches[0].position - touches[1].position)
                        .len() as f32;
                }

            }
            TouchMove { touches } => {
                if touches.len() == 1 {
                    let Vec2 { x, y } = touches[0].position;
                    if let Some(Vec2 {
                                    x: prev_x,
                                    y: prev_y,
                                }) = self.start_drag {
                        self.pos += vec2((x - prev_x) as f32, -(y - prev_y) as f32) /
                                    self.app.get_size().1 as f32 *
                                    self.camera_distance;
                        self.start_drag = Some(vec2(x, y));
                    }
                } else if touches.len() == 2 {
                    let now_dist = (touches[0].position - touches[1].position).len() as f32;
                    self.camera_distance /= now_dist / self.prev_zoom_touchdist;
                    self.prev_zoom_touchdist = now_dist;
                }
            }
            TouchEnd => {
                self.start_drag = None;
            }
            Wheel { delta } => {
                self.camera_distance = (self.camera_distance * f32::exp(delta as f32 / 1000.0))
                    .min(MAX_CAMERA_DIST)
                    .max(MIN_CAMERA_DIST)
            }
            _ => (),
        }
    }
}