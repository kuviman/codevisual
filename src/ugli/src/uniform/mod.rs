use ::*;

mod raw {
    use ::*;
    pub struct Location<'a> {
        pub location: GLint,
        pub texture_count: &'a mut usize,
    }
}

mod storage;
pub use self::storage::*;

pub(crate) use self::raw::Location as UniformLocation;

pub trait Uniform {
    fn apply<'a>(&self, location: raw::Location<'a>);
}

pub trait UniformConsumer {
    fn consume<U: Uniform>(&mut self, name: &str, uniform: &U);
}

impl Uniform for f32 {
    fn apply<'a>(&self, location: raw::Location<'a>) {
        unsafe {
            gl::Uniform1f(location.location, *self);
        }
    }
}

impl Uniform for Vec2<f32> {
    fn apply<'a>(&self, location: raw::Location<'a>) {
        unsafe {
            gl::Uniform2f(location.location, self.x, self.y);
        }
    }
}

impl Uniform for Mat4<f32> {
    fn apply<'a>(&self, location: raw::Location<'a>) {
        unsafe {
            gl::UniformMatrix4fv(
                location.location,
                1,
                gl::FALSE,
                self as *const Self as *const _,
            );
        }
    }
}

impl Uniform for Color {
    fn apply<'a>(&self, location: raw::Location<'a>) {
        unsafe {
            gl::Uniform4f(
                location.location,
                self.red,
                self.green,
                self.blue,
                self.alpha,
            );
        }
    }
}

impl Uniform for Texture2d {
    fn apply<'a>(&self, location: raw::Location<'a>) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + (*location.texture_count) as GLenum);
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
            gl::Uniform1i(location.location, (*location.texture_count) as GLint);
        }
        (*location.texture_count) += 1;
    }
}

impl<'a, U: Uniform> Uniform for &'a U {
    fn apply<'b>(&self, location: raw::Location<'b>) {
        (*self).apply(location);
    }
}
