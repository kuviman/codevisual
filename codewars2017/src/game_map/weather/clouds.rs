use ::*;

#[derive(Vertex)]
struct Vertex {
    i_pos: Vec3<f32>,
    i_type: f32,
    i_size: f32,
}

pub struct Clouds {
    app: Rc<codevisual::Application>,
    geometry: ugli::Cube,
    instances: ugli::VertexBuffer<Vertex>,
    material: Material,
    fs_material: Material,
    alpha: codevisual::SettingValue<f64>,
    tmp: Option<(ugli::Texture2d, ugli::Renderbuffer<ugli::DepthComponent>)>,
}

impl Clouds {
    pub fn new(app: &Rc<codevisual::Application>, game_log: &GameLog) -> Self {
        Self {
            app: app.clone(),
            geometry: {
                const SIZE: f32 = 1.7;
                const DEPTH: f32 = 0.5;
                ugli::Cube::new(app.ugli_context(), vec3(-SIZE, -SIZE, -DEPTH), vec3(SIZE, SIZE, DEPTH))
            },
            instances: {
                let mut vs: Vec<Vertex> = Vec::new();
                for i in 0..game_log.weather.len() {
                    for j in 0..game_log.weather[i].len() {
                        if game_log.weather[i][j] != game_log::WeatherType::CLEAR {
                            for _ in 0..1 {
                                vs.push(Vertex {
                                    i_pos: vec3(
                                        i as f32 * 32.0 + 8.0 + random::<f32>() * 16.0,
                                        j as f32 * 32.0 + 8.0 + random::<f32>() * 16.0,
                                        random::<f32>() * 10.0),
                                    i_type: match game_log.weather[i][j] {
                                        game_log::WeatherType::RAIN => 1.0,
                                        game_log::WeatherType::CLOUD => 0.0,
                                        _ => panic!("WTF"),
                                    },
                                    i_size: random::<f32>() * 5.0 + 4.0,
                                });
                            }
                        }
                    }
                }
                ugli::VertexBuffer::new_static(app.ugli_context(), vs)
            },
            material: Material::new(app.ugli_context(), (), (), include_str!("clouds.glsl")),
            fs_material: Material::new(app.ugli_context(), (), (), include_str!("fullscreen.glsl")),
            alpha: app.add_setting_f64("Clouds opacity", 0.0, 1.0, 0.3),
            tmp: None,
        }
    }

    pub fn draw<U: ugli::UniformStorage>(&mut self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
        if self.tmp.as_ref().map_or(true, |&(ref texture, _)| texture.get_size() != framebuffer.get_size()) {
            self.tmp = Some((
                ugli::Texture2d::new_uninitialized(self.app.ugli_context(), framebuffer.get_size()),
                ugli::Renderbuffer::<ugli::DepthComponent>::new(framebuffer.get_size())
            ))
        }
        {
            let tmp = self.tmp.as_mut().unwrap();
            let mut framebuffer = ugli::Framebuffer::new(
                self.app.ugli_context(),
                ugli::ColorAttachment::Texture(&mut tmp.0),
                ugli::DepthAttachment::Renderbuffer(&mut tmp.1));
            ugli::clear(&mut framebuffer, Some(Color::rgba(0.0, 0.0, 0.0, 0.0)), Some(1.0));
            ugli::draw(
                &mut framebuffer,
                &self.material.ugli_program(),
                ugli::DrawMode::Triangles,
                &ugli::instanced(&self.geometry.slice(..), &self.instances.slice(..)),
                uniforms,
                &ugli::DrawParameters {
                    depth_test: ugli::DepthTest::On,
                    blend_mode: ugli::BlendMode::Off,
                    cull_face: ugli::CullFace::Front,
                    ..Default::default()
                }
            );
        }
        let texture = &self.tmp.as_ref().unwrap().0;
        ugli::draw(
            framebuffer,
            &self.fs_material.ugli_program(),
            ugli::DrawMode::TriangleFan,
            &ugli::plain(&ugli::quad(self.app.ugli_context()).slice(..)),
            uniforms! {
                texture: texture,
                alpha: self.alpha.get() as f32,
            },
            &ugli::DrawParameters {
                depth_test: ugli::DepthTest::Off,
                blend_mode: ugli::BlendMode::Alpha,
                ..Default::default()
            }
        );
    }
}