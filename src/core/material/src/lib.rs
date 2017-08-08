extern crate vpl;

pub ( crate ) use vpl::*;

extern crate ugli;

#[macro_use]
extern crate codevisual_derive;

mod shader;

pub use shader::*;

pub trait Material {
    type Uniforms: ugli::UniformStorage;
    fn get_uniforms(&self) -> &Self::Uniforms;
    fn get_shader(&self) -> &Shader;
}

pub struct LazyMaterial<Lib = ShaderPrelude, U = (), D = ()>
    where
        Lib: ShaderLibrary,
        U: ugli::UniformStorage,
        D: ShaderDefineStorage + Clone + PartialEq,
{
    pub uniforms: U,
    pub defines: D,
    last_defines: RefCell<D>,
    shader: RefCell<Shader>,
    shader_source: String,
    ugli_context: Rc<ugli::Context>,
    phantom_data: PhantomData<Lib>,
}

impl<Lib, U, D> LazyMaterial<Lib, U, D>
    where
        Lib: ShaderLibrary,
        U: ugli::UniformStorage,
        D: ShaderDefineStorage + Clone + PartialEq,
{
    pub fn new<S: Into<String>>(
        context: &Rc<ugli::Context>,
        uniforms: U,
        defines: D,
        shader_source: S,
    ) -> Self {
        let shader_source = shader_source.into();
        let last_defines = RefCell::new(defines.clone());
        let shader = RefCell::new(Shader::compile::<Lib>(context, &defines, &shader_source));
        Self {
            uniforms,
            defines,
            last_defines,
            shader,
            shader_source,
            ugli_context: context.clone(),
            phantom_data: PhantomData,
        }
    }

    pub fn get_shader(&self) -> Ref<Shader> {
        if *self.last_defines.borrow() != self.defines {
            *self.last_defines.borrow_mut() = self.defines.clone();
            *self.shader.borrow_mut() =
                Shader::compile::<Lib>(&self.ugli_context, &self.defines, &self.shader_source);
        }
        self.shader.borrow()
    }
}