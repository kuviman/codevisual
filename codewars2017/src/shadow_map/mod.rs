use ::*;

pub struct ShadowMap {
    app: Rc<codevisual::Application>,
    map: Option<ugli::DepthTexture>,
    vehicle_material: Material,
}

impl ShadowMap {
    pub fn new(app: &Rc<codevisual::Application>, settings: &Rc<Settings>) -> Self {
        Self {
            app: app.clone(),
            map: None,
            vehicle_material: Material::new(
                app.ugli_context(), settings, include_str!("vehicle.glsl")),
        }
    }

    pub fn prepare<'a, U: ugli::UniformStorage>(&'a mut self,
                                                vehicles: &vehicles::Vehicles,
                                                framebuffer: &mut ugli::Framebuffer,
                                                uniforms: U) -> &'a ugli::DepthTexture {
        let need_size = framebuffer.get_size() * 2;
        if self.map.as_ref().map_or(true, |map| map.get_size() != need_size) {
            let mut map = ugli::DepthTexture::new_uninitialized(
                self.app.ugli_context(), need_size);
            map.set_filter(ugli::Filter::Nearest);
            self.map = Some(map);
        }
        let texture = self.map.as_mut().unwrap();
        {
            let mut framebuffer = ugli::Framebuffer::new(
                self.app.ugli_context(),
                ugli::ColorAttachment::None,
                ugli::DepthAttachment::Texture(texture));
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
        }
        texture
    }
}