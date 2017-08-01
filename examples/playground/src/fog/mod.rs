use ::*;

#[derive(Vertex)]
struct QuadVertex {
    a_v: Vec2<f32>,
}

#[derive(Uniforms)]
pub struct Uniforms {
    u_fog_map: ugli::Texture2d, // TODO: need only one color component
}

pub struct Fog {
    app: Rc<codevisual::Application>,
    quad: ugli::VertexBuffer<QuadVertex>,
    shader: codevisual::Shader,
    pub uniforms: Uniforms,
    settings: Rc<Settings>,
}

impl Fog {
    pub fn new(app: &Rc<codevisual::Application>, settings: &Rc<Settings>) -> Self {
        let context = app.get_window().ugli_context();

        Self {
            app: app.clone(),
            quad: ugli::VertexBuffer::new(
                context,
                vec![
                    QuadVertex { a_v: vec2(-1.0, -1.0) },
                    QuadVertex { a_v: vec2(1.0, -1.0) },
                    QuadVertex { a_v: vec2(1.0, 1.0) },
                    QuadVertex { a_v: vec2(-1.0, 1.0) },
                ],
            ),
            shader: codevisual::Shader::compile::<::ShaderLib>(
                context,
                &(),
                include_str!("shader.glsl"),
            ),
            uniforms: Uniforms { u_fog_map: ugli::Texture2d::new(context, vec2(256, 256)) },
            settings: settings.clone(),
        }
    }
    pub fn prepare<U: ugli::UniformStorage>(&mut self, units: &units::AllUnits, uniforms: &U) {
        let context = self.app.get_window().ugli_context();
        let mut framebuffer = ugli::Framebuffer::new_color(context, &mut self.uniforms.u_fog_map);
        ugli::clear(&mut framebuffer, Some(Color::rgb(0.0, 0.0, 0.0)), None);
        for instances in &[&units.cars.instances, &units.helis.instances] {
            ugli::draw(
                &mut framebuffer,
                self.shader.ugli_program(),
                ugli::DrawMode::TriangleFan,
                &ugli::instanced(
                    &self.quad.slice(..),
                    &instances.slice(..self.settings.draw_count.get()),
                ),
                uniforms,
                &ugli::DrawParameters {
                    blend_mode: ugli::BlendMode::Alpha,
                    depth_test: ugli::DepthTest::Off,
                    ..Default::default()
                },
            );
        }
    }
}
