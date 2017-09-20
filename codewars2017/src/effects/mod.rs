use ::*;

#[derive(Vertex, Clone)]
struct Vertex {
    a_start_time: f32,
    a_size: f32,
    a_color: Color,
    a_v: Vec3<f32>,
}

pub struct Effects {
    app: Rc<codevisual::Application>,
    settings: Rc<Settings>,
    game_log_loader: game_log::Loader,
    geometry: ugli::VertexBuffer<Vertex>,
    material: Material,
    particle_material: Material,
    particle_texture: ugli::Texture2d,
}

resources! {
    Resources {
        particle_texture: ugli::Texture2d = "assets/particle.png",
    }
}

const MAX_COUNT: usize = 2000;

impl Effects {
    pub fn new(app: &Rc<codevisual::Application>, resources: Resources, settings: &Rc<Settings>, game_log_loader: &game_log::Loader) -> Self {
        Self {
            app: app.clone(),
            settings: settings.clone(),
            particle_texture: resources.particle_texture,
            game_log_loader: game_log_loader.clone(),
            material: Material::new(app.ugli_context(), (), (), include_str!("shader.glsl")),
            particle_material: Material::new(app.ugli_context(), (), (), include_str!("particle.glsl")),
            geometry: ugli::VertexBuffer::new_dynamic(
                app.ugli_context(),
                vec![Vertex {
                    a_size: 0.0,
                    a_color: Color::BLACK,
                    a_start_time: 0.0,
                    a_v: vec3(0.0, 0.0, 0.0)
                }; MAX_COUNT * 2])
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
                    let vertex = data.next().unwrap();
                    vertex.a_size = 2.0;
                    vertex.a_start_time = attack.start_tick as f32 / 60.0;
                    vertex.a_color = Color::rgba(0.2, 0.2, 0.2, 0.7);
                    vertex.a_v = vehicle_pos;
                    let target_pos = game_log.vehicles.get_pos_by_id(tick, attack.target_id)
                        .extend(if game_log.vehicles.get_by_id(attack.target_id).unwrap().aerial {
                            self.settings.sky_height.get() as f32
                        } else {
                            DEFAULT_HEIGHT
                        });
                    let vertex = data.next().unwrap();
                    vertex.a_size = 5.0;
                    vertex.a_color = Color::rgba(1.0, 0.7, 0.0, 0.9);
                    vertex.a_start_time = attack.start_tick as f32 / 60.0;
                    vertex.a_v = target_pos;
                }
            }
            if self.settings.draw_attack_rays.get() {
                ugli::draw(framebuffer,
                           &self.material.ugli_program(),
                           ugli::DrawMode::Lines { line_width: 3.0 },
                           &ugli::plain(&self.geometry.slice(..attacks.len() * 2)),
                           &uniforms,
                           &ugli::DrawParameters {
                               depth_test: ugli::DepthTest::On,
                               blend_mode: ugli::BlendMode::Alpha,
                               ..Default::default()
                           });
            }
            ugli::draw(framebuffer,
                       &self.particle_material.ugli_program(),
                       ugli::DrawMode::Points,
                       &ugli::plain(&self.geometry.slice(..attacks.len() * 2)),
                       (&uniforms, uniforms!(u_texture: &self.particle_texture)),
                       &ugli::DrawParameters {
                           depth_test: ugli::DepthTest::On,
                           blend_mode: ugli::BlendMode::Alpha,
                           ..Default::default()
                       });
        }
    }
}