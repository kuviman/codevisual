use ::*;

pub trait ShaderDefine {
    fn into_code(&self) -> String;
}

impl ShaderDefine for f32 {
    fn into_code(&self) -> String {
        format!("float({})", self)
    }
}

impl ShaderDefine for bool {
    fn into_code(&self) -> String {
        format!("{}", *self as u8)
    }
}

impl ShaderDefine for () {
    fn into_code(&self) -> String {
        String::new()
    }
}

impl<'a, D: ShaderDefine> ShaderDefine for &'a D {
    fn into_code(&self) -> String {
        (*self).into_code()
    }
}

pub trait ShaderDefineStorage {
    fn into_code(&self, sources: &mut Vec<String>);
}

impl ShaderDefineStorage for () {
    fn into_code(&self, sources: &mut Vec<String>) {}
}

impl<'a, A, B> ShaderDefineStorage for (&'a A, &'a B)
where
    A: ShaderDefineStorage,
    B: ShaderDefineStorage,
{
    fn into_code(&self, sources: &mut Vec<String>) {
        self.0.into_code(sources);
        self.1.into_code(sources);
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
    fn into_code(&self, sources: &mut Vec<String>) {
        sources.push(format!("#define {} {}", self.name, self.value.into_code()));
    }
}
