use codevisual::commons::*;
use codevisual::{self, draw};

use std;

use {MAP_SIZE, TICK_TIME};

#[derive(Vertex)]
pub struct QuadVertex {
    a_v: Vec2<f32>,
}

#[derive(Uniforms)]
pub struct FogUniforms {
    u_time: f32,
}

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

impl InstanceData {
    fn update(&mut self, current_time: f32) {
        let target_pos = {
            let mut target_pos = self.i_start_pos +
                                 vec2(random::<f32>() * 2.0 - 1.0, random::<f32>() * 2.0 - 1.0) *
                                 MAP_SIZE;
            target_pos.x = target_pos.x.min(MAP_SIZE).max(-MAP_SIZE);
            target_pos.y = target_pos.y.min(MAP_SIZE).max(-MAP_SIZE);
            target_pos
        };
        let cur_pos = self.i_start_pos +
                      self.i_speed * (current_time.min(self.i_finish_time) - self.i_start_time);
        self.i_start_pos = cur_pos;
        self.i_speed = (target_pos - cur_pos).normalize() * SPEED;
        let current_angle = {
            let mut diff = self.i_angle - self.i_start_angle;
            const PI: f32 = std::f32::consts::PI;
            if diff < -PI {
                diff += 2.0 * PI;
            }
            if diff > PI {
                diff -= 2.0 * PI;
            }
            let passed_time = current_time - self.i_start_time;
            const W: f32 = 10.0;
            self.i_start_angle + diff.max(-W * passed_time).min(W * passed_time)
        };
        self.i_start_time = current_time;
        self.i_finish_time = self.i_start_time + (target_pos - cur_pos).len() / SPEED;
        let target_angle = f32::atan2(self.i_speed.y, self.i_speed.x);
        self.i_start_angle = current_angle;
        self.i_angle = target_angle;
    }
}

#[derive(Uniforms)]
struct Uniforms {
    u_car_texture: Rc<draw::Texture>,
    u_heli_texture: Rc<draw::Texture>,
}

pub struct Units {
    current_time: f32,
    quad: draw::PlainGeometry<QuadVertex>,
    draw_count: Rc<Cell<usize>>,
    actions_per_tick: Rc<Cell<usize>>,
    next_action: f32,
    geometry: ::obj::Geometry,
    instances: draw::vertex::Buffer<InstanceData>,
    heli_geometry: draw::InstancedGeometry<InstanceData, ::obj::Geometry>,
    shader: draw::Shader,
    shader_heli: draw::Shader,
    shader_fog: draw::Shader,
    uniforms: Uniforms,
}

pub const MAX_COUNT: usize = 10000;
pub const MIN_SIZE: f32 = 3.5;
pub const MAX_SIZE: f32 = 5.0;
pub const SPEED: f32 = 50.0;
pub const MAX_APS: usize = 10000;

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
        let heli_model = ::obj::parse(app, &resources.heli_obj.get());
        let heli_instance_data = instance_data.clone();
        Units {
            current_time: 0.0,
            next_action: 0.0,
            geometry: model,
            quad: draw::PlainGeometry::new(app,
                                           draw::geometry::Mode::TriangleFan,
                                           vec![QuadVertex { a_v: vec2(-1.0, -1.0) },
                                                QuadVertex { a_v: vec2(1.0, -1.0) },
                                                QuadVertex { a_v: vec2(1.0, 1.0) },
                                                QuadVertex { a_v: vec2(-1.0, 1.0) }]),
            instances: draw::vertex::Buffer::new(app, instance_data),
            heli_geometry: draw::InstancedGeometry::new(app,
                                                        Rc::new(heli_model),
                                                        heli_instance_data),
            shader: draw::Shader::compile(&app,
                                          include_str!("vertex_car.glsl"),
                                          include_str!("fragment_car.glsl"))
                    .unwrap(),
            shader_heli: draw::Shader::compile(&app,
                                               include_str!("vertex_heli.glsl"),
                                               include_str!("fragment_heli.glsl"))
                    .unwrap(),
            shader_fog: draw::Shader::compile(&app,
                                              include_str!("vertex_fog.glsl"),
                                              include_str!("fragment_fog.glsl"))
                    .unwrap(),
            uniforms: Uniforms {
                u_car_texture: resources.car_texture.clone(),
                u_heli_texture: resources.heli_texture.clone(),
            },
            draw_count: {
                let setting = Rc::new(Cell::new(50 as usize));
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
                let setting = Rc::new(Cell::new(1 as usize));
                {
                    let setting = setting.clone();
                    app.add_setting(codevisual::I32Setting {
                                        name: String::from("Actions per tick"),
                                        min_value: 0,
                                        max_value: MAX_APS as i32,
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
            for geometry in &mut [&mut self.heli_geometry] {
                if self.actions_per_tick.get() == MAX_APS {
                    for unit in geometry
                            .get_instance_data_mut()
                            .slice_mut(..self.draw_count.get())
                            .iter_mut() {
                        unit.update(self.current_time);
                    }
                } else {
                    for _ in 0..self.draw_count.get() * self.actions_per_tick.get() / MAX_APS {
                        let i = random_range(0..geometry.get_instance_data().len());
                        geometry
                            .get_instance_data_mut()
                            .index_mut(i)
                            .update(self.current_time);
                    }
                }
            }
            if self.actions_per_tick.get() == MAX_APS {
                for unit in self.instances.slice_mut(..self.draw_count.get()).iter_mut() {
                    unit.update(self.current_time);
                }
            } else {
                for _ in 0..
                         (self.draw_count.get() * self.actions_per_tick.get() + MAX_APS - 1) /
                         MAX_APS {
                    let i = random_range(0..self.instances.len());
                    self.instances.index_mut(i).update(self.current_time);
                }
            }
        }
    }

    pub fn get_fog(&self, texture: &mut draw::Texture, u_time: f32) {
        use draw::Target;
        let mut target = texture.as_target();
        const K: f32 = 0.5;
        target.clear(Color::rgb(K, K, K));
        use draw::vertex::BufferView;
        target.draw(&draw::geometry::Immediate::new(&self.quad,
                                                    &self.instances.slice(0..
                                                                          self.draw_count.get())),
                    &self.shader_fog,
                    &FogUniforms { u_time });
    }

    pub fn render<T: draw::Target>(&mut self, target: &mut T, global_uniforms: &::GlobalUniforms) {
        use draw::vertex::BufferView;
        target.draw(&draw::geometry::Immediate::new(&self.geometry,
                                                    &self.instances.slice(0..
                                                                          self.draw_count.get())),
                    &self.shader,
                    &draw::uniform::cons(global_uniforms, &self.uniforms));
    }
    pub fn render2<T: draw::Target>(&mut self,
                                    target: &mut T,
                                    global_uniforms: &::GlobalUniforms) {
        target.draw(&self.heli_geometry.slice(0..self.draw_count.get()),
                    &self.shader_heli,
                    &draw::uniform::cons(global_uniforms, &self.uniforms));
    }
}