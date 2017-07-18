use super::*;
use super::super::vertex;

pub struct Immediate<'a, V: vertex::BufferView + 'a, B: Geometry + 'a> {
    instance_data: &'a V,
    base: &'a B,
}

impl<'a, V: vertex::BufferView, B: Geometry> Geometry for Immediate<'a, V, B> {
    fn get_mode(&self) -> Mode {
        self.base.get_mode()
    }
    fn walk_data<F: vertex::DataConsumer>(&self, f: &mut F) {
        self.base.walk_data(f);
        f.consume(self.instance_data);
    }
}

impl<'a, V: vertex::BufferView, B: Geometry> Immediate<'a, V, B> {
    pub fn new(base: &'a B, instance_data: &'a V) -> Self {
        Self {
            instance_data,
            base,
        }
    }
}