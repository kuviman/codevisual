use common::*;

use gl::types::*;
use gl;

pub trait Attribute: Sized {
    fn get_gl_size() -> GLsizei;
    fn get_gl_type() -> GLenum;
}

impl Attribute for f32 {
    fn get_gl_size() -> GLsizei {
        1
    }
    fn get_gl_type() -> GLenum {
        gl::FLOAT
    }
}
impl<S> Attribute for Vec2<S> {
    fn get_gl_size() -> GLsizei {
        2
    }
    fn get_gl_type() -> GLenum {
        gl::FLOAT
    }
}
impl Attribute for Color {
    fn get_gl_size() -> GLsizei {
        4
    }
    fn get_gl_type() -> GLenum {
        gl::FLOAT
    }
}

pub trait AttributeConsumer {
    fn consume<A: Attribute>(&mut self, name: &str, value: &A);
}

pub trait Data: Sized {
    fn walk_attributes<F: AttributeConsumer>(&self, f: &mut F);
}

pub struct EmptyData;
impl Data for EmptyData {
    fn walk_attributes<F: AttributeConsumer>(&self, _: &mut F) {}
}