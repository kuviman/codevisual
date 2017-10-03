use ::*;

#[derive(Vertex)]
struct Vertex {
    a_pos: Vec3<f32>,
    a_vt: Vec2<f32>,
}

#[derive(Vertex)]
struct Instance {
    i_pos: Vec2<f32>,
    i_size: f32,
}

#[derive(Resources)]
pub struct Resources {
    #[path = "assets/bush.png"]
    bush_texture: ugli::Texture2d,
    #[path = "assets/palm.png"]
    palm_texture: ugli::Texture2d,
}

#[derive(ShaderDefines, Clone, PartialEq)]
struct Defines {
    d_fog_enabled: bool,
    d_transparency_enabled: bool,
    d_heightmap_enabled: bool,
    d_is_palm: bool,
}

struct Decor {
    texture: ugli::Texture2d,
    material: codevisual::Material<::ShaderLib, (), Defines>,
    geometry: ugli::VertexBuffer<Vertex>,
    instances: ugli::VertexBuffer<Instance>,
    settings: Rc<RefCell<Settings>>,
}

impl Decor {
    pub fn new(
        app: &codevisual::Application,
        settings: &Rc<RefCell<Settings>>,
        geometry: ugli::VertexBuffer<Vertex>,
        texture: ugli::Texture2d,
        map_texture: &ugli::Texture2d,
        predicate: fn(Color) -> bool,
        density: usize,
        is_palm: bool,
    ) -> Self {
        let context = app.ugli_context();
        let instances = {
            let mut instances = Vec::new();
            let map_size = map_texture.get_size();
            let framebuffer = ugli::FramebufferRead::new_color(
                context, ugli::ColorAttachmentRead::Texture(map_texture));
            let map = framebuffer.read_color();
            for _ in 0..density {
                let x = random::<f32>();
                let y = random::<f32>();
                let pixel = map.get_pixel(
                    (x * map_size.x as f32) as usize,
                    (y * map_size.y as f32) as usize,
                );
                if predicate(pixel) {
                    instances.push(Instance {
                        i_pos: vec2(x * 2.0 * MAP_SIZE - MAP_SIZE, y * 2.0 * MAP_SIZE - MAP_SIZE),
                        i_size: random::<f32>() * 0.5 + 1.0,
                    });
                }
            }
            ugli::VertexBuffer::new_static(context, instances)
        };
        Self {
            texture,
            material: codevisual::Material::new(
                context,
                (),
                Defines {
                    d_is_palm: is_palm,
                    d_fog_enabled: true,
                    d_heightmap_enabled: true,
                    d_transparency_enabled: true,
                },
                include_str!("shader.glsl"),
            ),
            geometry,
            instances,
            settings: settings.clone(),
        }
    }

    pub fn draw<U: ugli::Uniforms>(
        &mut self,
        framebuffer: &mut ugli::Framebuffer,
        uniforms: &U,
        percent: f64,
    ) {
        self.material.defines.d_fog_enabled = self.settings.borrow().fog_enabled;
        self.material.defines.d_transparency_enabled = self.settings.borrow().decor_transparency;
        self.material.defines.d_heightmap_enabled = self.settings.borrow().heightmap_enabled;
        let count = (self.instances.slice(..).len() as f64 * percent) as usize;
        ugli::draw(
            framebuffer,
            &self.material.ugli_program(),
            ugli::DrawMode::Triangles,
            &ugli::instanced(&self.geometry.slice(..), &self.instances.slice(..count)),
            &(uniforms, uniforms!(u_texture: &self.texture)),
            &ugli::DrawParameters {
                blend_mode: ugli::BlendMode::Alpha,
                ..Default::default()
            },
        );
    }
}

pub struct AllDecor {
    palms: Decor,
    bushes: Decor,
    settings: Rc<RefCell<Settings>>,
}

impl AllDecor {
    pub fn new(
        app: &codevisual::Application,
        resources: Resources,
        map_texture: &ugli::Texture2d,
        settings: &Rc<RefCell<Settings>>,
    ) -> Self {
        let context = app.ugli_context();
        macro_rules! vertex_data {
            ($scale:expr, [$(pos: $pos:expr, vt: $vt:expr);*;]) => {{
                let mut data = Vec::new();
                $(data.push(Vertex {
                    a_pos: $pos,
                    a_vt: $vt,
                }));*;
                data.into_iter()
                    .map(|mut v| {
                            v.a_pos = v.a_pos * $scale;
                            v
                        })
                    .collect()
            }}

        }
        let bushes = {
            let vertex_data = vertex_data!(3.0, [
                pos: vec3(-1.0, -1.0, 0.5), vt: vec2(0.0, 0.0);
                pos: vec3(-1.0, 1.0, 0.5), vt: vec2(1.0, 0.0);
                pos: vec3(1.0, 1.0, 0.5), vt: vec2(1.0, 1.0);

                pos: vec3(-1.0, -1.0, 0.5), vt: vec2(0.0, 0.0);
                pos: vec3(1.0, 1.0, 0.5), vt: vec2(1.0, 1.0);
                pos: vec3(1.0, -1.0, 0.5), vt: vec2(0.0, 1.0);

                pos: vec3(-1.0, -1.0, 0.0), vt: vec2(0.0, 0.5);
                pos: vec3(1.0, 1.0, 0.0), vt: vec2(1.0, 0.5);
                pos: vec3(1.0, 1.0, 1.0), vt: vec2(1.0, 1.0);

                pos: vec3(-1.0, -1.0, 0.0), vt: vec2(0.0, 0.5);
                pos: vec3(1.0, 1.0, 1.0), vt: vec2(1.0, 1.0);
                pos: vec3(-1.0, -1.0, 1.0), vt: vec2(0.0, 1.0);

                pos: vec3(-1.0, 1.0, 0.0), vt: vec2(0.0, 0.5);
                pos: vec3(1.0, -1.0, 0.0), vt: vec2(1.0, 0.5);
                pos: vec3(1.0, -1.0, 1.0), vt: vec2(1.0, 1.0);

                pos: vec3(-1.0, 1.0, 0.0), vt: vec2(0.0, 0.5);
                pos: vec3(1.0, -1.0, 1.0), vt: vec2(1.0, 1.0);
                pos: vec3(-1.0, 1.0, 1.0), vt: vec2(0.0, 1.0);
            ]);
            let geometry = ugli::VertexBuffer::new_static(context, vertex_data);
            Decor::new(
                app,
                settings,
                geometry,
                resources.bush_texture,
                map_texture,
                |color| color.blue < 0.1 && color.green > 0.5,
                50000,
                false,
            )
        };
        let palms = {
            const TRUNK_SIZE: f32 = 0.1;
            let vertex_data = vertex_data!(20.0, [
                pos: vec3(-TRUNK_SIZE, 0.0, 0.0), vt: vec2(0.75, 0.5);
                pos: vec3(0.0, -TRUNK_SIZE, 0.0), vt: vec2(0.75, 0.5);
                pos: vec3(0.0, 0.0, 1.0), vt: vec2(0.75, 0.5);

                pos: vec3(-TRUNK_SIZE, 0.0, 0.0), vt: vec2(0.75, 0.5);
                pos: vec3(0.0, TRUNK_SIZE, 0.0), vt: vec2(0.75, 0.5);
                pos: vec3(0.0, 0.0, 1.0), vt: vec2(0.75, 0.5);

                pos: vec3(TRUNK_SIZE, 0.0, 0.0), vt: vec2(0.75, 0.5);
                pos: vec3(0.0, TRUNK_SIZE, 0.0), vt: vec2(0.75, 0.5);
                pos: vec3(0.0, 0.0, 1.0), vt: vec2(0.75, 0.5);

                pos: vec3(TRUNK_SIZE, 0.0, 0.0), vt: vec2(0.75, 0.5);
                pos: vec3(0.0, -TRUNK_SIZE, 0.0), vt: vec2(0.75, 0.5);
                pos: vec3(0.0, 0.0, 1.0), vt: vec2(0.75, 0.5);

                pos: vec3(-1.0, -1.0, 1.0), vt: vec2(0.0, 0.0);
                pos: vec3(1.0, -1.0, 1.0), vt: vec2(0.5, 0.0);
                pos: vec3(1.0, 1.0, 1.0), vt: vec2(0.5, 1.0);

                pos: vec3(-1.0, -1.0, 1.0), vt: vec2(0.0, 0.0);
                pos: vec3(1.0, 1.0, 1.0), vt: vec2(0.5, 1.0);
                pos: vec3(-1.0, 1.0, 1.0), vt: vec2(0.0, 1.0);
            ]);
            let geometry = ugli::VertexBuffer::new_static(context, vertex_data);
            Decor::new(
                app,
                settings,
                geometry,
                {
                    let mut texture = resources.palm_texture;
                    texture.set_wrap_mode(ugli::WrapMode::Repeat);
                    texture
                },
                map_texture,
                |color| color.red > 0.5,
                20000,
                true,
            )
        };
        Self {
            bushes,
            palms,
            settings: settings.clone(),
        }
    }

    pub fn draw<U: ugli::Uniforms>(
        &mut self,
        framebuffer: &mut ugli::Framebuffer,
        uniforms: &U,
    ) {
        if self.settings.borrow().show_bushes {
            self.bushes.draw(
                framebuffer,
                uniforms,
                self.settings.borrow().decor_percent,
            );
        }
        self.palms.draw(
            framebuffer,
            uniforms,
            self.settings.borrow().decor_percent,
        );
    }
}
