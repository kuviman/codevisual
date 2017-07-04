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
    u_map_size: f32,
    u_grass_texture: Rc<draw::Texture>,
    u_darkgrass_texture: Rc<draw::Texture>,
    u_dirt_texture: Rc<draw::Texture>,
    u_map_texture: Rc<draw::Texture>,
    u_bush_texture: Rc<draw::Texture>,
}

#[derive(Vertex)]
struct BushVertex {
    a_pos: Vec3<f32>,
    a_vt: Vec2<f32>,
}

#[derive(Vertex)]
struct BushInstance {
    i_pos: Vec2<f32>,
}

pub struct Ground {
    geometry: draw::PlainGeometry<VertexData>,
    uniforms: Uniforms,
    shader: draw::Shader,
    bush_geometry: draw::InstancedGeometry<BushInstance, draw::PlainGeometry<BushVertex>>,
    bush_shader: draw::Shader,
    water_geometry: draw::PlainGeometry<VertexData>,
    water_shader: draw::Shader,
}

impl Ground {
    pub fn new(app: &codevisual::Application, resources: &::Resources) -> Self {
        let bush_geometry = {
            let mut bush_instances = Vec::new();
            let map_size = resources.map_texture.get_size();
            let map = resources.map_texture.get_data();
            for _ in 0..10000 {
                let x = random::<f32>();
                let y = random::<f32>();
                let pixel = map.get_pixel((x * map_size.0 as f32) as usize,
                                          (y * map_size.1 as f32) as usize);
                if pixel.green < 0.5 {
                    continue;
                }
                bush_instances.push(BushInstance {
                                        i_pos: vec2(x * 2.0 * MAP_SIZE - MAP_SIZE,
                                                    y * 2.0 * MAP_SIZE - MAP_SIZE),
                                    });
            }
            let geometry = {
                let mut vertices = Vec::new();
                vertices.push(BushVertex {
                                  a_pos: vec3(-1.0, -1.0, 0.5),
                                  a_vt: vec2(0.0, 0.0),
                              });
                vertices.push(BushVertex {
                                  a_pos: vec3(-1.0, 1.0, 0.5),
                                  a_vt: vec2(1.0, 0.0),
                              });
                vertices.push(BushVertex {
                                  a_pos: vec3(1.0, 1.0, 0.5),
                                  a_vt: vec2(1.0, 1.0),
                              });
                vertices.push(BushVertex {
                                  a_pos: vec3(-1.0, -1.0, 0.5),
                                  a_vt: vec2(0.0, 0.0),
                              });
                vertices.push(BushVertex {
                                  a_pos: vec3(1.0, 1.0, 0.5),
                                  a_vt: vec2(1.0, 1.0),
                              });
                vertices.push(BushVertex {
                                  a_pos: vec3(1.0, -1.0, 0.5),
                                  a_vt: vec2(0.0, 1.0),
                              });
                for sgn in [-1.0 as f32, 1.0].iter() {
                    vertices.push(BushVertex {
                                      a_pos: vec3(-1.0, -1.0 * sgn, 0.0),
                                      a_vt: vec2(0.0, 0.5),
                                  });
                    vertices.push(BushVertex {
                                      a_pos: vec3(1.0, 1.0 * sgn, 0.0),
                                      a_vt: vec2(1.0, 0.5),
                                  });
                    vertices.push(BushVertex {
                                      a_pos: vec3(1.0, 1.0 * sgn, 1.0),
                                      a_vt: vec2(1.0, 1.0),
                                  });
                    vertices.push(BushVertex {
                                      a_pos: vec3(-1.0, -1.0 * sgn, 0.0),
                                      a_vt: vec2(0.0, 0.5),
                                  });
                    vertices.push(BushVertex {
                                      a_pos: vec3(1.0, 1.0 * sgn, 1.0),
                                      a_vt: vec2(1.0, 1.0),
                                  });
                    vertices.push(BushVertex {
                                      a_pos: vec3(-1.0, -1.0 * sgn, 1.0),
                                      a_vt: vec2(0.0, 1.0),
                                  });
                }
                Rc::new(draw::PlainGeometry::new(app, draw::geometry::Mode::Triangles, vertices))
            };
            draw::InstancedGeometry::new(app, geometry, bush_instances)
        };
        Ground {
            geometry: {
                let mut vertices = Vec::new();
                const N: usize = 64;
                for i in 0..N {
                    for j in 0..N {
                        let x1 = -MAP_SIZE + 2.0 * MAP_SIZE * i as f32 / N as f32;
                        let y1 = -MAP_SIZE + 2.0 * MAP_SIZE * j as f32 / N as f32;
                        let x2 = -MAP_SIZE + 2.0 * MAP_SIZE * (i as f32 + 1.0) / N as f32;
                        let y2 = -MAP_SIZE + 2.0 * MAP_SIZE * (j as f32 + 1.0) / N as f32;

                        vertices.push(VertexData { a_pos: vec2(x1, y1) });
                        vertices.push(VertexData { a_pos: vec2(x2, y1) });
                        vertices.push(VertexData { a_pos: vec2(x2, y2) });

                        vertices.push(VertexData { a_pos: vec2(x1, y1) });
                        vertices.push(VertexData { a_pos: vec2(x2, y2) });
                        vertices.push(VertexData { a_pos: vec2(x1, y2) });
                    }
                }
                draw::PlainGeometry::new(app, draw::geometry::Mode::Triangles, vertices)
            },
            uniforms: Uniforms {
                u_matrix: Mat4::identity(),
                u_map_size: MAP_SIZE,
                u_dirt_texture: resources.dirt_texture.clone(),
                u_grass_texture: resources.grass_texture.clone(),
                u_darkgrass_texture: resources.darkgrass_texture.clone(),
                u_map_texture: resources.map_texture.clone(),
                u_bush_texture: resources.bush_texture.clone(),
            },
            shader: draw::Shader::compile(app,
                                          include_str!("vertex.glsl"),
                                          include_str!("fragment.glsl"))
                    .unwrap(),
            bush_shader: draw::Shader::compile(app,
                                               include_str!("bush_vertex.glsl"),
                                               include_str!("bush_fragment.glsl"))
                    .unwrap(),
            bush_geometry,
            water_geometry: draw::PlainGeometry::new(app,
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
            water_shader: draw::Shader::compile(app,
                                                include_str!("water_vertex.glsl"),
                                                include_str!("water_fragment.glsl"))
                    .unwrap(),
        }
    }

    pub fn render<T: draw::Target>(&mut self, target: &mut T, global_uniforms: &::GlobalUniforms) {
        self.uniforms.u_matrix = global_uniforms.u_matrix;
        target.draw(&self.geometry, &self.shader, &self.uniforms);
        target.draw(&self.bush_geometry, &self.bush_shader, &self.uniforms);
        target.draw(&self.water_geometry, &self.water_shader, &self.uniforms);
    }
}