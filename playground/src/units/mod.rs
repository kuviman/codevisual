use codevisual::commons::*;
use codevisual::{self, draw};

use std;

use {MAP_SIZE, TICK_TIME};

#[derive(Vertex, Debug, Copy, Clone)]
pub struct InstanceData {
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
    u_texture: Rc<draw::Texture>,
}

pub struct Units {
    current_time: f32,
    pub actions_per_tick: usize,
    next_action: f32,
    geometry: draw::InstancedGeometry<InstanceData, ::obj::Geometry>,
    shader: draw::Shader,
    uniforms: Uniforms,
}

pub const MAX_COUNT: usize = 10000;
pub const MIN_SIZE: f32 = 0.5;
pub const MAX_SIZE: f32 = 1.5;
pub const SPEED: f32 = 15.0;

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
                                   i_size: random::<f32>() * (MAX_SIZE - MIN_SIZE) + MAX_SIZE,
                                   i_color: Color::rgb(1.0, random::<f32>(), 0.0),
                                   i_angle: 0.0,
                                   i_start_angle: 0.0,
                               });
        }
        let model = ::obj::parse(app, include_str!("../../public/assets/car.obj"));
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
            actions_per_tick: 0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.current_time += delta_time;
        self.next_action -= delta_time;
        while self.next_action < 0.0 {
            self.next_action += TICK_TIME;
            for _ in 0..self.actions_per_tick {
                let i = random_range(0..self.geometry.get_instance_data().len());
                let mut cur = self.geometry.get_instance_data_mut().index_mut(i);
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

    pub fn render<T: draw::Target>(&mut self,
                                   count: usize,
                                   target: &mut T,
                                   global_uniforms: &::GlobalUniforms) {
        self.uniforms.u_time = global_uniforms.u_time;
        self.uniforms.u_matrix = global_uniforms.u_matrix;
        target.draw(&self.geometry.slice(0..count), &self.shader, &self.uniforms);
    }
}