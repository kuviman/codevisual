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

#[derive(Resources)]
pub struct Resources {
    #[path = "assets/grass.png"]
    grass_texture: ugli::Texture2d,
    #[path = "assets/darkgrass.png"]
    darkgrass_texture: ugli::Texture2d,
    #[path = "assets/dirt.png"]
    dirt_texture: ugli::Texture2d,
    #[path = "assets/map.png"]
    pub map_texture: ugli::Texture2d,
}

#[derive(ShaderDefines, Clone, PartialEq)]
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
    settings: Rc<RefCell<Settings>>,
}

fn repeatable(mut texture: ugli::Texture2d) -> ugli::Texture2d {
    texture.set_wrap_mode(ugli::WrapMode::Repeat);
    texture
}

impl Ground {
    pub fn new(
        app: &codevisual::Application,
        resources: Resources,
        settings: &Rc<RefCell<Settings>>,
    ) -> Self {
        let context = app.ugli_context();
        let defines = Defines {
            d_fog_enabled: settings.borrow().fog_enabled,
            d_heightmap_enabled: settings.borrow().heightmap_enabled,
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

    pub fn draw<U: ugli::Uniforms>(
        &mut self,
        framebuffer: &mut ugli::Framebuffer,
        uniforms: &U,
    ) {
        self.material.defines.d_fog_enabled = self.settings.borrow().fog_enabled;
        self.material.defines.d_heightmap_enabled = self.settings.borrow().heightmap_enabled;
        self.water_material.defines.d_fog_enabled = self.settings.borrow().fog_enabled;
        self.water_material.defines.d_heightmap_enabled = self.settings.borrow().heightmap_enabled;
        ugli::draw(
            framebuffer,
            &self.material.ugli_program(),
            ugli::DrawMode::Triangles,
            &self.geometry,
            &(uniforms, &self.uniforms),
            ugli::DrawParameters::default(),
        );
        ugli::draw(
            framebuffer,
            &self.water_material.ugli_program(),
            ugli::DrawMode::TriangleFan,
            &self.water_geometry,
            &(uniforms, &self.uniforms),
            ugli::DrawParameters {
                blend_mode: Some(default()),
                ..Default::default()
            },
        );
    }
}
