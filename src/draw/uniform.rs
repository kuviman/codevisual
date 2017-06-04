use common::*;
use gl::types::*;
use gl;

pub trait Value: Sized {
    fn apply(&self, location: GLint, texture_count: &mut usize);
}

impl Value for f32 {
    fn apply(&self, location: GLint, _: &mut usize) {
        unsafe {
            gl::Uniform1f(location, *self);
        }
    }
}

impl Value for Vec2<f32> {
    fn apply(&self, location: GLint, _: &mut usize) {
        unsafe {
            gl::Uniform2f(location, self.x, self.y);
        }
    }
}

impl Value for Mat4<f32> {
    fn apply(&self, location: GLint, _: &mut usize) {
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, self as *const Self as *const _);
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

impl Value for super::Texture {
    fn apply(&self, location: GLint, texture_count: &mut usize) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + (*texture_count) as GLenum);
            gl::BindTexture(gl::TEXTURE_2D, self.get_handle());
            gl::Uniform1i(location, (*texture_count) as GLint);
            (*texture_count) += 1;
        }
    }
}

pub trait ValueConsumer {
    fn consume<V: Value>(&mut self, name: &str, value: &V);
}

pub trait Data {
    fn walk<F: ValueConsumer>(&self, &mut F);
}