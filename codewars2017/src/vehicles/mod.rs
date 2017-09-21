use ::*;

#[derive(Vertex, Debug, Clone)]
pub struct Instance {
    i_pos: Vec2<f32>,
    i_height: f32,
    i_radius: f32,
    i_color: Color,
    i_angle: f32,
}

resources! {
    Resources {
        tank_1: obj::Model = "assets/vehicles/Tank",
        ifv_1: obj::Model = "assets/vehicles/BTR",
        arrv_1: obj::Model = "assets/vehicles/Truck",
        fighter_1: obj::Model = "assets/vehicles/Fighter",
        helicopter_1: obj::Model = "assets/vehicles/Helicopter",

        tank_2: obj::Model = "assets/vehicles/Tank",
        ifv_2: obj::Model = "assets/vehicles/BTR",
        arrv_2: obj::Model = "assets/vehicles/Truck",
        fighter_2: obj::Model = "assets/vehicles/Fighter",
        helicopter_2: obj::Model = "assets/vehicles/Helicopter",
    }
}

impl Instance {
    fn new() -> Self {
        unsafe { std::mem::uninitialized() }
    }
}

pub struct SameVehicles {
    app: Rc<codevisual::Application>,
    pub instances: ugli::VertexBuffer<Instance>,
    pub count: usize,
    material: Material,
    pub model: obj::Model,
}

impl SameVehicles {
    fn new(app: &Rc<codevisual::Application>, model: obj::Model) -> Self {
        Self {
            app: app.clone(),
            count: 0,
            model,
            instances: ugli::VertexBuffer::new_dynamic(
                app.ugli_context(), vec![Instance::new(); MAX_COUNT]),
            material: Material::new(app.ugli_context(), (), (), include_str!("shader.glsl")),
        }
    }
    pub fn draw<U: ugli::UniformStorage>(&self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
        ugli::draw(framebuffer, &self.material.ugli_program(), ugli::DrawMode::Triangles,
                   &ugli::instanced(&self.model.geometry.slice(..),
                                    &self.instances.slice(..self.count)),
                   (uniforms, uniforms!(texture: &self.model.texture)),
                   &ugli::DrawParameters {
                       ..Default::default()
                   });
    }
}

pub struct Vehicles {
    app: Rc<codevisual::Application>,
    pub vehicles_by_type: HashMap<(game_log::VehicleType, game_log::ID), SameVehicles>,
    game_log_loader: game_log::Loader,
}

const MAX_COUNT: usize = 2000;

impl Vehicles {
    pub fn new(app: &Rc<codevisual::Application>, resources: Resources, game_log_loader: &game_log::Loader) -> Self {
        Self {
            app: app.clone(),
            vehicles_by_type: {
                use game_log::VehicleType::*;
                let mut map = HashMap::new();
                map.insert((TANK, 1), SameVehicles::new(app, resources.tank_1));
                map.insert((IFV, 1), SameVehicles::new(app, resources.ifv_1));
                map.insert((ARRV, 1), SameVehicles::new(app, resources.arrv_1));
                map.insert((HELICOPTER, 1), SameVehicles::new(app, resources.helicopter_1));
                map.insert((FIGHTER, 1), SameVehicles::new(app, resources.fighter_1));
                map.insert((TANK, 2), SameVehicles::new(app, resources.tank_2));
                map.insert((IFV, 2), SameVehicles::new(app, resources.ifv_2));
                map.insert((ARRV, 2), SameVehicles::new(app, resources.arrv_2));
                map.insert((HELICOPTER, 2), SameVehicles::new(app, resources.helicopter_2));
                map.insert((FIGHTER, 2), SameVehicles::new(app, resources.fighter_2));
                map
            },
            game_log_loader: game_log_loader.clone(),
        }
    }

    pub fn update_to(&mut self, tick: usize) {
        let data = self.game_log_loader.read().vehicles.get(tick);
        for (&(typ, player_id), vehicles) in self.vehicles_by_type.iter_mut() {
            let mut instances = vehicles.instances.slice_mut(..data.len());
            let mut instances = instances.iter_mut();
            vehicles.count = 0;
            for data in &data {
                use game_log::VehicleType::*;
                if (typ, player_id) == (data.typ, data.player_id) {
                    vehicles.count += 1;
                    let mut instance = instances.next().unwrap();
                    instance.i_pos = vec2(data.pos.x as f32, data.pos.y as f32);
                    instance.i_radius = data.radius;
                    instance.i_color = match (typ, player_id) {
                        (_, 1) => Color::WHITE,

                        (TANK, 2) => Color::argb_hex(0xFF0042FF),
                        (IFV, 2) => Color::argb_hex(0xFF7EBFF1),
                        (HELICOPTER, 2) => Color::argb_hex(0xFF1CE6B9),
                        (ARRV, 2) => Color::argb_hex(0xFF686969),
                        (FIGHTER, 2) => Color::argb_hex(0xFF9290B2),

                        _ => panic!("WTF"),
                    };
                    instance.i_angle = data.angle;
                    instance.i_height = if data.aerial { 1.0 } else { 0.0 };
                }
            }
        }
    }

    pub fn draw<U: ugli::UniformStorage>(&self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
        for vehicles in self.vehicles_by_type.values() {
            vehicles.draw(framebuffer, &uniforms);
        }
    }

    pub fn get_instances(&self) -> Vec<ugli::VertexBufferSlice<Instance>> {
        self.vehicles_by_type.values().map(|vehicles| {
            let vehicles: &SameVehicles = vehicles;
            vehicles.instances.slice(..vehicles.count)
        }).collect()
    }
}