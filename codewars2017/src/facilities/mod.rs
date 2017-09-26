use ::*;

resources! {
    Resources {
        control_center: obj::Model = "assets/facilities/RadarStation",
        factory: obj::Model = "assets/facilities/Factory",
    }
}

#[derive(Vertex)]
struct Instance {
    i_pos: Vec2<f32>,
}

pub struct Facilities {
    control_center_model: obj::Model,
    factory_model: obj::Model,
    control_center_instances: ugli::VertexBuffer<Instance>,
    factory_instances: ugli::VertexBuffer<Instance>,
    material: ShadowCastMaterial,
}

impl Facilities {
    pub fn new(app: &Rc<codevisual::Application>,
               resources: Resources,
               settings: &Rc<Settings>,
               game_log: &GameLog) -> Self {
        Self {
            control_center_instances: ugli::VertexBuffer::new_static(
                app.ugli_context(),
                game_log.facilities.control_centers.iter().map(|&pos| Instance { i_pos: pos }).collect()),
            factory_instances: ugli::VertexBuffer::new_static(
                app.ugli_context(),
                game_log.facilities.factories.iter().map(|&pos| Instance { i_pos: pos }).collect()),
            control_center_model: resources.control_center,
            factory_model: resources.factory,
            material: ShadowCastMaterial::new(app.ugli_context(), settings, include_str!("shader.glsl")),
        }
    }

    pub fn draw<U: ugli::UniformStorage>(&self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
        for &(ref model, ref instances) in &[
            (&self.control_center_model, &self.control_center_instances),
            (&self.factory_model, &self.factory_instances)] {
            ugli::draw(
                framebuffer,
                &self.material.ugli_program(),
                ugli::DrawMode::Triangles,
                &ugli::instanced(&model.geometry.slice(..), &instances.slice(..)),
                (&uniforms, uniforms!(u_texture: &model.texture)),
                &ugli::DrawParameters {
                    blend_mode: ugli::BlendMode::Alpha,
                    depth_test: ugli::DepthTest::On,
                    ..Default::default()
                }
            );
        }
    }

    pub fn draw_shadows<U: ugli::UniformStorage>(&self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
        for &(ref model, ref instances) in &[
            (&self.control_center_model, &self.control_center_instances),
            (&self.factory_model, &self.factory_instances)] {
            ugli::draw(
                framebuffer,
                &self.material.shadow_material.ugli_program(),
                ugli::DrawMode::Triangles,
                &ugli::instanced(&model.geometry.slice(..), &instances.slice(..)),
                (&uniforms, uniforms!(u_texture: &model.texture)),
                &ugli::DrawParameters {
                    blend_mode: ugli::BlendMode::Off,
                    depth_test: ugli::DepthTest::On,
                    ..Default::default()
                }
            );
        }
    }
}