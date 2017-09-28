use ::*;

#[derive(Vertex, Debug, Clone)]
pub struct Instance {
    i_pos: Vec2<f32>,
    i_height: f32,
    i_radius: f32,
    i_angle: f32,
    i_color: Color,
}

resources! {
    Resources {
        tank_1: obj::Model = "assets/vehicles/player_1/Tank",
        ifv_1: obj::Model = "assets/vehicles/player_1/BTR",
        arrv_1: obj::Model = "assets/vehicles/player_1/Truck",
        fighter_1: obj::Model = "assets/vehicles/player_1/Fighter",
        helicopter_1: obj::ModelParts = "assets/vehicles/player_1/Helicopter",

        tank_2: obj::Model = "assets/vehicles/player_2/Tank",
        ifv_2: obj::Model = "assets/vehicles/player_2/BTR",
        arrv_2: obj::Model = "assets/vehicles/player_2/Truck",
        fighter_2: obj::Model = "assets/vehicles/player_2/Fighter",
        helicopter_2: obj::ModelParts = "assets/vehicles/player_2/Helicopter",
    }
}

impl Instance {
    fn new() -> Self {
        unsafe { std::mem::uninitialized() }
    }
}

pub struct SameVehicles {
    app: Rc<codevisual::Application>,
    instances: ugli::VertexBuffer<Instance>,
    count: usize,
    texture: ugli::Texture2d,
    parts: Vec<(ShadowCastMaterial, ugli::VertexBuffer<obj::VertexData>)>,
}

impl SameVehicles {
    fn new(app: &Rc<codevisual::Application>,
           settings: &Rc<Settings>,
           texture: ugli::Texture2d,
           parts: Vec<(ShadowCastMaterial, ugli::VertexBuffer<obj::VertexData>)>) -> Self {
        Self {
            app: app.clone(),
            count: 0,
            instances: ugli::VertexBuffer::new_dynamic(
                app.ugli_context(), vec![Instance::new(); MAX_COUNT]),
            texture,
            parts,
        }
    }
    pub fn draw<U: ugli::UniformStorage>(&self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
        let uniforms = (uniforms, uniforms!(texture: &self.texture));
        for &(ref material, ref geometry) in &self.parts {
            ugli::draw(framebuffer, &material.ugli_program(), ugli::DrawMode::Triangles,
                       &ugli::instanced(&geometry.slice(..),
                                        &self.instances.slice(..self.count)),
                       &uniforms,
                       &ugli::DrawParameters {
                           ..Default::default()
                       });
        }
    }
    pub fn draw_shadows<U: ugli::UniformStorage>(&self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
        for &(ref material, ref geometry) in &self.parts {
            ugli::draw(
                framebuffer,
                &material.shadow_material.ugli_program(),
                ugli::DrawMode::Triangles,
                &ugli::instanced(&geometry.slice(..),
                                 &self.instances.slice(..self.count)),
                &uniforms,
                &ugli::DrawParameters {
                    depth_test: ugli::DepthTest::On,
                    blend_mode: ugli::BlendMode::Off,
                    cull_face: ugli::CullFace::None,
                    ..Default::default()
                });
        }
    }
}

pub struct Vehicles {
    app: Rc<codevisual::Application>,
    vehicles_by_type: HashMap<(game_log::VehicleType, game_log::ID), SameVehicles>,
    game_log_loader: game_log::Loader,
}

const MAX_COUNT: usize = 2000;

impl Vehicles {
    pub fn new(app: &Rc<codevisual::Application>, settings: &Rc<Settings>, resources: Resources, game_log_loader: &game_log::Loader) -> Self {
        Self {
            app: app.clone(),
            vehicles_by_type: {
                use game_log::VehicleType::*;
                let mut map = HashMap::new();
                let program_source = include_str!("shader.glsl");
                macro_rules! material {
                    ($source:expr) => {
                        ShadowCastMaterial::new(app.ugli_context(), settings, $source)
                    };
                }
                map.insert((TANK, 1), SameVehicles::new(
                    app, settings, resources.tank_1.texture,
                    vec![(material!(program_source), resources.tank_1.geometry)]));
                map.insert((ARRV, 1), SameVehicles::new(
                    app, settings, resources.arrv_1.texture,
                    vec![(material!(program_source), resources.arrv_1.geometry)]));
                map.insert((HELICOPTER, 1), SameVehicles::new(
                    app, settings, resources.helicopter_1.texture,
                    resources.helicopter_1.parts.into_iter().map(|(name, geometry)| {
                        (match name.as_str() {
                            "HelicopterScrew_Untitled.003" => material!(&format!("#define HELICOPTER\n{}", program_source)),
                            "Helicopter_Untitled.002" => material!(program_source),
                            _ => unreachable!("Unexpected obj part name: {}", name)
                        }, geometry)
                    }).collect()));
                map.insert((FIGHTER, 1), SameVehicles::new(
                    app, settings, resources.fighter_1.texture,
                    vec![(material!(program_source), resources.fighter_1.geometry)]));
                map.insert((IFV, 1), SameVehicles::new(
                    app, settings, resources.ifv_1.texture,
                    vec![(material!(program_source), resources.ifv_1.geometry)]));
                map.insert((TANK, 2), SameVehicles::new(
                    app, settings, resources.tank_2.texture,
                    vec![(material!(program_source), resources.tank_2.geometry)]));
                map.insert((ARRV, 2), SameVehicles::new(
                    app, settings, resources.arrv_2.texture,
                    vec![(material!(program_source), resources.arrv_2.geometry)]));
                map.insert((HELICOPTER, 2), SameVehicles::new(
                    app, settings, resources.helicopter_2.texture,
                    resources.helicopter_2.parts.into_iter().map(|(name, geometry)| {
                        (match name.as_str() {
                            "HelicopterScrew_B" => material!(&format!("#define HELICOPTER\n{}", program_source)),
                            "Helicopter_B" => material!(program_source),
                            _ => unreachable!("Unexpected obj part name: {}", name)
                        }, geometry)
                    }).collect()));
                map.insert((FIGHTER, 2), SameVehicles::new(
                    app, settings, resources.fighter_2.texture,
                    vec![(material!(program_source), resources.fighter_2.geometry)]));
                map.insert((IFV, 2), SameVehicles::new(
                    app, settings, resources.ifv_2.texture,
                    vec![(material!(program_source), resources.ifv_2.geometry)]));
                map
            },
            game_log_loader: game_log_loader.clone(),
        }
    }

    pub fn update_to(&mut self, tick: f32) {
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
                    instance.i_angle = data.angle;
                    instance.i_height = if data.aerial { 1.0 } else { 0.0 };
                    instance.i_color = match (data.typ, data.player_id) {
                        (TANK, 1) => Color::argb_hex(0xFFFF0303),
                        (IFV, 1) => Color::argb_hex(0xFFFEBA0E),
                        (HELICOPTER, 1) => Color::argb_hex(0xFFEDEA00),
                        (ARRV, 1) => Color::argb_hex(0xFF9E5507),
                        (FIGHTER, 1) => Color::argb_hex(0xFFFFCED1),

                        (TANK, 2) => Color::argb_hex(0xFF0042FF),
                        (IFV, 2) => Color::argb_hex(0xFF7EBFF1),
                        (HELICOPTER, 2) => Color::argb_hex(0xFF1CE6B9),
                        (ARRV, 2) => Color::argb_hex(0xFF686969),
                        (FIGHTER, 2) => Color::argb_hex(0xFF9290B2),

                        _ => panic!("WTF"),
                    };
                }
            }
        }
    }

    pub fn draw<U: ugli::UniformStorage>(&self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
        for vehicles in self.vehicles_by_type.values() {
            vehicles.draw(framebuffer, &uniforms);
        }
    }

    pub fn draw_shadows<U: ugli::UniformStorage>(&self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
        for vehicles in self.vehicles_by_type.values() {
            vehicles.draw_shadows(framebuffer, &uniforms);
        }
    }

    pub fn get_instances(&self) -> Vec<ugli::VertexBufferSlice<Instance>> {
        self.vehicles_by_type.values().map(|vehicles| {
            let vehicles: &SameVehicles = vehicles;
            vehicles.instances.slice(..vehicles.count)
        }).collect()
    }
}