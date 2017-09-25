use ::*;

resources! {
    Resources {
        model: obj::Model = "assets/trees/Tree",
        green_texture: ugli::Texture2d = "assets/trees/TreeGreen.png",
        red_texture: ugli::Texture2d = "assets/trees/TreeRed.png",
        yellow_texture: ugli::Texture2d = "assets/trees/TreeYellow.png",
    }
}

#[derive(Vertex)]
pub struct Instance {
    i_pos: Vec2<f32>,
    i_size: f32,
    i_rotation: f32,
}

pub struct Trees {
    app: Rc<codevisual::Application>,
    pub geometry: ugli::VertexBuffer<obj::VertexData>,
    pub instances_with_textures: Vec<(ugli::Texture2d, ugli::VertexBuffer<Instance>)>,
    material: Material,
}

impl Trees {
    pub fn new(app: &Rc<codevisual::Application>, resources: Resources, game_log: &GameLog, settings: &Rc<Settings>) -> Self {
        let mut instances_with_textures = Vec::new();
        for (count, texture) in vec![(100, resources.model.texture),
                                     (100, resources.green_texture),
                                     (30, resources.red_texture),
                                     (30, resources.yellow_texture)] {
            let mut instances = Vec::new();
            for _ in 0..50 * count {
                let pos = vec2(random::<f32>(), random::<f32>());
                let cell_x = min((pos.x * game_log.terrain.len() as f32) as usize, game_log.terrain.len() - 1);
                let cell_y = min((pos.y * game_log.terrain[0].len() as f32) as usize, game_log.terrain[0].len() - 1);
                if game_log.terrain[cell_x][cell_y] == game_log::TerrainType::FOREST {
                    instances.push(Instance {
                        i_pos: vec2(pos.x * game_log.map_size.x, pos.y * game_log.map_size.y),
                        i_rotation: random::<f32>() * 2.0 * std::f32::consts::PI,
                        i_size: random::<f32>() * 0.03 + 0.08,
                    });
                }
            }
            instances_with_textures.push((texture, ugli::VertexBuffer::new_static(app.ugli_context(), instances)));
        }
        Self {
            app: app.clone(),
            geometry: resources.model.geometry,
            material: Material::new(app.ugli_context(), settings, include_str!("shader.glsl")),
            instances_with_textures,
        }
    }

    pub fn draw<U: ugli::UniformStorage>(&mut self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
        for &(ref texture, ref instances) in &self.instances_with_textures {
            ugli::draw(
                framebuffer,
                &self.material.ugli_program(),
                ugli::DrawMode::Triangles,
                &ugli::instanced(&self.geometry.slice(..), &instances.slice(..)),
                (&uniforms, uniforms!(texture: texture)),
                &ugli::DrawParameters {
                    depth_test: ugli::DepthTest::On,
                    blend_mode: ugli::BlendMode::Off,
                    ..Default::default()
                }
            );
        }
    }
}