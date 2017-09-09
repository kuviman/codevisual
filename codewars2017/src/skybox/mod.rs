use ::*;

pub struct SkyBox {
    app: Rc<codevisual::Application>,
    texture: ugli::Texture2d,
    material: Material,
}

resources! {
    Resources {
        texture: ugli::Texture2d = "assets/skybox.png",
    }
}

impl SkyBox {
    pub fn new(app: &Rc<codevisual::Application>, resources: Resources) -> Self {
        Self {
            app: app.clone(),
            texture: resources.texture,
            material: Material::new(app.ugli_context(), (), (), include_str!("shader.glsl")),
        }
    }
    pub fn draw<U: ugli::UniformStorage>(&mut self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
        ugli::draw(
            framebuffer,
            &self.material.ugli_program(),
            ugli::Cube::DRAW_MODE,
            &ugli::plain(&ugli::cube(self.app.ugli_context()).slice(..)),
            (uniforms, uniforms!(texture: &self.texture)),
            &ugli::DrawParameters {
                depth_test: ugli::DepthTest::Off,
                blend_mode: ugli::BlendMode::Off,
                ..Default::default()
            });
    }
}