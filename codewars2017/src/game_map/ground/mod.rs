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

pub struct Ground {
    app: Rc<codevisual::Application>,
    uniforms: Uniforms,
    geometry: ugli::Quad,
    material: Material,
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
                    //                    texture = blur::gauss(app.ugli_context(), &texture);
                    texture.set_filter(ugli::Filter::Nearest);
                    texture
                }
            },
            geometry: ugli::Quad::new(app.ugli_context(), vec2(0.0, 0.0), game_log.map_size),
            material: Material::new(app.ugli_context(), (), (), include_str!("ground.glsl")),
        }
    }

    pub fn draw<U: ugli::UniformStorage>(&mut self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
        ugli::draw(
            framebuffer,
            &self.material.ugli_program(),
            ugli::DrawMode::TriangleFan,
            &ugli::plain(&self.geometry.slice(..)),
            (uniforms, &self.uniforms),
            &ugli::DrawParameters {
                depth_test: ugli::DepthTest::On,
                blend_mode: ugli::BlendMode::Off,
                ..Default::default()
            }
        );
    }
}