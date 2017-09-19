use ::*;

#[derive(Vertex)]
struct Vertex {
    a_v: Vec3<f32>,
}

pub struct StaticMap {
    geometry: ugli::VertexBuffer<Vertex>,
    material: Material,
}

impl StaticMap {
    pub fn new(app: &Rc<codevisual::Application>) -> Self {
        let geometry = {
            let mut data = Vec::new();
            for (x, y) in vec![(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
                data.push(Vertex { a_v: vec3(x, 0.0, y) });
            }
            ugli::VertexBuffer::new_static(app.ugli_context(), data)
        };
        Self {
            geometry,
            material: Material::new(app.ugli_context(), (), (), include_str!("shader.glsl")),
        }
    }
    pub fn draw<U: ugli::UniformStorage>(&mut self, framebuffer: &mut ugli::Framebuffer, uniforms: U) {
        ugli::draw(framebuffer,
                   &self.material.ugli_program(),
                   ugli::DrawMode::LineLoop,
                   &ugli::plain(&self.geometry.slice(..)),
                   &uniforms,
                   &ugli::DrawParameters {
                       ..Default::default()
                   });
    }
}