use super::*;
use super::super::vertex;

use commons::*;
use std::rc::Rc;

pub struct InstancedGeometry<I: vertex::Data, B: Geometry> {
    instance_data: vertex::Buffer<I>,
    base: Rc<B>,
}

impl<I: vertex::Data, B: Geometry> Geometry for InstancedGeometry<I, B> {
    fn get_mode(&self) -> Mode {
        self.base.get_mode()
    }
    fn walk_data<F: vertex::DataConsumer>(&self, f: &mut F) {
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
    fn walk_data<F: vertex::DataConsumer>(&self, f: &mut F) {
        use super::vertex::BufferView;
        self.original.base.walk_data(f);
        f.consume(&self.original
                       .instance_data
                       .slice(CustomRange::from(&self.range)));
    }
}