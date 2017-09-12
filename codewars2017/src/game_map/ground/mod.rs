use ::*;

#[derive(Uniforms)]
struct Uniforms {
    plain_texture: ugli::Texture2d,
    forest_texture: ugli::Texture2d,
    swamp_texture: ugli::Texture2d,
    terrain_map: ugli::Texture2d,
}

resources! {
    Resources {
        plain_texture: ugli::Texture2d = "assets/grass.png",
        forest_texture: ugli::Texture2d = "assets/darkgrass.png",
        swamp_texture: ugli::Texture2d = "assets/dirt.png",
    }
}

#[derive(Vertex)]
struct UndergroundVertex {
    a_v: Vec3<f32>,
    a_vn: Vec3<f32>,
}

pub struct Ground {
    app: Rc<codevisual::Application>,
    uniforms: Uniforms,
    geometry: ugli::Quad,
    material: Material,
    underground_material: Material,
    underground_geometry: ugli::VertexBuffer<UndergroundVertex>,
}

impl Ground {
    pub fn new(app: &Rc<codevisual::Application>, resources: Resources, game_log: &GameLog) -> Self {
        macro_rules! repeatable {
            ($tex:expr) => {{
                let mut texture = $tex;
                texture.set_wrap_mode(ugli::WrapMode::Repeat);
                texture
            }};
        }
        Self {
            app: app.clone(),
            uniforms: Uniforms {
                plain_texture: repeatable!(resources.plain_texture),
                forest_texture: repeatable!(resources.forest_texture),
                swamp_texture: repeatable!(resources.swamp_texture),
                terrain_map: {
                    let terrain_data: &Vec<Vec<game_log::TerrainType>> = &game_log.terrain;
                    let mut texture = ugli::Texture2d::new_with(
                        app.ugli_context(),
                        vec2(terrain_data.len(), terrain_data[0].len()),
                        |pos| {
                            use game_log::TerrainType::*;
                            match terrain_data[pos.x][pos.y] {
                                PLAIN => Color::rgb(1.0, 0.0, 0.0),
                                FOREST => Color::rgb(0.0, 1.0, 0.0),
                                SWAMP => Color::rgb(0.0, 0.0, 1.0),
                            }
                        });
                    // texture.set_filter(ugli::Filter::Nearest);
                    let mut result = ugli::Texture2d::new_uninitialized(
                        app.ugli_context(), texture.get_size() * 8);
                    {
                        let mut framebuffer = ugli::Framebuffer::new_color(
                            app.ugli_context(), ugli::ColorAttachment::Texture(&mut result));
                        let prepare_material = Material::new(
                            app.ugli_context(), (), (),
                            include_str!("prepare.glsl"));
                        ugli::draw(&mut framebuffer,
                                   &prepare_material.ugli_program(),
                                   ugli::DrawMode::TriangleFan,
                                   &ugli::plain(&ugli::quad(app.ugli_context()).slice(..)),
                                   uniforms!(texture: texture),
                                   &ugli::DrawParameters {
                                       depth_test: ugli::DepthTest::Off,
                                       blend_mode: ugli::BlendMode::Off,
                                       ..Default::default()
                                   });
                    }
                    result
                }
            },
            geometry: ugli::Quad::new(app.ugli_context(), vec2(0.0, 0.0), game_log.map_size),
            material: Material::new(app.ugli_context(), (), (), include_str!("ground.glsl")),

            underground_material: Material::new(app.ugli_context(), (), (), include_str!("underground.glsl")),
            underground_geometry: {
                let mut vs = Vec::new();
                {
                    const SIZE: usize = 16;
                    let mut height_map: Vec<Vec<f32>> = Vec::new();
                    for i in 0..SIZE + 1 {
                        let mut row = Vec::new();
                        for j in 0..SIZE + 1 {
                            row.push(random::<f32>() * 10.0 - 50.0);
                        }
                        height_map.push(row);
                    }
                    let mut add_v = |i: usize, j: usize, k: bool| {
                        vs.push(UndergroundVertex {
                            a_v: vec3(i as f32 / SIZE as f32 * game_log.map_size.x,
                                      j as f32 / SIZE as f32 * game_log.map_size.y,
                                      if k { height_map[i][j] } else { 0.0 }),
                            a_vn: vec3(0.0, 0.0, 0.0),
                        });
                    };
                    for i in 0..SIZE {
                        for j in vec![0, SIZE] {
                            add_v(i, j, false);
                            add_v(i, j, true);
                            add_v(i + 1, j, true);

                            add_v(i, j, false);
                            add_v(i + 1, j, true);
                            add_v(i + 1, j, false);
                        }
                    }
                    for j in 0..SIZE {
                        for i in vec![0, SIZE] {
                            add_v(i, j, false);
                            add_v(i, j, true);
                            add_v(i, j + 1, true);

                            add_v(i, j, false);
                            add_v(i, j + 1, true);
                            add_v(i, j + 1, false);
                        }
                    }
                }
                ugli::VertexBuffer::new_static(app.ugli_context(), vs)
            }
        }
    }

    pub fn draw<U: ugli::UniformStorage>(&mut self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
        ugli::draw(
            framebuffer,
            &self.material.ugli_program(),
            ugli::DrawMode::TriangleFan,
            &ugli::plain(&self.geometry.slice(..)),
            (&uniforms, &self.uniforms),
            &ugli::DrawParameters {
                depth_test: ugli::DepthTest::On,
                blend_mode: ugli::BlendMode::Off,
                ..Default::default()
            }
        );
        ugli::draw(
            framebuffer,
            &self.underground_material.ugli_program(),
            ugli::DrawMode::Triangles,
            &ugli::plain(&self.underground_geometry.slice(..)),
            (&uniforms, &self.uniforms),
            &ugli::DrawParameters {
                depth_test: ugli::DepthTest::On,
                blend_mode: ugli::BlendMode::Off,
                ..Default::default()
            }
        );
    }
}