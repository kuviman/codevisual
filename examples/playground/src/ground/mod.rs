use ::*;

#[derive(Vertex, Debug, Copy, Clone)]
pub struct Vertex {
    a_pos: Vec2<f32>,
}

#[derive(Uniforms)]
pub struct Uniforms {
    u_grass_texture: ugli::Texture2d,
    u_darkgrass_texture: ugli::Texture2d,
    u_dirt_texture: ugli::Texture2d,
    u_map_texture: ugli::Texture2d,
}

resources! {
    Resources {
        grass_texture: ugli::Texture2d = "assets/grass.png",
        darkgrass_texture: ugli::Texture2d = "assets/darkgrass.png",
        dirt_texture: ugli::Texture2d = "assets/dirt.png",
        map_texture: ugli::Texture2d = "assets/map.png",
    }
}

pub struct Ground {
    geometry: ugli::VertexBuffer<Vertex>,
    pub uniforms: Uniforms,
    shader: codevisual::Shader,
    water_geometry: ugli::VertexBuffer<Vertex>,
    water_shader: codevisual::Shader,
}

impl Ground {
    pub fn new(app: &codevisual::Application, resources: Resources) -> Self {
        let context = app.get_window().ugli_context();
        Ground {
            geometry: {
                let mut data = Vec::new();
                const N: usize = 64;
                for i in 0..N {
                    for j in 0..N {
                        let x1 = -MAP_SIZE + 2.0 * MAP_SIZE * i as f32 / N as f32;
                        let y1 = -MAP_SIZE + 2.0 * MAP_SIZE * j as f32 / N as f32;
                        let x2 = -MAP_SIZE + 2.0 * MAP_SIZE * (i + 1) as f32 / N as f32;
                        let y2 = -MAP_SIZE + 2.0 * MAP_SIZE * (j + 1) as f32 / N as f32;

                        data.push(Vertex { a_pos: vec2(x1, y1) });
                        data.push(Vertex { a_pos: vec2(x2, y1) });
                        data.push(Vertex { a_pos: vec2(x2, y2) });

                        data.push(Vertex { a_pos: vec2(x1, y1) });
                        data.push(Vertex { a_pos: vec2(x2, y2) });
                        data.push(Vertex { a_pos: vec2(x1, y2) });
                    }
                }
                ugli::VertexBuffer::new(context, data)
            },
            uniforms: Uniforms {
                u_dirt_texture: resources.dirt_texture,
                u_grass_texture: resources.grass_texture,
                u_darkgrass_texture: resources.darkgrass_texture,
                u_map_texture: resources.map_texture,
            },
            shader: codevisual::Shader::compile::<::ShaderLib>(context,
                                                               &(),
                                                               include_str!("shader.glsl")),
            water_geometry: ugli::VertexBuffer::new(context,
                                                    vec![Vertex {
                                                             a_pos: vec2(-MAP_SIZE, -MAP_SIZE),
                                                         },
                                                         Vertex {
                                                             a_pos: vec2(-MAP_SIZE, MAP_SIZE),
                                                         },
                                                         Vertex {
                                                             a_pos: vec2(MAP_SIZE, MAP_SIZE),
                                                         },
                                                         Vertex {
                                                             a_pos: vec2(MAP_SIZE, -MAP_SIZE),
                                                         }]),
            water_shader: codevisual::Shader::compile::<::ShaderLib>(context,
                                                                     &(),
                                                                     include_str!("water.glsl")),
        }
    }

    pub fn draw<U: ugli::UniformStorage>(&mut self,
                                         framebuffer: &mut ugli::DefaultFramebuffer,
                                         uniforms: &U) {
        ugli::draw(framebuffer,
                   self.shader.ugli_program(),
                   ugli::DrawMode::Triangles,
                   &ugli::plain(&self.geometry.slice(..)),
                   &(uniforms, &self.uniforms),
                   &Default::default());
        ugli::draw(framebuffer,
                   self.water_shader.ugli_program(),
                   ugli::DrawMode::TriangleFan,
                   &ugli::plain(&self.water_geometry.slice(..)),
                   &(uniforms, &self.uniforms),
                   &ugli::DrawParameters {
                       blend_mode: ugli::BlendMode::Alpha,
                       ..Default::default()
                   });
    }
}