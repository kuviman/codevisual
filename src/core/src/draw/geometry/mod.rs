use super::vertex;

mod plain;
pub use self::plain::*;

mod instanced;
pub use self::instanced::*;

mod immediate;
pub use self::immediate::*;

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Points,
    Lines,
    LineStrip,
    Triangles,
    TriangleFan,
    TriangleStrip,
}

pub trait Geometry {
    fn get_mode(&self) -> Mode;
    fn walk_data<F: vertex::DataConsumer>(&self, f: &mut F);
}