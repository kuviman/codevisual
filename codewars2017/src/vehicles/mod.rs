use ::*;

#[derive(Vertex, Debug, Clone)]
struct Instance {
    i_pos: Vec2<f32>,
}

impl Instance {
    fn new() -> Self {
        Self { i_pos: vec2(1.0, 1.0) }
    }
}

pub struct Vehicles {
    instances: ugli::VertexBuffer<Instance>,
    material: codevisual::Material<ShaderLib>,
    game_log_loader: gamelog::loader::Loader,
}

const MAX_COUNT: usize = 2000;

impl Vehicles {
    pub fn new(app: &Rc<codevisual::Application>, game_log_loader: &gamelog::loader::Loader) -> Self {
        Self {
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
        }
        ugli::draw(framebuffer, &self.material.ugli_program(), ugli::DrawMode::Points,
                   &ugli::plain(&self.instances.slice(..data.len())), uniforms, &ugli::DrawParameters {
                ..Default::default()
            });
    }
}