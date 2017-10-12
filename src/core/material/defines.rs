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

impl ShaderDefine for i32 {
    fn as_glsl(&self) -> String {
        format!("{}", self)
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
    fn as_glsl(&self, _: &mut Vec<String>) {}
}

impl<'a, D> ShaderDefineStorage for &'a D
    where
        D: ShaderDefineStorage,
{
    fn as_glsl(&self, sources: &mut Vec<String>) {
        (*self).as_glsl(sources);
    }
}

impl<A, B> ShaderDefineStorage for (A, B)
    where
        A: ShaderDefineStorage,
        B: ShaderDefineStorage,
{
    fn as_glsl(&self, sources: &mut Vec<String>) {
        self.0.as_glsl(sources);
        self.1.as_glsl(sources);
    }
}

#[derive(PartialEq, Clone)]
pub struct SingleShaderDefine<'a, D: ShaderDefine> {
    name: &'a str,
    value: D,
}

impl<'a, D: ShaderDefine> SingleShaderDefine<'a, D> {
    pub fn new(name: &'a str, value: D) -> Self {
        Self { name, value }
    }
    pub fn set_value(&mut self, value: D) {
        self.value = value;
    }
}

impl<'a, D: ShaderDefine> ShaderDefineStorage for SingleShaderDefine<'a, D> {
    fn as_glsl(&self, sources: &mut Vec<String>) {
        sources.push(format!("#define {} {}", self.name, self.value.as_glsl()));
    }
}
