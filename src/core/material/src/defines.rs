use ::*;

pub trait ShaderDefine {
    fn as_glsl(&self) -> String;
}

impl ShaderDefine for f32 {
    fn as_glsl(&self) -> String {
        format!("float({})", self)
    }
}

impl ShaderDefine for bool {
    fn as_glsl(&self) -> String {
        format!("{}", *self as u8)
    }
}

impl ShaderDefine for () {
    fn as_glsl(&self) -> String {
        String::new()
    }
}

impl<'a, D: ShaderDefine> ShaderDefine for &'a D {
    fn as_glsl(&self) -> String {
        (*self).as_glsl()
    }
}

pub trait ShaderDefineStorage {
    fn as_glsl(&self, sources: &mut Vec<String>);
}

impl ShaderDefineStorage for () {
    fn as_glsl(&self, sources: &mut Vec<String>) {}
}

impl<'a, D> ShaderDefineStorage for &'a D
    where
        D: ShaderDefineStorage,
{
    fn as_glsl(&self, sources: &mut Vec<String>) {
        (*self).as_glsl(sources);
    }
}

impl<'a, A, B> ShaderDefineStorage for (&'a A, &'a B)
    where
        A: ShaderDefineStorage,
        B: ShaderDefineStorage,
{
    fn as_glsl(&self, sources: &mut Vec<String>) {
        self.0.as_glsl(sources);
        self.1.as_glsl(sources);
    }
}

pub struct SingleShaderDefine<'a, D: ShaderDefine> {
    name: &'a str,
    value: D,
}

impl<'a, D: ShaderDefine> SingleShaderDefine<'a, D> {
    pub fn new(name: &'a str, value: D) -> Self {
        Self { name, value }
    }
}

impl<'a, D: ShaderDefine> ShaderDefineStorage for SingleShaderDefine<'a, D> {
    fn as_glsl(&self, sources: &mut Vec<String>) {
        sources.push(format!("#define {} {}", self.name, self.value.as_glsl()));
    }
}
