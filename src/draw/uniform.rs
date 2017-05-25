use common::*;
use gl::types::*;
use gl;

pub trait Value: Sized {
    fn apply(&self, location: GLint, texture_count: &mut usize);
}

impl Value for Vec2 {
    fn apply(&self, location: GLint, _: &mut usize) {
        unsafe {
            gl::Uniform2f(location, self.x, self.y);
        }
    }
}

impl Value for Color {
    fn apply(&self, location: GLint, _: &mut usize) {
        unsafe {
            gl::Uniform4f(location, self.red, self.green, self.blue, self.alpha);
        }
    }
}

pub trait ValueConsumer {
    fn consume<V: Value>(&mut self, name: &str, value: &V);
}

pub trait Data {
    fn walk<F: ValueConsumer>(&self, &mut F);
}