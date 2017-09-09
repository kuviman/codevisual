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
        }
    }

    pub fn draw<U: ugli::UniformStorage>(&mut self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
        let texture = {
            let mut texture = ugli::Texture2d::new_uninitialized(self.app.ugli_context(), framebuffer.get_size());
            {
                let mut framebuffer = ugli::Framebuffer::new_color(self.app.ugli_context(), ugli::ColorAttachment::Texture(&mut texture));
                ugli::clear(&mut framebuffer, Some(Color::rgba(0.0, 0.0, 0.0, 0.0)), None);
                ugli::draw(
                    &mut framebuffer,
                    &self.material.ugli_program(),
                    ugli::DrawMode::Triangles,
                    &ugli::instanced(&self.geometry.slice(..), &self.instances.slice(..)),
                    uniforms,
                    &ugli::DrawParameters {
                        depth_test: ugli::DepthTest::On,
                        blend_mode: ugli::BlendMode::Alpha,
                        cull_face: ugli::CullFace::Front,
                        ..Default::default()
                    }
                );
            }
            texture
        };
        let material: codevisual::Material = codevisual::Material::new(
            self.app.ugli_context(), (), (), include_str!("fullscreen.glsl"));
        ugli::draw(
            framebuffer,
            &material.ugli_program(),
            ugli::DrawMode::TriangleFan,
            &ugli::plain(&ugli::quad(self.app.ugli_context()).slice(..)),
            uniforms!(texture: texture),
            &ugli::DrawParameters {
                depth_test: ugli::DepthTest::Off,
                blend_mode: ugli::BlendMode::Alpha,
                ..Default::default()
            }
        );
    }
}