use ::*;

pub struct ShadowMap {
    app: Rc<codevisual::Application>,
    map: Option<(ugli::Texture2d, ugli::DepthTexture)>,
    vehicle_material: Material,
    trees_material: Material,
}

impl ShadowMap {
    pub fn new(app: &Rc<codevisual::Application>, settings: &Rc<Settings>) -> Self {
        Self {
            app: app.clone(),
            map: None,
            vehicle_material: Material::new(
                app.ugli_context(), settings, include_str!("vehicle.glsl")),
            trees_material: Material::new(
                app.ugli_context(), settings, include_str!("trees.glsl")),
        }
    }

    pub fn prepare<'a, U: ugli::UniformStorage>(&'a mut self,
                                                vehicles: &vehicles::Vehicles,
                                                trees: &game_map::Trees,
                                                framebuffer: &mut ugli::Framebuffer,
                                                uniforms: U) -> &'a ugli::DepthTexture {
        let need_size = framebuffer.get_size();
        if self.map.as_ref().map_or(true, |map| map.0.get_size() != need_size) {
            // TODO: need only depth, but fails on MacOS
            self.map = Some((
                ugli::Texture2d::new_uninitialized(
                    self.app.ugli_context(), need_size),
                ugli::DepthTexture::new_uninitialized(
                    self.app.ugli_context(), need_size)));
        }
        let map = self.map.as_mut().unwrap();
        let (color, depth) = (&mut map.0, &mut map.1);
        {
            let mut framebuffer = ugli::Framebuffer::new(
                self.app.ugli_context(),
                ugli::ColorAttachment::Texture(color),
                ugli::DepthAttachment::Texture(depth));
            let framebuffer = &mut framebuffer;
            ugli::clear(framebuffer, None, Some(1.0));
            for vehicles in vehicles.vehicles_by_type.values() {
                ugli::draw(
                    framebuffer,
                    &self.vehicle_material.ugli_program(),
                    ugli::DrawMode::Triangles,
                    &ugli::instanced(&vehicles.model.geometry.slice(..),
                                     &vehicles.instances.slice(..vehicles.count)),
                    &uniforms,
                    &ugli::DrawParameters {
                        depth_test: ugli::DepthTest::On,
                        blend_mode: ugli::BlendMode::Off,
                        cull_face: ugli::CullFace::None,
                        ..Default::default()
                    });
            }
            ugli::draw(
                framebuffer,
                &self.trees_material.ugli_program(),
                ugli::DrawMode::Triangles,
                &ugli::instanced(&trees.model.geometry.slice(..),
                                 &trees.instances.slice(..)),
                &uniforms,
                &ugli::DrawParameters {
                    depth_test: ugli::DepthTest::On,
                    blend_mode: ugli::BlendMode::Off,
                    cull_face: ugli::CullFace::None,
                    ..Default::default()
                });
        }
        depth
    }
}