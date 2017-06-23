use commons::*;

use gl;
use gl::types::*;

pub trait Attribute: Copy + Sized {
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
impl Attribute for Vec2<f32> {
    fn get_gl_size() -> GLsizei {
        2
    }
    fn get_gl_type() -> GLenum {
        gl::FLOAT
    }
}
impl Attribute for Vec3<f32> {
    fn get_gl_size() -> GLsizei {
        3
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
