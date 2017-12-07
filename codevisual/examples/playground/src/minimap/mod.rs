use ::*;

pub struct Minimap {
    material: codevisual::Material<::ShaderLib, (), ()>,
    settings: Rc<RefCell<Settings>>,
}

impl Minimap {
    pub fn new(app: &codevisual::Application, settings: &Rc<RefCell<Settings>>) -> Self {
        Self {
            material: codevisual::Material::new(app.ugli_context(), (), (), include_str!("shader.glsl")),
            settings: settings.clone(),
        }
    }
    pub fn render<U>(&self, framebuffer: &mut ugli::Framebuffer, units: &units::AllUnits, uniforms: &U) where U: ugli::Uniforms {
        let h = framebuffer.get_size().y;
        let conv = |x| {
            x * h / 480
        };
        let draw_parameters = ugli::DrawParameters {
            depth_func: None,
            blend_mode: Some(default()),
            viewport: Some(Rect::from_corners(vec2(conv(10), conv(10)), vec2(conv(100), conv(100)))),
            ..Default::default()
        };
        for &(instances, color) in [(&units.cars.instances, Color::rgb(0.0, 0.0, 1.0)), (&units.helis.instances, Color::rgb(1.0, 0.0, 0.0))].into_iter() {
            ugli::draw(
                framebuffer,
                &self.material.ugli_program(),
                ugli::DrawMode::Points,
                instances.slice(..self.settings.borrow().draw_count),
                &(uniforms, uniforms! {
                    color: color,
                    point_size: conv(2) as f32,
                }),
                &draw_parameters,
            );
        }
    }
}