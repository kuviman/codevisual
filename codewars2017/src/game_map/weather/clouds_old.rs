//use ::*;
//
//pub struct Clouds {
//    app: Rc<codevisual::Application>,
//    geometry: ugli::Quad,
//    material: Material,
//}
//
//impl Clouds {
//    pub fn new(app: &Rc<codevisual::Application>, game_log: &GameLog) -> Self {
//        Self {
//            app: app.clone(),
//            geometry: ugli::Quad::new(app.ugli_context(), vec2(0.0, 0.0), game_log.map_size),
//            material: Material::new(app.ugli_context(), (), (), include_str!("clouds.glsl")),
//        }
//    }
//
//    pub fn draw<U: ugli::UniformStorage>(&mut self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
//        ugli::draw(
//            framebuffer,
//            &self.material.ugli_program(),
//            ugli::DrawMode::TriangleFan,
//            &ugli::plain(&self.geometry.slice(..)),
//            uniforms,
//            &ugli::DrawParameters {
//                depth_test: ugli::DepthTest::On,
//                blend_mode: ugli::BlendMode::Alpha,
//                ..Default::default()
//            }
//        );
//    }
//}