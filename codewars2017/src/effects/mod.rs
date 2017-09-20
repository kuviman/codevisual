use ::*;

#[derive(Vertex, Clone)]
struct Vertex {
    a_v: Vec3<f32>,
}

pub struct Effects {
    app: Rc<codevisual::Application>,
    settings: Rc<Settings>,
    game_log_loader: game_log::Loader,
    geometry: ugli::VertexBuffer<Vertex>,
    material: Material,
}

const MAX_COUNT: usize = 2000;

impl Effects {
    pub fn new(app: &Rc<codevisual::Application>, settings: &Rc<Settings>, game_log_loader: &game_log::Loader) -> Self {
        Self {
            app: app.clone(),
            settings: settings.clone(),
            game_log_loader: game_log_loader.clone(),
            material: Material::new(app.ugli_context(), (), (), include_str!("shader.glsl")),
            geometry: ugli::VertexBuffer::new_dynamic(
                app.ugli_context(),
                vec![Vertex { a_v: vec3(0.0, 0.0, 0.0) }; MAX_COUNT * 2])
        }
    }
    pub fn draw<U: ugli::UniformStorage>(&mut self,
                                         framebuffer: &mut ugli::Framebuffer,
                                         uniforms: U,
                                         tick: usize) {
        let game_log = self.game_log_loader.read();
        if let Some(attacks) = game_log.effects.get_attacks(tick) {
            {
                let mut data = self.geometry.slice_mut(..attacks.len() * 2);
                let mut data = data.iter_mut();
                const DEFAULT_HEIGHT: f32 = 1.0;
                for attack in attacks {
                    let vehicle_pos = game_log.vehicles.get_pos_by_id(tick, attack.vehicle_id)
                        .extend(if game_log.vehicles.get_by_id(attack.vehicle_id).unwrap().aerial {
                            self.settings.sky_height.get() as f32
                        } else {
                            DEFAULT_HEIGHT
                        });
                    data.next().unwrap().a_v = vehicle_pos;
                    let target_pos = game_log.vehicles.get_pos_by_id(tick, attack.target_id)
                        .extend(if game_log.vehicles.get_by_id(attack.target_id).unwrap().aerial {
                            self.settings.sky_height.get() as f32
                        } else {
                            DEFAULT_HEIGHT
                        });
                    data.next().unwrap().a_v = target_pos;
                }
            }
            ugli::draw(framebuffer,
                       &self.material.ugli_program(),
                       ugli::DrawMode::Lines { line_width: 3.0 },
                       &ugli::plain(&self.geometry.slice(..attacks.len() * 2)),
                       (&uniforms, uniforms!(u_color: Color::rgba(1.0, 0.0, 0.0, 0.5))),
                       &ugli::DrawParameters {
                           depth_test: ugli::DepthTest::On,
                           blend_mode: ugli::BlendMode::Alpha,
                           ..Default::default()
                       });
        }
    }
}