use ::*;

pub struct Minimap {
    material: codevisual::Material<::ShaderLib, (), ()>,
    settings: Rc<Settings>,
}

impl Minimap {
    pub fn new(app: &codevisual::Application, settings: &Rc<Settings>) -> Self {
        Self {
            material: codevisual::Material::new(app.ugli_context(), (), (), include_str!("shader.glsl")),
            settings: settings.clone(),
        }
    }
    pub fn render<U>(&self, framebuffer: &mut ugli::Framebuffer, units: &units::AllUnits, uniforms: &U) where U: ugli::UniformStorage {
        let h = framebuffer.get_size().y;
        let conv = |x| {
            x * h / 480
        };
        let draw_parameters = ugli::DrawParameters {
            depth_test: ugli::DepthTest::Off,
            blend_mode: ugli::BlendMode::Alpha,
            viewport: Some((conv(10), conv(10), conv(100), conv(100))),
            ..Default::default()
        };
        for &(instances, color) in [(&units.cars.instances, Color::rgb(0.0, 0.0, 1.0)), (&units.helis.instances, Color::rgb(1.0, 0.0, 0.0))].into_iter() {
            ugli::draw(
                framebuffer,
                &self.material.ugli_program(),
                ugli::DrawMode::Points,
                &ugli::plain(&instances.slice(..self.settings.draw_count.get())),
                &(uniforms, uniforms! {
                    color: color,
                    point_size: conv(2) as f32,
                }),
                &draw_parameters,
            );
        }
    }
}
