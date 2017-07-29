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
    enabled: Rc<Cell<bool>>,
    quad: ugli::VertexBuffer<QuadVertex>,
    shader: codevisual::Shader,
    pub uniforms: Uniforms,
}

impl Fog {
    pub fn new(app: &Rc<codevisual::Application>) -> Self {
        let context = app.get_window().ugli_context();

        Self {
            app: app.clone(),
            enabled: {
                let setting = Rc::new(Cell::new(false));
                {
                    let setting = setting.clone();
                    app.add_setting(codevisual::Setting::Bool {
                        name: String::from("Fog'o'war"),
                        default_value: setting.get(),
                        setter: Box::new(move |new_value| { setting.set(new_value); }),
                    });
                }
                setting
            },
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
        }
    }
    pub fn prepare<U: ugli::UniformStorage>(&mut self, units: &units::AllUnits, uniforms: &U) {
        let context = self.app.get_window().ugli_context();
        let mut framebuffer = ugli::Framebuffer::new_color(context, &mut self.uniforms.u_fog_map);
        if self.enabled.get() {
            ugli::clear(&mut framebuffer, Some(Color::rgb(0.0, 0.0, 0.0)), None);
            for instances in &[&units.cars.instances, &units.helis.instances] {
                ugli::draw(
                    &mut framebuffer,
                    self.shader.ugli_program(),
                    ugli::DrawMode::TriangleFan,
                    &ugli::instanced(
                        &self.quad.slice(..),
                        &instances.slice(..units.draw_count.get()),
                    ),
                    uniforms,
                    &ugli::DrawParameters {
                        blend_mode: ugli::BlendMode::Alpha,
                        depth_test: ugli::DepthTest::Off,
                        ..Default::default()
                    },
                );
            }
        } else {
            ugli::clear(&mut framebuffer, Some(Color::rgb(1.0, 1.0, 1.0)), None);
        }
    }
}
