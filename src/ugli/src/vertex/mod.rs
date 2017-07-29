mod attribute;
pub use self::attribute::*;

mod buffer;
pub use self::buffer::*;

pub trait VertexAttributeConsumer {
    fn consume<A: VertexAttribute>(&mut self, name: &str, attribute: &A);
}

pub trait VertexData {
    fn walk_attributes<C>(&self, consumer: C)
    where
        C: VertexAttributeConsumer;
}

pub trait VertexDataConsumer {
    fn consume<D: VertexData>(&mut self, data: &VertexBufferSlice<D>, divisor: Option<usize>);
}

pub trait VertexDataSource {
    fn walk_data<C>(&self, consumer: C)
    where
        C: VertexDataConsumer;
}

pub struct PlainVertexDataSource<'a, T: VertexData + 'a> {
    buffer: &'a VertexBufferSlice<'a, T>,
}

impl<'a, T: VertexData + 'a> VertexDataSource for PlainVertexDataSource<'a, T> {
    fn walk_data<C>(&self, mut consumer: C)
    where
        C: VertexDataConsumer,
    {
        consumer.consume(self.buffer, None);
    }
}

pub fn plain<'a, T>(buffer: &'a VertexBufferSlice<'a, T>) -> PlainVertexDataSource<'a, T>
where
    T: VertexData + 'a,
{
    PlainVertexDataSource { buffer }
}

pub struct InstancedVertexDataSource<'a, V: VertexData + 'a, I: VertexData + 'a> {
    vertices: &'a VertexBufferSlice<'a, V>,
    instances: &'a VertexBufferSlice<'a, I>,
}

impl<'a, V, I> VertexDataSource for InstancedVertexDataSource<'a, V, I>
where
    V: VertexData + 'a,
    I: VertexData + 'a,
{
    fn walk_data<C>(&self, mut consumer: C)
    where
        C: VertexDataConsumer,
    {
        consumer.consume(self.vertices, None);
        consumer.consume(self.instances, Some(1));
    }
}

pub fn instanced<'a, V, I>(
    vertices: &'a VertexBufferSlice<'a, V>,
    instances: &'a VertexBufferSlice<'a, I>,
) -> InstancedVertexDataSource<'a, V, I>
where
    V: VertexData + 'a,
    I: VertexData + 'a,
{
    InstancedVertexDataSource {
        vertices,
        instances,
    }
}
