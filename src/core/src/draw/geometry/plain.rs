use super::*;
use super::super::vertex;

pub struct PlainGeometry<D: vertex::Data> {
    mode: Mode,
    data: vertex::Buffer<D>,
}

impl<D: vertex::Data> Geometry for PlainGeometry<D> {
    fn get_mode(&self) -> Mode {
        self.mode
    }
    fn walk_data<F: vertex::DataConsumer>(&self, f: &mut F) {
        f.consume(&self.data);
    }
}

impl<D: vertex::Data> PlainGeometry<D> {
    pub fn new(app: &::Application, mode: Mode, data: Vec<D>) -> Self {
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
            data: vertex::Buffer::new(app, data),
        }
    }
    pub fn get_data(&self) -> &vertex::Buffer<D> {
        &self.data
    }
    pub fn get_data_mut(&mut self) -> &mut vertex::Buffer<D> {
        &mut self.data
    }
}