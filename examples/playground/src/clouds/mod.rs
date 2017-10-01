use ::*;

resources! {
    Resources {
        cloud_texture: ugli::Texture2d = "assets/bush.png",
        cloud_map: ugli::Texture2d = "assets/cloud_map.png",
    }
}

#[derive(Uniforms)]
struct Uniforms {
    u_texture: ugli::Texture2d,
}

#[derive(Vertex)]
struct Vertex {
    a_pos: Vec3<f32>,
}

pub struct Clouds {
    uniforms: Uniforms,
    data: ugli::VertexBuffer<Vertex>,
    material: codevisual::Material<ShaderLib>,
}

impl Clouds {
    pub fn new(
        app: &Rc<codevisual::Application>,
        resources: Resources
    ) -> Self {
        let mut data = Vec::new();
        let map_size = resources.cloud_map.get_size();
        let framebuffer = ugli::FramebufferRead::new_color(
            app.ugli_context(), ugli::ColorAttachmentRead::Texture(&resources.cloud_map));
        let map = framebuffer.read_color();
        for _ in 0..70000 {
            let x = random::<f32>();
            let y = random::<f32>();
            let pixel = map.get_pixel(
                (x * map_size.x as f32) as usize,
                (y * map_size.y as f32) as usize,
            );
            if pixel.red > 0.5 {
                data.push(Vertex {
                    a_pos: vec3(x * 2.0 * MAP_SIZE - MAP_SIZE, y * 2.0 * MAP_SIZE - MAP_SIZE, random::<f32>() * 2.0 - 1.0),
                });
            }
        }
        Self {
            uniforms: Uniforms { u_texture: resources.cloud_texture },
            data: ugli::VertexBuffer::new_static(app.ugli_context(), data),
            material: codevisual::Material::new(app.ugli_context(), (), (), include_str!("shader.glsl")),
        }
    }

    pub fn draw<U: ugli::Uniforms>(
        &mut self,
        framebuffer: &mut ugli::Framebuffer,
        uniforms: &U,
    ) {
        ugli::draw(
            framebuffer,
            &self.material.ugli_program(),
            ugli::DrawMode::Points,
            &ugli::plain(&self.data.slice(..)),
            &(uniforms, &self.uniforms),
            &ugli::DrawParameters {
                depth_test: ugli::DepthTest::Off,
                blend_mode: ugli::BlendMode::Alpha,
                ..Default::default()
            },
        );
    }
}
