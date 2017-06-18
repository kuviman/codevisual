use std::rc::Rc;

use super::vertex;
use commons::*;

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Points,
    Lines,
    LineStrip,
    Triangles,
    TriangleFan,
    TriangleStrip,
}

pub trait VertexDataConsumer {
    fn consume<B: vertex::BufferView>(&mut self, data: &B);
}

pub trait Geometry {
    fn get_mode(&self) -> Mode;
    fn walk_data<F: VertexDataConsumer>(&self, f: &mut F);
}

pub struct PlainGeometry<D: vertex::Data> {
    mode: Mode,
    data: vertex::Buffer<D>,
}

impl<D: vertex::Data> Geometry for PlainGeometry<D> {
    fn get_mode(&self) -> Mode {
        self.mode
    }
    fn walk_data<F: VertexDataConsumer>(&self, f: &mut F) {
        f.consume(&self.data);
    }
}

impl<D: vertex::Data> PlainGeometry<D> {
    pub fn new(mode: Mode, data: Vec<D>) -> Self {
        assert!(match mode {
                    Mode::Points => true,
                    Mode::Lines => data.len() % 2 == 0,
                    Mode::LineStrip => data.len() >= 2,
                    Mode::TriangleFan => data.len() >= 3,
                    Mode::Triangles => data.len() % 3 == 0,
                    Mode::TriangleStrip => data.len() >= 3,
                },
                "Wroing vertex count");
        Self {
            mode,
            data: vertex::Buffer::new(data),
        }
    }
    pub fn get_data(&self) -> &vertex::Buffer<D> {
        &self.data
    }
    pub fn get_data_mut(&mut self) -> &mut vertex::Buffer<D> {
        &mut self.data
    }
}

pub struct InstancedGeometry<I: vertex::Data, B: Geometry> {
    instance_data: vertex::Buffer<I>,
    base: Rc<B>,
}

impl<I: vertex::Data, B: Geometry> Geometry for InstancedGeometry<I, B> {
    fn get_mode(&self) -> Mode {
        self.base.get_mode()
    }
    fn walk_data<F: VertexDataConsumer>(&self, f: &mut F) {
        self.base.walk_data(f);
        f.consume(&self.instance_data);
    }
}

impl<I: vertex::Data, B: Geometry> InstancedGeometry<I, B> {
    pub fn new(base: Rc<B>, instance_data: Vec<I>) -> Self {
        Self {
            instance_data: vertex::Buffer::new(instance_data),
            base,
        }
    }
    pub fn get_instance_data(&self) -> &vertex::Buffer<I> {
        &self.instance_data
    }
    pub fn get_instance_data_mut(&mut self) -> &mut vertex::Buffer<I> {
        &mut self.instance_data
    }
    pub fn slice<'a, R>(&'a self, range: R) -> InstancedGeometrySlice<'a, I, B, R>
        where R: RangeArgument<usize>
    {
        InstancedGeometrySlice {
            original: self,
            range,
        }
    }
}

pub struct InstancedGeometrySlice<'a, I: 'a + vertex::Data, B: 'a + Geometry, R: RangeArgument<usize>> {
    original: &'a InstancedGeometry<I, B>,
    range: R,
}

impl<'a, I, B, R> Geometry for InstancedGeometrySlice<'a, I, B, R>
    where I: vertex::Data,
          B: Geometry,
          R: RangeArgument<usize>
{
    fn get_mode(&self) -> Mode {
        self.original.get_mode()
    }
    fn walk_data<F: VertexDataConsumer>(&self, f: &mut F) {
        use super::vertex::BufferView;
        self.original.base.walk_data(f);
        f.consume(&self.original
                       .instance_data
                       .slice(CustomRange::from(&self.range)));
    }
}