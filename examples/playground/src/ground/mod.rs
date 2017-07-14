use codevisual::commons::*;
use codevisual::{self, draw};

use MAP_SIZE;

#[derive(Vertex, Debug, Copy, Clone)]
pub struct VertexData {
    a_pos: Vec2<f32>,
}

#[derive(Uniforms)]
struct Uniforms {
    u_map_size: f32,
    u_grass_texture: Rc<draw::Texture>,
    u_darkgrass_texture: Rc<draw::Texture>,
    u_dirt_texture: Rc<draw::Texture>,
    u_map_texture: Rc<draw::Texture>,
    u_bush_texture: Rc<draw::Texture>,
    u_palm_texture: Rc<draw::Texture>,
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

#[derive(Vertex)]
struct PalmVertex {
    a_pos: Vec3<f32>,
    a_vt: Vec2<f32>,
}

#[derive(Vertex)]
struct PalmInstance {
    i_pos: Vec2<f32>,
    i_size: f32,
}

pub struct Ground {
    geometry: draw::PlainGeometry<VertexData>,
    uniforms: Uniforms,
    shader: draw::Shader,
    bush_geometry: draw::InstancedGeometry<BushInstance, draw::PlainGeometry<BushVertex>>,
    bush_shader: draw::Shader,
    water_geometry: draw::PlainGeometry<VertexData>,
    water_shader: draw::Shader,
    palm_geometry: draw::InstancedGeometry<PalmInstance, draw::PlainGeometry<PalmVertex>>,
    palm_shader: draw::Shader,
}

impl Ground {
    pub fn new(app: &codevisual::Application, resources: &::Resources) -> Self {
        let bush_geometry = {
            let mut instances = Vec::new();
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
                instances.push(BushInstance {
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
            draw::InstancedGeometry::new(app, geometry, instances)
        };
        let palm_geometry = {
            let mut instances = Vec::new();
            let map_size = resources.map_texture.get_size();
            let map = resources.map_texture.get_data();
            for _ in 0..4000 {
                let x = random::<f32>();
                let y = random::<f32>();
                let pixel = map.get_pixel((x * map_size.0 as f32) as usize,
                                          (y * map_size.1 as f32) as usize);
                if pixel.red < 0.5 {
                    continue;
                }
                instances.push(PalmInstance {
                                   i_pos: vec2(x * 2.0 * MAP_SIZE - MAP_SIZE,
                                               y * 2.0 * MAP_SIZE - MAP_SIZE),
                                   i_size: random::<f32>() * 0.5 + 1.0,
                               });
            }
            let geometry = {
                let mut vertices = Vec::new();
                const TRUNK_SIZE: f32 = 0.1;
                vertices.push(PalmVertex {
                                  a_pos: vec3(-TRUNK_SIZE, 0.0, 0.0),
                                  a_vt: vec2(0.75, 0.5),
                              });
                vertices.push(PalmVertex {
                                  a_pos: vec3(0.0, -TRUNK_SIZE, 0.0),
                                  a_vt: vec2(0.75, 0.5),
                              });
                vertices.push(PalmVertex {
                                  a_pos: vec3(0.0, 0.0, 1.0),
                                  a_vt: vec2(0.75, 0.5),
                              });

                vertices.push(PalmVertex {
                                  a_pos: vec3(-TRUNK_SIZE, 0.0, 0.0),
                                  a_vt: vec2(0.75, 0.5),
                              });
                vertices.push(PalmVertex {
                                  a_pos: vec3(0.0, TRUNK_SIZE, 0.0),
                                  a_vt: vec2(0.75, 0.5),
                              });
                vertices.push(PalmVertex {
                                  a_pos: vec3(0.0, 0.0, 1.0),
                                  a_vt: vec2(0.75, 0.5),
                              });

                vertices.push(PalmVertex {
                                  a_pos: vec3(TRUNK_SIZE, 0.0, 0.0),
                                  a_vt: vec2(0.75, 0.5),
                              });
                vertices.push(PalmVertex {
                                  a_pos: vec3(0.0, TRUNK_SIZE, 0.0),
                                  a_vt: vec2(0.75, 0.5),
                              });
                vertices.push(PalmVertex {
                                  a_pos: vec3(0.0, 0.0, 1.0),
                                  a_vt: vec2(0.75, 0.5),
                              });

                vertices.push(PalmVertex {
                                  a_pos: vec3(TRUNK_SIZE, 0.0, 0.0),
                                  a_vt: vec2(0.75, 0.5),
                              });
                vertices.push(PalmVertex {
                                  a_pos: vec3(0.0, -TRUNK_SIZE, 0.0),
                                  a_vt: vec2(0.75, 0.5),
                              });
                vertices.push(PalmVertex {
                                  a_pos: vec3(0.0, 0.0, 1.0),
                                  a_vt: vec2(0.75, 0.5),
                              });

                vertices.push(PalmVertex {
                                  a_pos: vec3(-1.0, -1.0, 1.0),
                                  a_vt: vec2(0.0, 0.0),
                              });
                vertices.push(PalmVertex {
                                  a_pos: vec3(1.0, -1.0, 1.0),
                                  a_vt: vec2(0.5, 0.0),
                              });
                vertices.push(PalmVertex {
                                  a_pos: vec3(1.0, 1.0, 1.0),
                                  a_vt: vec2(0.5, 1.0),
                              });

                vertices.push(PalmVertex {
                                  a_pos: vec3(-1.0, -1.0, 1.0),
                                  a_vt: vec2(0.0, 0.0),
                              });
                vertices.push(PalmVertex {
                                  a_pos: vec3(1.0, 1.0, 1.0),
                                  a_vt: vec2(0.5, 1.0),
                              });
                vertices.push(PalmVertex {
                                  a_pos: vec3(-1.0, 1.0, 1.0),
                                  a_vt: vec2(0.0, 1.0),
                              });

                Rc::new(draw::PlainGeometry::new(app, draw::geometry::Mode::Triangles, vertices))
            };
            draw::InstancedGeometry::new(app, geometry, instances)
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
                u_map_size: MAP_SIZE,
                u_dirt_texture: resources.dirt_texture.clone(),
                u_grass_texture: resources.grass_texture.clone(),
                u_darkgrass_texture: resources.darkgrass_texture.clone(),
                u_map_texture: resources.map_texture.clone(),
                u_bush_texture: resources.bush_texture.clone(),
                u_palm_texture: resources.palm_texture.clone(),
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
            palm_shader: draw::Shader::compile(app,
                                               include_str!("palm_vertex.glsl"),
                                               include_str!("palm_fragment.glsl"))
                    .unwrap(),
            palm_geometry,
        }
    }

    pub fn render<T: draw::Target>(&mut self, target: &mut T, global_uniforms: &::GlobalUniforms) {
        let uniforms = draw::uniform::cons(global_uniforms, &self.uniforms);
        target.draw(&self.geometry, &self.shader, &uniforms);
        target.draw(&self.bush_geometry, &self.bush_shader, &uniforms);
        target.draw(&self.palm_geometry, &self.palm_shader, &uniforms);
        target.draw(&self.water_geometry, &self.water_shader, &uniforms);
    }
}