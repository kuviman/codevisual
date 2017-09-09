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

#[derive(Defines, Clone, PartialEq)]
struct Defines {
    d_fog_enabled: bool,
    d_heightmap_enabled: bool,
}

pub struct Ground {
    geometry: ugli::VertexBuffer<Vertex>,
    pub uniforms: Uniforms,
    material: codevisual::Material<::ShaderLib, (), Defines>,
    water_geometry: ugli::VertexBuffer<Vertex>,
    water_material: codevisual::Material<::ShaderLib, (), Defines>,
    settings: Rc<Settings>,
}

fn repeatable(mut texture: ugli::Texture2d) -> ugli::Texture2d {
    texture.set_wrap_mode(ugli::WrapMode::Repeat);
    texture
}

impl Ground {
    pub fn new(
        app: &codevisual::Application,
        resources: Resources,
        settings: &Rc<Settings>,
    ) -> Self {
        let context = app.ugli_context();
        let defines = Defines {
            d_fog_enabled: settings.fog_enabled.get(),
            d_heightmap_enabled: settings.heightmap_enabled.get(),
        };
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
                ugli::VertexBuffer::new_static(context, data)
            },
            uniforms: Uniforms {
                u_dirt_texture: repeatable(resources.dirt_texture),
                u_grass_texture: repeatable(resources.grass_texture),
                u_darkgrass_texture: repeatable(resources.darkgrass_texture),
                u_map_texture: resources.map_texture,
            },
            material: codevisual::Material::new(
                context,
                (),
                defines.clone(),
                include_str!("shader.glsl"),
            ),
            water_geometry: ugli::VertexBuffer::new_static(
                context,
                vec![
                    Vertex { a_pos: vec2(-MAP_SIZE, -MAP_SIZE) },
                    Vertex { a_pos: vec2(-MAP_SIZE, MAP_SIZE) },
                    Vertex { a_pos: vec2(MAP_SIZE, MAP_SIZE) },
                    Vertex { a_pos: vec2(MAP_SIZE, -MAP_SIZE) },
                ],
            ),
            water_material: codevisual::Material::new(
                context,
                (),
                defines.clone(),
                include_str!("water.glsl"),
            ),
            settings: settings.clone(),
        }
    }

    pub fn draw<U: ugli::UniformStorage>(
        &mut self,
        framebuffer: &mut ugli::Framebuffer,
        uniforms: &U,
    ) {
        self.material.defines.d_fog_enabled = self.settings.fog_enabled.get();
        self.material.defines.d_heightmap_enabled = self.settings.heightmap_enabled.get();
        self.water_material.defines.d_fog_enabled = self.settings.fog_enabled.get();
        self.water_material.defines.d_heightmap_enabled = self.settings.heightmap_enabled.get();
        ugli::draw(
            framebuffer,
            &self.material.ugli_program(),
            ugli::DrawMode::Triangles,
            &ugli::plain(&self.geometry.slice(..)),
            &(uniforms, &self.uniforms),
            &Default::default(),
        );
        ugli::draw(
            framebuffer,
            &self.water_material.ugli_program(),
            ugli::DrawMode::TriangleFan,
            &ugli::plain(&self.water_geometry.slice(..)),
            &(uniforms, &self.uniforms),
            &ugli::DrawParameters {
                blend_mode: ugli::BlendMode::Alpha,
                ..Default::default()
            },
        );
    }
}
