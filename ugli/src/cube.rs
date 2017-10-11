use ::*;

#[derive(Debug)]
pub struct CubeVertex {
    a_v: Vec3<f32>,
    a_vt: Vec2<f32>,
    a_n: Vec3<f32>,
    a_cube_pos: Vec3<f32>,
}

impl Vertex for CubeVertex {
    fn walk_attributes<C>(&self, mut consumer: C)
        where
            C: VertexAttributeConsumer,
    {
        consumer.consume("a_v", &self.a_v);
        consumer.consume("a_vt", &self.a_vt);
        consumer.consume("a_n", &self.a_n);
        consumer.consume("a_cube_pos", &self.a_cube_pos);
    }
}

pub struct Cube(VertexBuffer<CubeVertex>);

impl Cube {
    pub const DRAW_MODE: DrawMode = DrawMode::Triangles;
    pub fn new_symmetric(context: &Context, size: f32) -> Self {
        Self::new(context, vec3(-size, -size, -size), vec3(size, size, size))
    }
    pub fn new(context: &Context, p1: Vec3<f32>, p2: Vec3<f32>) -> Self {
        Cube(VertexBuffer::new_static(
            context,
            {
                let mut vs = Vec::new();
                {
                    let mut add_v = |v: Vec3<f32>, vt: Vec2<f32>, n: Vec3<f32>| {
                        let vt = vec2(vt.x / 4.0, vt.y / 3.0);
                        vs.push(CubeVertex {
                            a_v: vec3(
                                p1.x * v.x + p2.x * (1.0 - v.x),
                                p1.y * v.y + p2.y * (1.0 - v.y),
                                p1.z * v.z + p2.z * (1.0 - v.z),
                            ),
                            a_vt: vt,
                            a_cube_pos: v,
                            a_n: n,
                        });
                    };
                    let mut add_quad = |p: Vec3<f32>, e1: Vec3<f32>, e2: Vec3<f32>,
                                        t_p: Vec2<f32>, t_e1: Vec2<f32>, t_e2: Vec2<f32>| {
                        let n = Vec3::cross(e1, e2);

                        add_v(p, t_p, n);
                        add_v(p + e1, t_p + t_e1, n);
                        add_v(p + e1 + e2, t_p + t_e1 + t_e2, n);

                        add_v(p, t_p, n);
                        add_v(p + e1 + e2, t_p + t_e1 + t_e2, n);
                        add_v(p + e2, t_p + t_e2, n);
                    };

                    add_quad(vec3(0.0, 0.0, 0.0), vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                             vec2(1.0, 0.0), vec2(1.0, 0.0), vec2(0.0, 1.0));
                    add_quad(vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0),
                             vec2(0.0, 1.0), vec2(1.0, 0.0), vec2(0.0, 1.0));
                    add_quad(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0), vec3(1.0, 0.0, 0.0),
                             vec2(4.0, 1.0), vec2(0.0, 1.0), vec2(-1.0, 0.0));
                    add_quad(vec3(1.0, 1.0, 1.0), vec3(0.0, -1.0, 0.0), vec3(-1.0, 0.0, 0.0),
                             vec2(2.0, 2.0), vec2(0.0, 1.0), vec2(-1.0, 0.0));
                    add_quad(vec3(1.0, 1.0, 1.0), vec3(0.0, 0.0, -1.0), vec3(0.0, -1.0, 0.0),
                             vec2(2.0, 2.0), vec2(0.0, -1.0), vec2(1.0, 0.0));
                    add_quad(vec3(1.0, 1.0, 1.0), vec3(-1.0, 0.0, 0.0), vec3(0.0, 0.0, -1.0),
                             vec2(2.0, 2.0), vec2(-1.0, 0.0), vec2(0.0, -1.0));
                }

                vs
            },
        ))
    }
}

impl Deref for Cube {
    type Target = VertexBuffer<CubeVertex>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct CubeRef<'a> {
    cube: Ref<'a, Option<Cube>>,
}

impl<'a> Deref for CubeRef<'a> {
    type Target = Cube;
    fn deref(&self) -> &Cube {
        self.cube.deref().as_ref().unwrap()
    }
}

pub fn cube(context: &Context) -> CubeRef {
    if context.cube.borrow().is_none() {
        *context.cube.borrow_mut() = Some(Cube::new_symmetric(context, 1.0));
    }
    CubeRef { cube: context.cube.borrow() }
}