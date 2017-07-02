use codevisual::commons::*;
use codevisual::{self, draw};

use MAP_SIZE;

#[derive(Vertex, Debug, Copy, Clone)]
pub struct VertexData {
    a_pos: Vec2<f32>,
}

#[derive(Uniforms)]
struct Uniforms {
    u_matrix: Mat4<f32>,
    u_grass_texture: Rc<draw::Texture>,
    u_dirt_texture: Rc<draw::Texture>,
    u_map_texture: Rc<draw::Texture>,
}

pub struct Ground {
    geometry: draw::PlainGeometry<VertexData>,
    uniforms: Uniforms,
    shader: draw::Shader,
}

impl Ground {
    pub fn new(app: &codevisual::Application, resources: &::Resources) -> Self {
        Ground {
            geometry: draw::PlainGeometry::new(app,
                                               draw::geometry::Mode::TriangleFan,
                                               vec![VertexData {
                                                        a_pos: vec2(-MAP_SIZE, -MAP_SIZE),
                                                    },
                                                    VertexData {
                                                        a_pos: vec2(-MAP_SIZE, MAP_SIZE),
                                                    },
                                                    VertexData {
                                                        a_pos: vec2(MAP_SIZE, MAP_SIZE),
                                                    },
                                                    VertexData {
                                                        a_pos: vec2(MAP_SIZE, -MAP_SIZE),
                                                    }]),
            uniforms: Uniforms {
                u_matrix: Mat4::identity(),
                u_dirt_texture: resources.dirt_texture.clone(),
                u_grass_texture: resources.grass_texture.clone(),
                u_map_texture: resources.map_texture.clone(),
            },
            shader: draw::Shader::compile(app,
                                          include_str!("vertex.glsl"),
                                          include_str!("fragment.glsl"))
                    .unwrap(),
        }
    }

    pub fn render<T: draw::Target>(&mut self, target: &mut T, global_uniforms: &::GlobalUniforms) {
        self.uniforms.u_matrix = global_uniforms.u_matrix;
        target.draw(&self.geometry, &self.shader, &self.uniforms);
    }
}