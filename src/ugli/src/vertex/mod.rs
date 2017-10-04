mod attribute;

pub use self::attribute::*;

mod buffer;

pub use self::buffer::*;

pub trait VertexAttributeConsumer {
    fn consume<A: VertexAttribute>(&mut self, name: &str, attribute: &A);
}

pub trait Vertex {
    fn walk_attributes<C>(&self, consumer: C)
        where
            C: VertexAttributeConsumer;
}

pub trait VertexDataConsumer {
    fn consume<'a, T: Vertex + 'a, D: IntoVertexBufferSlice<'a, T>>(&mut self, data: D, divisor: Option<usize>);
}

pub trait VertexDataSource {
    fn walk_data<C>(&self, consumer: C)
        where
            C: VertexDataConsumer;
}

impl<'a, S: VertexDataSource> VertexDataSource for &'a S {
    fn walk_data<C>(&self, consumer: C) where
        C: VertexDataConsumer {
        (*self).walk_data(consumer);
    }
}

impl<'a, T: Vertex + 'a> VertexDataSource for &'a VertexBuffer<T> {
    fn walk_data<C>(&self, mut consumer: C)
        where
            C: VertexDataConsumer,
    {
        consumer.consume(*self, None);
    }
}

impl<'a, T: Vertex + 'a> VertexDataSource for VertexBufferSlice<'a, T> {
    fn walk_data<C>(&self, mut consumer: C)
        where
            C: VertexDataConsumer,
    {
        consumer.consume(self, None);
    }
}

pub struct InstancedVertexDataSource<'a, V: Vertex + 'a, I: Vertex + 'a> {
    vertices: VertexBufferSlice<'a, V>,
    instances: VertexBufferSlice<'a, I>,
}

impl<'a, V, I> VertexDataSource for InstancedVertexDataSource<'a, V, I>
    where
        V: Vertex + 'a,
        I: Vertex + 'a,
{
    fn walk_data<C>(&self, mut consumer: C)
        where
            C: VertexDataConsumer,
    {
        consumer.consume(&self.vertices, None);
        consumer.consume(&self.instances, Some(1));
    }
}

pub fn instanced<'a, V, I, VS, IS>(vertices: VS, instances: IS) -> InstancedVertexDataSource<'a, V, I>
    where
        V: Vertex + 'a,
        I: Vertex + 'a,
        VS: IntoVertexBufferSlice<'a, V>,
        IS: IntoVertexBufferSlice<'a, I>
{
    InstancedVertexDataSource {
        vertices: vertices.into_slice(),
        instances: instances.into_slice(),
    }
}
