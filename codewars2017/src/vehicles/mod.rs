use ::*;

#[derive(Vertex, Debug, Clone)]
struct Instance {
    i_pos: Vec2<f32>,
    i_height: f32,
    i_radius: f32,
    i_color: Color,
    i_angle: f32,
}

resources! {
    Resources {
        car: obj::Model = "assets/car",
        heli: obj::Model = "assets/heli",
    }
}

impl Instance {
    fn new() -> Self {
        unsafe { std::mem::uninitialized() }
    }
}

struct SameVehicles {
    app: Rc<codevisual::Application>,
    instances: ugli::VertexBuffer<Instance>,
    count: usize,
    material: Material,
    model: obj::Model,
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
    pub fn draw<U: ugli::UniformStorage>(&mut self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
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
    cars: SameVehicles,
    helis: SameVehicles,
    game_log_loader: game_log::Loader,
}

const MAX_COUNT: usize = 2000;

impl Vehicles {
    pub fn new(app: &Rc<codevisual::Application>, resources: Resources, game_log_loader: &game_log::Loader) -> Self {
        Self {
            app: app.clone(),
            cars: SameVehicles::new(app, resources.car),
            helis: SameVehicles::new(app, resources.heli),
            game_log_loader: game_log_loader.clone(),
        }
    }
    pub fn draw<U: ugli::UniformStorage>(&mut self, tick: usize, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
        let counts = {
            let data = self.game_log_loader.read().vehicles.get(tick);

            let mut car_instances = self.cars.instances.slice_mut(..data.len());
            let mut car_instances = car_instances.iter_mut().enumerate();
            let mut heli_instances = self.helis.instances.slice_mut(..data.len());
            let mut heli_instances = heli_instances.iter_mut().enumerate();

            for data in &data {
                use game_log::VehicleType::*;
                let instance = match data.typ {
                    TANK | IFV | ARRV => car_instances.next().unwrap().1,
                    HELICOPTER | FIGHTER => heli_instances.next().unwrap().1,
                };
                instance.i_pos = vec2(data.pos.x as f32, data.pos.y as f32);
                instance.i_radius = data.radius;
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
                instance.i_angle = data.angle;
                instance.i_height = if data.aerial { 1.0 } else { 0.0 };
            }

            (car_instances.next().unwrap().0, heli_instances.next().unwrap().0)
        };
        self.cars.count = counts.0;
        self.helis.count = counts.1;

        self.cars.draw(framebuffer, &uniforms);
        self.helis.draw(framebuffer, &uniforms);
    }
}