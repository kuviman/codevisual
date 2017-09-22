use ::*;

resources! {
    Resources {
        model: obj::Model = "assets/trees/Tree",
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
    pub model: obj::Model,
    pub instances: ugli::VertexBuffer<Instance>,
    material: Material,
}

impl Trees {
    pub fn new(app: &Rc<codevisual::Application>, resources: Resources, game_log: &GameLog, settings: &Rc<Settings>) -> Self {
        let mut instances = Vec::new();
        for _ in 0..100000 {
            let pos = vec2(random::<f32>(), random::<f32>());
            let cell_x = min((pos.x * game_log.terrain.len() as f32) as usize, game_log.terrain.len() - 1);
            let cell_y = min((pos.y * game_log.terrain[0].len() as f32) as usize, game_log.terrain[0].len() - 1);
            if game_log.terrain[cell_x][cell_y] == game_log::TerrainType::FOREST {
                instances.push(Instance {
                    i_pos: vec2(pos.x * game_log.map_size.x, pos.y * game_log.map_size.y),
                    i_rotation: random::<f32>() * 2.0 * std::f32::consts::PI,
                    i_size: random::<f32>() * 1e-2 + 0.05,
                });
            }
        }
        Self {
            app: app.clone(),
            model: resources.model,
            material: Material::new(app.ugli_context(), settings, include_str!("shader.glsl")),
            instances: ugli::VertexBuffer::new_static(app.ugli_context(), instances),
        }
    }

    pub fn draw<U: ugli::UniformStorage>(&mut self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
        ugli::draw(
            framebuffer,
            &self.material.ugli_program(),
            ugli::DrawMode::Triangles,
            &ugli::instanced(&self.model.geometry.slice(..), &self.instances.slice(..)),
            (uniforms, uniforms!(texture: &self.model.texture)),
            &ugli::DrawParameters {
                depth_test: ugli::DepthTest::On,
                blend_mode: ugli::BlendMode::Off,
                ..Default::default()
            }
        );
    }
}