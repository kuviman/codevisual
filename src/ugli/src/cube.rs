use ::*;

#[derive(Debug)]
pub struct CubeVertex {
    a_v: Vec3<f32>,
    a_n: Vec3<f32>,
    a_cube_pos: Vec3<f32>,
}

impl VertexData for CubeVertex {
    fn walk_attributes<C>(&self, mut consumer: C)
        where
            C: VertexAttributeConsumer,
    {
        consumer.consume("a_v", &self.a_v);
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
                    let mut add_v = |p: Vec3<f32>, n: Vec3<f32>| {
                        vs.push(CubeVertex {
                            a_v: vec3(
                                p1.x * p.x + p2.x * (1.0 - p.x),
                                p1.y * p.y + p2.y * (1.0 - p.y),
                                p1.z * p.z + p2.z * (1.0 - p.z),
                            ),
                            a_cube_pos: p,
                            a_n: n,
                        });
                    };
                    let mut add_quad = |p: Vec3<f32>, e1: Vec3<f32>, e2: Vec3<f32>| {
                        let n = Vec3::cross(e1, e2);

                        add_v(p, n);
                        add_v(p + e1, n);
                        add_v(p + e1 + e2, n);

                        add_v(p, n);
                        add_v(p + e1 + e2, n);
                        add_v(p + e2, n);
                    };

                    add_quad(vec3(0.0, 0.0, 0.0), vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0));
                    add_quad(vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0));
                    add_quad(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0), vec3(1.0, 0.0, 0.0));
                    add_quad(vec3(1.0, 1.0, 1.0), vec3(0.0, -1.0, 0.0), vec3(-1.0, 0.0, 0.0));
                    add_quad(vec3(1.0, 1.0, 1.0), vec3(0.0, 0.0, -1.0), vec3(0.0, -1.0, 0.0));
                    add_quad(vec3(1.0, 1.0, 1.0), vec3(-1.0, 0.0, 0.0), vec3(0.0, 0.0, -1.0));
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