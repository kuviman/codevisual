use ::*;

pub struct QuadVertex {
    a_v: Vec2<f32>,
}

impl VertexData for QuadVertex {
    fn walk_attributes<C>(&self, mut consumer: C)
    where
        C: VertexAttributeConsumer,
    {
        consumer.consume("a_v", &self.a_v);
    }
}

pub type Quad = VertexBuffer<QuadVertex>;

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
    {
        if {
            if let None = *context.quad.borrow() {
                true
            } else {
                false
            }
        }
        {
            *context.quad.borrow_mut() = Some(Quad::new_static(
                context,
                vec![
                    QuadVertex { a_v: vec2(-1.0, -1.0) },
                    QuadVertex { a_v: vec2(1.0, -1.0) },
                    QuadVertex { a_v: vec2(1.0, 1.0) },
                    QuadVertex { a_v: vec2(-1.0, 1.0) },
                ],
            ));
        }
    }
    QuadRef { quad: context.quad.borrow() }
}