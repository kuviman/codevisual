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

resources! {
    Resources {
        bush_texture: ugli::Texture2d = "assets/bush.png",
        palm_texture: ugli::Texture2d = "assets/palm.png",
    }
}

struct Decor {
    texture: ugli::Texture2d,
    shader: codevisual::Shader,
    geometry: ugli::VertexBuffer<Vertex>,
    instances: ugli::VertexBuffer<Instance>,
}

impl Decor {
    pub fn new(app: &codevisual::Application,
               geometry: ugli::VertexBuffer<Vertex>,
               texture: ugli::Texture2d,
               map_texture: &ugli::Texture2d,
               predicate: fn(Color) -> bool,
               density: usize)
               -> Self {
        let context = app.get_window().ugli_context();
        let instances = {
            let mut instances = Vec::new();
            let map_size = map_texture.get_size();
            let framebuffer = ugli::Framebuffer::new_color(context, map_texture);
            let map = framebuffer.read_color();
            for _ in 0..density {
                let x = random::<f32>();
                let y = random::<f32>();
                let pixel = map.get_pixel((x * map_size.x as f32) as usize,
                                          (y * map_size.y as f32) as usize);
                if predicate(pixel) {
                    instances.push(Instance {
                                       i_pos: vec2(x * 2.0 * MAP_SIZE - MAP_SIZE,
                                                   y * 2.0 * MAP_SIZE - MAP_SIZE),
                                       i_size: random::<f32>() * 0.5 + 1.0,
                                   });
                }
            }
            ugli::VertexBuffer::new(context, instances)
        };
        Self {
            texture,
            shader: codevisual::Shader::compile::<::ShaderLib>(context,
                                                               &(),
                                                               include_str!("shader.glsl")),
            geometry,
            instances,
        }
    }

    pub fn draw<U: ugli::UniformStorage>(&mut self,
                                         framebuffer: &mut ugli::DefaultFramebuffer,
                                         uniforms: &U,
                                         percent: f64) {
        let count = (self.instances.slice(..).len() as f64 * percent) as usize;
        ugli::draw(framebuffer,
                   self.shader.ugli_program(),
                   ugli::DrawMode::Triangles,
                   &ugli::instanced(&self.geometry.slice(..), &self.instances.slice(..count)),
                   &(uniforms, uniforms!(u_texture: &self.texture)),
                   &ugli::DrawParameters {
                       blend_mode: ugli::BlendMode::Alpha,
                       ..Default::default()
                   });
    }
}

pub struct AllDecor {
    palms: Decor,
    bushes: Decor,
    percent: Rc<Cell<f64>>,
}

impl AllDecor {
    pub fn new(app: &codevisual::Application,
               resources: Resources,
               map_texture: &ugli::Texture2d)
               -> Self {
        let context = app.get_window().ugli_context();
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
            let geometry = ugli::VertexBuffer::new(context, vertex_data);
            Decor::new(app,
                       geometry,
                       resources.bush_texture,
                       map_texture,
                       |color| color.blue < 0.1 && color.green > 0.5,
                       10000)
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
            let geometry = ugli::VertexBuffer::new(context, vertex_data);
            Decor::new(app,
                       geometry,
                       resources.palm_texture,
                       map_texture,
                       |color| color.red > 0.5,
                       4000)
        };
        Self {
            bushes,
            palms,
            percent: {
                let setting = Rc::new(Cell::new(0.0));
                {
                    let setting = setting.clone();
                    const MAX_VALUE: i32 = 1000;
                    app.add_setting(codevisual::Setting::I32 {
                                        name: String::from("Decorations"),
                                        min_value: 0,
                                        max_value: MAX_VALUE,
                                        default_value: (MAX_VALUE as f64 * setting.get()) as i32,
                                        setter: Box::new(move |new_value| {
                                                             setting.set(new_value as f64 /
                                                                         MAX_VALUE as f64);
                                                         }),
                                    });
                }
                setting
            },
        }
    }

    pub fn draw<U: ugli::UniformStorage>(&mut self,
                                         framebuffer: &mut ugli::DefaultFramebuffer,
                                         uniforms: &U) {
        self.bushes.draw(framebuffer, uniforms, self.percent.get());
        self.palms.draw(framebuffer, uniforms, self.percent.get());
    }
}