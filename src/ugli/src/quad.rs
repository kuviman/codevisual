use ::*;

#[derive(Debug)]
pub struct QuadVertex {
    a_v: Vec2<f32>,
    a_quad_pos: Vec2<f32>,
}

impl VertexData for QuadVertex {
    fn walk_attributes<C>(&self, mut consumer: C)
        where
            C: VertexAttributeConsumer,
    {
        consumer.consume("a_v", &self.a_v);
        consumer.consume("a_quad_pos", &self.a_quad_pos);
    }
}

pub struct Quad(VertexBuffer<QuadVertex>);

impl Quad {
    pub const DRAW_MODE: DrawMode = DrawMode::TriangleFan;
    pub fn new_symmetric(context: &Context, size: f32) -> Self {
        Self::new(context, vec2(-size, -size), vec2(size, size))
    }
    pub fn new(context: &Context, p1: Vec2<f32>, p2: Vec2<f32>) -> Self {
        Quad(VertexBuffer::new_static(
            context,
            vec![
                QuadVertex { a_v: vec2(p1.x, p1.y), a_quad_pos: vec2(0.0, 0.0) },
                QuadVertex { a_v: vec2(p2.x, p1.y), a_quad_pos: vec2(1.0, 0.0) },
                QuadVertex { a_v: vec2(p2.x, p2.y), a_quad_pos: vec2(1.0, 1.0) },
                QuadVertex { a_v: vec2(p1.x, p2.y), a_quad_pos: vec2(0.0, 1.0) },
            ],
        ))
    }
}

impl Deref for Quad {
    type Target = VertexBuffer<QuadVertex>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct QuadRef<'a> {
    quad: Ref<'a, Option<Quad>>,
}

impl<'a> Deref for QuadRef<'a> {
    type Target = Quad;
    fn deref(&self) -> &Quad {
        self.quad.deref().as_ref().unwrap()
    }
}

pub fn quad(context: &Context) -> QuadRef {
    if context.quad.borrow().is_none() {
        *context.quad.borrow_mut() = Some(Quad::new_symmetric(context, 1.0));
    }
    QuadRef { quad: context.quad.borrow() }
}