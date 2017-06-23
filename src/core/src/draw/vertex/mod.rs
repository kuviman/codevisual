mod attribute;
pub use self::attribute::*;

mod buffer;
pub use self::buffer::*;

pub trait Data: Sized {
    fn walk_attributes<F: AttributeConsumer>(&self, f: &mut F);
}

pub trait DataConsumer {
    fn consume<B: BufferView>(&mut self, data: &B);
}