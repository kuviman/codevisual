use ::*;

#[derive(Vertex)]
struct RainVertex {
    a_v: Vec3<f32>
}

pub struct Rain {
    material: Material,
    geometry: ugli::VertexBuffer<RainVertex>,
}

impl Rain {
    pub fn new(app: &Rc<codevisual::Application>, game_log: &GameLog) -> Self {
        Self {
            material: Material::new(app.ugli_context(), (), (), include_str!("rain.glsl")),
            geometry: ugli::VertexBuffer::new_static(app.ugli_context(), {
                let mut vs = Vec::new();
                let weather_data: &Vec<Vec<game_log::WeatherType>> = &game_log.weather;
                for i in 0..weather_data.len() as usize {
                    for j in 0..weather_data[0].len() as usize {
                        if let game_log::WeatherType::RAIN = weather_data[i][j] {
                            const CELL_SIZE: f32 = 32.0; // TODO

                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE, j as f32 * CELL_SIZE, 0.0) });
                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE + CELL_SIZE, j as f32 * CELL_SIZE + CELL_SIZE, 0.0) });
                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE + CELL_SIZE, j as f32 * CELL_SIZE + CELL_SIZE, 1.0) });

                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE, j as f32 * CELL_SIZE, 0.0) });
                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE + CELL_SIZE, j as f32 * CELL_SIZE + CELL_SIZE, 1.0) });
                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE, j as f32 * CELL_SIZE, 1.0) });

                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE + CELL_SIZE, j as f32 * CELL_SIZE, 0.0) });
                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE, j as f32 * CELL_SIZE + CELL_SIZE, 0.0) });
                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE, j as f32 * CELL_SIZE + CELL_SIZE, 1.0) });

                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE + CELL_SIZE, j as f32 * CELL_SIZE, 0.0) });
                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE, j as f32 * CELL_SIZE + CELL_SIZE, 1.0) });
                            vs.push(RainVertex { a_v: vec3(i as f32 * CELL_SIZE + CELL_SIZE, j as f32 * CELL_SIZE, 1.0) });
                        }
                    }
                }
                vs
            }),
        }
    }

    pub fn draw<U: ugli::UniformStorage>(&mut self, framebuffer: &mut ugli::DefaultFramebuffer, uniforms: U) {
        ugli::draw(
            framebuffer,
            &self.material.ugli_program(),
            ugli::DrawMode::Triangles,
            &ugli::plain(&self.geometry.slice(..)),
            &uniforms,
            &ugli::DrawParameters {
                depth_test: ugli::DepthTest::Off,
                blend_mode: ugli::BlendMode::Alpha,
                ..Default::default()
            }
        );
    }
}