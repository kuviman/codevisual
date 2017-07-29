use ::*;

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

impl InstanceData {
    fn update(&mut self, current_time: f32) {
        let target_pos = {
            let mut target_pos = self.i_start_pos +
                vec2(random::<f32>() * 2.0 - 1.0, random::<f32>() * 2.0 - 1.0) * MAP_SIZE;
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

#[derive(Debug)]
pub enum UnitType {
    Car,
    Heli,
}

pub const MAX_COUNT: usize = 10000;
pub const MIN_SIZE: f32 = 3.5;
pub const MAX_SIZE: f32 = 5.0;
pub const SPEED: f32 = 50.0;
pub const MAX_APS: usize = 1000;

pub struct Units {
    geometry: obj::Geometry,
    pub instances: ugli::VertexBuffer<InstanceData>,
    shader: codevisual::Shader,
    texture: ugli::Texture2d,
    count: usize,
    pub current_time: f32,
}

impl Units {
    pub fn new(
        app: &codevisual::Application,
        unit_type: UnitType,
        geometry: obj::Geometry,
        texture: ugli::Texture2d,
    ) -> Self {
        let mut instance_data = Vec::new();
        for _ in 0..MAX_COUNT {
            let angle = random::<f32>() * 2.0 * std::f32::consts::PI;
            instance_data.push(InstanceData {
                i_start_pos: vec2(random::<f32>() * 2.0 - 1.0, random::<f32>() * 2.0 - 1.0) *
                    MAP_SIZE,
                i_speed: vec2(0.0, 0.0),
                i_start_time: 0.0,
                i_finish_time: 0.0,
                i_size: random::<f32>() * (MAX_SIZE - MIN_SIZE) + MAX_SIZE,
                i_color: Color::rgb(1.0, random::<f32>(), 0.0),
                i_angle: angle,
                i_start_angle: angle,
            });
        }
        let context = app.get_window().ugli_context();
        Self {
            geometry,
            instances: ugli::VertexBuffer::new(context, instance_data),
            shader: codevisual::Shader::compile::<::ShaderLib>(
                context,
                defines!(HELI: if let UnitType::Heli = unit_type { true } else { false }),
                include_str!("shader.glsl"),
            ),
            texture,
            count: 0,
            current_time: 0.0,
        }
    }
    pub fn update(&mut self, percent: Option<f64>, point_updates: bool) {
        match percent {
            Some(percent) => {
                let count = (self.count as f64 * percent) as usize;
                let indices = rand::sample(&mut thread_rng(), 0..self.count, count);
                if point_updates {
                    for &i in &indices {
                        let mut data = self.instances.slice_mut(i..i + 1);
                        data[0].update(self.current_time);
                    }
                } else {
                    let mut data = self.instances.slice_mut(..self.count);
                    for &i in &indices {
                        data[i].update(self.current_time);
                    }
                }
            }
            None => {
                let mut data = self.instances.slice_mut(..self.count);
                for unit in data.iter_mut() {
                    unit.update(self.current_time);
                }
            }
        };
    }
    pub fn draw<U: ugli::UniformStorage>(
        &mut self,
        framebuffer: &mut ugli::DefaultFramebuffer,
        uniforms: &U,
    ) {
        ugli::draw(
            framebuffer,
            self.shader.ugli_program(),
            ugli::DrawMode::Triangles,
            &ugli::instanced(
                &self.geometry.slice(..),
                &self.instances.slice(..self.count),
            ),
            &(uniforms, uniforms!(u_texture: &self.texture)),
            &ugli::DrawParameters::default(),
        );
    }
}

resources! {
    Resources {
        car_texture: ugli::Texture2d = "assets/car.png",
        heli_texture: ugli::Texture2d = "assets/heli.png",
        car_obj: String = "assets/car.obj",
        heli_obj: String = "assets/heli.obj",
    }
}

pub struct AllUnits {
    app: Rc<codevisual::Application>,
    current_time: f32,
    point_updates: Rc<Cell<bool>>,
    actions_per_tick: Rc<Cell<usize>>,
    next_action: f32,
    pub draw_count: Rc<Cell<usize>>,
    pub cars: Units,
    pub helis: Units,
    screen_used_texture: Option<ugli::Texture2d>,
    screen_used_shader: codevisual::Shader,
}

impl AllUnits {
    pub fn new(app: &Rc<codevisual::Application>, resources: Resources) -> Self {
        let context = app.get_window().ugli_context();
        let cars = Units::new(
            app,
            UnitType::Car,
            obj::parse(app, &resources.car_obj),
            resources.car_texture,
        );
        let helis = Units::new(
            app,
            UnitType::Heli,
            obj::parse(app, &resources.heli_obj),
            resources.heli_texture,
        );
        Self {
            app: app.clone(),
            current_time: 0.0,
            actions_per_tick: {
                let setting = Rc::new(Cell::new(1 as usize));
                {
                    let setting = setting.clone();
                    app.add_setting(codevisual::Setting::I32 {
                        name: String::from("Actions per tick"),
                        min_value: 0,
                        max_value: MAX_APS as i32,
                        default_value: setting.get() as i32,
                        setter: Box::new(move |new_value| { setting.set(new_value as usize); }),
                    });
                }
                setting
            },
            point_updates: {
                let setting = Rc::new(Cell::new(false));
                {
                    let setting = setting.clone();
                    app.add_setting(codevisual::Setting::Bool {
                        name: String::from("Point updates"),
                        default_value: setting.get(),
                        setter: Box::new(move |new_value| { setting.set(new_value); }),
                    });
                }
                setting
            },
            next_action: 0.0,
            draw_count: {
                let setting = Rc::new(Cell::new(50 as usize));
                {
                    let setting = setting.clone();
                    app.add_setting(codevisual::Setting::I32 {
                        name: String::from("Count"),
                        min_value: 0,
                        max_value: MAX_COUNT as i32,
                        default_value: setting.get() as i32,
                        setter: Box::new(move |new_value| {
                            println!("Drawing {} instances", new_value);
                            setting.set(new_value as usize);
                        }),
                    });
                }
                setting
            },
            cars,
            helis,
            screen_used_texture: None,
            screen_used_shader: codevisual::Shader::compile::<::ShaderLib>(
                context,
                &defines!(HELI: false),
                include_str!("screen_used.glsl"),
            ),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.current_time += delta_time;
        self.next_action -= delta_time;
        self.cars.count = self.draw_count.get();
        self.helis.count = self.draw_count.get();
        self.cars.current_time = self.current_time;
        self.helis.current_time = self.current_time;
        while self.next_action < 0.0 {
            self.next_action += TICK_TIME;
            for units in &mut [&mut self.cars, &mut self.helis] {
                if self.actions_per_tick.get() == MAX_APS {
                    units.update(None, self.point_updates.get());
                } else {
                    units.update(
                        Some(self.actions_per_tick.get() as f64 / MAX_APS as f64),
                        self.point_updates.get(),
                    );
                }
            }
        }
    }

    pub fn get_screen_used_texture<U: ugli::UniformStorage>(
        &mut self,
        uniforms: &U,
    ) -> &ugli::Texture2d {
        let context = self.app.get_window().ugli_context();
        let need_size = {
            let nearest = |n| {
                let mut x = 1;
                while x * 2 <= n {
                    x *= 2;
                }
                x
            };
            let need_size = self.app.get_window().get_size() / 5;
            vec2(nearest(need_size.x), nearest(need_size.y))
        };
        if match self.screen_used_texture {
            Some(ref texture) => texture.get_size() != need_size,
            None => true,
        }
        {
            self.screen_used_texture = Some(ugli::Texture2d::new(context, need_size));
        }
        {
            let texture = self.screen_used_texture.as_mut().unwrap();
            let mut framebuffer = ugli::Framebuffer::new_color(context, texture);
            ugli::clear(&mut framebuffer, Some(Color::rgb(1.0, 1.0, 1.0)), None);
            ugli::draw(
                &mut framebuffer,
                self.screen_used_shader.ugli_program(),
                ugli::DrawMode::TriangleFan,
                &ugli::instanced(
                    &ugli::quad(context).slice(..),
                    &self.cars.instances.slice(..self.draw_count.get()),
                ),
                uniforms,
                &ugli::DrawParameters {
                    blend_mode: ugli::BlendMode::Alpha,
                    depth_test: ugli::DepthTest::Off,
                    ..Default::default()
                },
            );
        }
        self.screen_used_texture.as_ref().unwrap()
    }

    pub fn draw<U: ugli::UniformStorage>(
        &mut self,
        framebuffer: &mut ugli::DefaultFramebuffer,
        uniforms: &U,
    ) {
        self.cars.draw(framebuffer, uniforms);
        self.helis.draw(framebuffer, uniforms);
    }
}
