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

#[derive(ShaderDefines, Clone, PartialEq)]
struct Defines {
    d_is_heli: bool,
    d_heightmap_enabled: bool,
}

pub struct Units {
    geometry: obj::Geometry,
    pub instances: ugli::VertexBuffer<InstanceData>,
    material: codevisual::Material<::ShaderLib, (), Defines>,
    texture: ugli::Texture2d,
    count: usize,
    pub current_time: f32,
    settings: Rc<RefCell<Settings>>,
}

impl Units {
    pub fn new(
        app: &codevisual::Application,
        settings: &Rc<RefCell<Settings>>,
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
        let context = app.ugli_context();
        Self {
            geometry,
            instances: ugli::VertexBuffer::new_dynamic(context, instance_data),
            material: codevisual::Material::new(
                context,
                (),
                Defines {
                    d_is_heli: if let UnitType::Heli = unit_type {
                        true
                    } else {
                        false
                    },
                    d_heightmap_enabled: true,
                },
                include_str!("shader.glsl"),
            ),
            texture,
            count: 0,
            current_time: 0.0,
            settings: settings.clone(),
        }
    }
    pub fn update(&mut self, percent: Option<f64>) {
        match percent {
            Some(percent) => {
                let count = (self.count as f64 * percent) as usize;
                let indices = rand::sample(&mut thread_rng(), 0..self.count, count);
                let mut data = &mut self.instances[..self.count];
                for &i in &indices {
                    data[i].update(self.current_time);
                }
            }
            None => {
                let data = &mut self.instances[..self.count];
                for unit in data.iter_mut() {
                    unit.update(self.current_time);
                }
            }
        };
    }
    pub fn draw<U: ugli::Uniforms>(
        &mut self,
        framebuffer: &mut ugli::Framebuffer,
        uniforms: &U,
    ) {
        self.material.defines.d_heightmap_enabled = self.settings.borrow().heightmap_enabled;
        ugli::draw(
            framebuffer,
            &self.material.ugli_program(),
            ugli::DrawMode::Triangles,
            ugli::instanced(
                &self.geometry,
                self.instances.slice(..self.count),
            ),
            &(uniforms, uniforms!(u_texture: &self.texture)),
            ugli::DrawParameters::default(),
        );
    }
}

#[derive(Resources)]
pub struct Resources {
    #[path = "assets/car.png"]
    car_texture: ugli::Texture2d,
    #[path = "assets/heli.png"]
    heli_texture: ugli::Texture2d,
    #[path = "assets/car.obj"]
    car_obj: String,
    #[path = "assets/heli.obj"]
    heli_obj: String,
}

#[derive(Vertex)]
struct QuadVertex {
    a_v: Vec2<f32>,
}

pub struct AllUnits {
    app: Rc<codevisual::Application>,
    current_time: f32,
    next_action: f32,
    pub cars: Units,
    pub helis: Units,
    screen_used_texture: Option<ugli::Texture2d>,
    screen_used_material: codevisual::Material<::ShaderLib, (), Defines>,
    settings: Rc<RefCell<Settings>>,
    quad: ugli::VertexBuffer<QuadVertex>,
}

impl AllUnits {
    pub fn new(
        app: &Rc<codevisual::Application>,
        resources: Resources,
        settings: &Rc<RefCell<Settings>>,
    ) -> Self {
        let context = app.ugli_context();
        let cars = Units::new(
            app,
            settings,
            UnitType::Car,
            obj::parse(app, &resources.car_obj),
            resources.car_texture,
        );
        let helis = Units::new(
            app,
            settings,
            UnitType::Heli,
            obj::parse(app, &resources.heli_obj),
            resources.heli_texture,
        );
        Self {
            app: app.clone(),
            current_time: 0.0,
            next_action: 0.0,
            cars,
            helis,
            screen_used_texture: None,
            screen_used_material: codevisual::Material::new(
                context,
                (),
                Defines {
                    d_is_heli: false,
                    d_heightmap_enabled: true,
                },
                include_str!("screen_used.glsl"),
            ),
            settings: settings.clone(),
            quad: ugli::VertexBuffer::new_static(app.ugli_context(), vec![
                QuadVertex { a_v: vec2(-1.0, -1.0) },
                QuadVertex { a_v: vec2(-1.0, 1.0) },
                QuadVertex { a_v: vec2(1.0, 1.0) },
                QuadVertex { a_v: vec2(1.0, -1.0) }, ]),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.current_time += delta_time;
        self.next_action -= delta_time;
        self.cars.count = self.settings.borrow().draw_count;
        self.helis.count = self.settings.borrow().draw_count;
        self.cars.current_time = self.current_time;
        self.helis.current_time = self.current_time;
        while self.next_action < 0.0 {
            self.next_action += TICK_TIME;
            for units in &mut [&mut self.cars, &mut self.helis] {
                if self.settings.borrow().actions_per_tick >= 1.0 - 1e-6 {
                    units.update(None);
                } else {
                    units.update(Some(self.settings.borrow().actions_per_tick));
                }
            }
        }
    }

    pub fn get_screen_used_texture<U: ugli::Uniforms>(
        &mut self,
        uniforms: &U,
    ) -> &ugli::Texture2d {
        self.screen_used_material.defines.d_heightmap_enabled =
            self.settings.borrow().heightmap_enabled;
        let context = self.app.ugli_context();
        let need_size = {
            let nearest = |n| {
                let mut x = 1;
                while x * 2 <= n {
                    x *= 2;
                }
                x
            };
            let need_size = self.app.window().get_size() / 5;
            vec2(nearest(need_size.x), nearest(need_size.y))
        };
        if match self.screen_used_texture {
            Some(ref texture) => texture.get_size() != need_size,
            None => true,
        }
            {
                self.screen_used_texture = Some(ugli::Texture2d::new_uninitialized(context, need_size));
            }
        {
            let texture = self.screen_used_texture.as_mut().unwrap();
            let mut framebuffer = ugli::Framebuffer::new_color(context, ugli::ColorAttachment::Texture(texture));
            ugli::clear(&mut framebuffer, Some(Color::rgb(1.0, 1.0, 1.0)), None);
            ugli::draw(
                &mut framebuffer,
                &self.screen_used_material.ugli_program(),
                ugli::DrawMode::TriangleFan,
                ugli::instanced(
                    &self.quad,
                    self.cars.instances.slice(..self.settings.borrow().draw_count),
                ),
                uniforms,
                ugli::DrawParameters {
                    blend_mode: Some(default()),
                    depth_func: None,
                    ..Default::default()
                },
            );
        }
        self.screen_used_texture.as_ref().unwrap()
    }

    pub fn draw<U: ugli::Uniforms>(
        &mut self,
        framebuffer: &mut ugli::Framebuffer,
        uniforms: &U,
    ) {
        self.cars.draw(framebuffer, uniforms);
        self.helis.draw(framebuffer, uniforms);
    }
}