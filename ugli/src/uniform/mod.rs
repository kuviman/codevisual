use ::*;

mod storage;

pub use self::storage::*;

pub(crate) static mut UNIFORM_TEXTURE_COUNT: usize = 0;

pub trait Uniform {
    fn apply(&self, info: &UniformInfo);
    #[allow(unused_variables)]
    fn walk_extra<C>(&self, name: &str, visitor: &mut C)
    where
        C: UniformVisitor,
    {
    }
}

pub trait UniformVisitor {
    fn visit<U: Uniform>(&mut self, name: &str, uniform: &U);
}

impl Uniform for f32 {
    fn apply(&self, info: &UniformInfo) {
        unsafe {
            gl::Uniform1f(info.location, *self);
        }
    }
}

impl Uniform for Vec2<f32> {
    fn apply(&self, info: &UniformInfo) {
        unsafe {
            gl::Uniform2f(info.location, self.x, self.y);
        }
    }
}

impl Uniform for Vec2<usize> {
    fn apply(&self, info: &UniformInfo) {
        unsafe {
            gl::Uniform2i(info.location, self.x as GLint, self.y as GLint);
        }
    }
}

impl Uniform for Vec3<f32> {
    fn apply(&self, info: &UniformInfo) {
        unsafe {
            gl::Uniform3f(info.location, self.x, self.y, self.z);
        }
    }
}

impl Uniform for Mat4<f32> {
    fn apply(&self, info: &UniformInfo) {
        unsafe {
            gl::UniformMatrix4fv(info.location, 1, gl::FALSE, self as *const Self as *const _);
        }
    }
}

impl Uniform for Color {
    fn apply(&self, info: &UniformInfo) {
        unsafe {
            gl::Uniform4f(info.location, self.red, self.green, self.blue, self.alpha);
        }
    }
}

impl<P: Pixel> Uniform for Texture<P> {
    fn apply(&self, info: &UniformInfo) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + UNIFORM_TEXTURE_COUNT as GLenum);
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
            gl::Uniform1i(info.location, UNIFORM_TEXTURE_COUNT as GLint);
            UNIFORM_TEXTURE_COUNT += 1;
        }
    }
    fn walk_extra<C>(&self, name: &str, visitor: &mut C)
    where
        C: UniformVisitor,
    {
        visitor.visit(&(name.to_owned() + "_size"), &self.get_size());
    }
}

impl<'a, U: Uniform> Uniform for &'a U {
    fn apply(&self, info: &UniformInfo) {
        (*self).apply(info);
    }
    fn walk_extra<C>(&self, name: &str, visitor: &mut C)
    where
        C: UniformVisitor,
    {
        (*self).walk_extra(name, visitor);
    }
}

impl<U: Uniform> Uniform for Option<U> {
    fn apply(&self, info: &UniformInfo) {
        if let Some(ref value) = *self {
            value.apply(info);
        }
    }
    fn walk_extra<C>(&self, name: &str, visitor: &mut C)
    where
        C: UniformVisitor,
    {
        if let Some(ref value) = *self {
            value.walk_extra(name, visitor);
        }
    }
}
