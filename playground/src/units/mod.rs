use codevisual::commons::*;
use codevisual::{self, draw};

use std;

use {MAP_SIZE, TICK_TIME};

#[derive(Vertex, Debug, Copy, Clone)]
pub struct InstanceData {
    i_start_pos: Vec2<f32>,
    i_speed: Vec2<f32>,
    i_start_time: f32,
    i_finish_time: f32,
    i_color: Color,
    i_size: f32,
    i_angle: f32,
    i_start_angle: f32,
}

#[derive(Uniforms)]
struct Uniforms {
    u_time: f32,
    u_matrix: Mat4<f32>,
    u_texture: Rc<draw::Texture>,
}

pub struct Units {
    current_time: f32,
    draw_count: Rc<Cell<usize>>,
    actions_per_tick: Rc<Cell<usize>>,
    next_action: f32,
    geometry: draw::InstancedGeometry<InstanceData, ::obj::Geometry>,
    shader: draw::Shader,
    uniforms: Uniforms,
}

pub const MAX_COUNT: usize = 10000;
pub const MIN_SIZE: f32 = 3.5;
pub const MAX_SIZE: f32 = 5.0;
pub const SPEED: f32 = 50.0;

impl Units {
    pub fn new(app: &codevisual::Application, resources: &::Resources) -> Self {
        let mut instance_data = Vec::new();
        for _ in 0..MAX_COUNT {
            instance_data.push(InstanceData {
                                   i_start_pos: vec2(random::<f32>() * 2.0 - 1.0,
                                                     random::<f32>() * 2.0 - 1.0) *
                                                MAP_SIZE,
                                   i_speed: vec2(0.0, 0.0),
                                   i_start_time: 0.0,
                                   i_finish_time: 0.0,
                                   i_size: random::<f32>() * (MAX_SIZE - MIN_SIZE) + MAX_SIZE,
                                   i_color: Color::rgb(1.0, random::<f32>(), 0.0),
                                   i_angle: 0.0,
                                   i_start_angle: 0.0,
                               });
        }
        let model = ::obj::parse(app, &resources.car_obj.get());
        Units {
            current_time: 0.0,
            next_action: 0.0,
            geometry: draw::InstancedGeometry::new(app, Rc::new(model), instance_data),
            shader: draw::Shader::compile(&app,
                                          include_str!("vertex.glsl"),
                                          include_str!("fragment.glsl"))
                    .unwrap(),
            uniforms: Uniforms {
                u_time: 0.0,
                u_matrix: Mat4::identity(),
                u_texture: resources.car_texture.clone(),
            },
            draw_count: {
                let setting = Rc::new(Cell::new(0 as usize));
                {
                    let setting = setting.clone();
                    app.add_setting(codevisual::I32Setting {
                                        name: String::from("Count"),
                                        min_value: 0,
                                        max_value: MAX_COUNT as i32,
                                        default_value: setting.get() as i32,
                                        setter: move |new_value| {
                                            println!("Drawing {} instances", new_value);
                                            setting.set(new_value as usize);
                                        },
                                    });
                }
                setting
            },
            actions_per_tick: {
                let setting = Rc::new(Cell::new(0 as usize));
                {
                    let setting = setting.clone();
                    app.add_setting(codevisual::I32Setting {
                                        name: String::from("Actions per tick"),
                                        min_value: 0,
                                        max_value: 1000,
                                        default_value: setting.get() as i32,
                                        setter: move |new_value| {
                                            setting.set(new_value as usize);
                                        },
                                    });
                }
                setting
            },
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.current_time += delta_time;
        self.next_action -= delta_time;
        while self.next_action < 0.0 {
            self.next_action += TICK_TIME;
            for _ in 0..self.actions_per_tick.get() {
                let i = random_range(0..self.geometry.get_instance_data().len());
                let mut cur = self.geometry.get_instance_data_mut().index_mut(i);
                let target_pos = {
                    let mut target_pos =
                        cur.i_start_pos +
                        vec2(random::<f32>() * 2.0 - 1.0, random::<f32>() * 2.0 - 1.0) * MAP_SIZE;
                    target_pos.x = target_pos.x.min(MAP_SIZE).max(-MAP_SIZE);
                    target_pos.y = target_pos.y.min(MAP_SIZE).max(-MAP_SIZE);
                    target_pos
                };
                let cur_pos = cur.i_start_pos +
                              cur.i_speed *
                              (self.current_time.min(cur.i_finish_time) - cur.i_start_time);
                cur.i_start_pos = cur_pos;
                cur.i_speed = (target_pos - cur_pos).normalize() * SPEED;
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
                cur.i_finish_time = cur.i_start_time + (target_pos - cur_pos).len() / SPEED;
                let target_angle = f32::atan2(cur.i_speed.y, cur.i_speed.x);
                cur.i_start_angle = current_angle;
                cur.i_angle = target_angle;
            }
        }
    }

    pub fn render<T: draw::Target>(&mut self, target: &mut T, global_uniforms: &::GlobalUniforms) {
        self.uniforms.u_time = global_uniforms.u_time;
        self.uniforms.u_matrix = global_uniforms.u_matrix;
        target.draw(&self.geometry.slice(0..self.draw_count.get()),
                    &self.shader,
                    &draw::uniform::cons(global_uniforms, &self.uniforms));
    }
}