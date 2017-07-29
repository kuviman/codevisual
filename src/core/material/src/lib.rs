extern crate vpl;
use vpl::*;

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
