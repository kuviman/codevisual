use ::*;

#[derive(Vertex, Debug, Clone)]
struct Instance {
    i_pos: Vec2<f32>,
    i_height: f32,
    i_radius: f32,
    i_color: Color,
}

impl Instance {
    fn new() -> Self {
        unsafe { std::mem::uninitialized() }
    }
}

pub struct Vehicles {
    app: Rc<codevisual::Application>,
    instances: ugli::VertexBuffer<Instance>,
    material: codevisual::Material<ShaderLib>,
    game_log_loader: game_log::Loader,
}

const MAX_COUNT: usize = 2000;

impl Vehicles {
    pub fn new(app: &Rc<codevisual::Application>, game_log_loader: &game_log::Loader) -> Self {
        Self {
            app: app.clone(),
            instances: ugli::VertexBuffer::new_dynamic(
                app.ugli_context(), vec![Instance::new(); MAX_COUNT]),
            material: codevisual::Material::new(app.ugli_context(), (), (), include_str!("shader.glsl")),
            game_log_loader: game_log_loader.clone(),
        }
    }
    pub fn draw<U: ugli::UniformStorage>(&mut self, tick: usize, framebuffer: &mut ugli::DefaultFramebuffer, uniforms: U) {
        let data = self.game_log_loader.read().vehicles.get(tick);
        for (instance, data) in self.instances.slice_mut(..data.len()).iter_mut().zip(&data) {
            instance.i_pos = vec2(data.pos.x as f32, data.pos.y as f32);
            instance.i_radius = data.radius;
            use game_log::VehicleType::*;
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
            instance.i_height = if data.aerial { 1.0 } else { 0.0 };
        }
        ugli::draw(framebuffer, &self.material.ugli_program(), ugli::DrawMode::TriangleFan,
                   &ugli::instanced(&ugli::quad(self.app.ugli_context()).slice(..),
                                    &self.instances.slice(..data.len())),
                   uniforms, &ugli::DrawParameters { ..Default::default() });
    }
}