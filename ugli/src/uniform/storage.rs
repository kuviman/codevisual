use ::*;

pub trait Uniforms {
    fn walk_uniforms<C>(&self, visitor: &mut C)
    where
        C: UniformVisitor;
}

impl Uniforms for () {
    fn walk_uniforms<C>(&self, _: &mut C)
    where
        C: UniformVisitor,
    {
    }
}

pub struct SingleUniform<'a, U: Uniform> {
    name: &'a str,
    value: U,
}

impl<'a, U: Uniform> SingleUniform<'a, U> {
    pub fn new(name: &'a str, value: U) -> Self {
        Self { name, value }
    }
}

impl<'a, U: Uniform> Uniforms for SingleUniform<'a, U> {
    fn walk_uniforms<C>(&self, visitor: &mut C)
    where
        C: UniformVisitor,
    {
        visitor.visit(self.name, &self.value);
    }
}

impl<'a, U: Uniforms> Uniforms for &'a U {
    fn walk_uniforms<C>(&self, visitor: &mut C)
    where
        C: UniformVisitor,
    {
        (*self).walk_uniforms(visitor);
    }
}

impl<A: Uniforms, B: Uniforms> Uniforms for (A, B) {
    fn walk_uniforms<C>(&self, visitor: &mut C)
    where
        C: UniformVisitor,
    {
        self.0.walk_uniforms(visitor);
        self.1.walk_uniforms(visitor);
    }
}

impl<U: Uniforms> Uniforms for Option<U> {
    fn walk_uniforms<C>(&self, visitor: &mut C)
    where
        C: UniformVisitor,
    {
        self.as_ref().map(|u| u.walk_uniforms(visitor));
    }
}
