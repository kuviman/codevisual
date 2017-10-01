#![deny(warnings)]

extern crate prelude;

pub ( crate ) use prelude::*;

extern crate ugli;

mod defines;

pub use defines::*;

mod library;

pub use library::*;

pub fn compile_ugli_program<Lib>(
    ugli_context: &ugli::Context,
    defines: &ShaderDefineStorage,
    source: &str,
) -> ugli::Program
    where
        Lib: ShaderLibrary,
{
    #[cfg(target_os = "emscripten")]
    const PRELUDE_INCLUDE: &str = "#define EMSCRIPTEN\n#include <prelude>";
    #[cfg(not(target_os = "emscripten"))]
    const PRELUDE_INCLUDE: &str = "#include <prelude>";

    let mut defines_sources = Vec::new();
    let mut sources = vec!["#define VERTEX"];
    defines.as_glsl(&mut defines_sources);
    for define in &defines_sources {
        sources.push(define);
    }
    sources.push(PRELUDE_INCLUDE);
    sources.push(source);
    let vertex_shader = ugli::Shader::new(
        ugli_context,
        ugli::ShaderType::Vertex,
        &[&PreprocessedShader::new::<Lib>(&sources).get_source()],
    ).expect("Could not compile vertex shader");

    sources[0] = "#define FRAGMENT";
    let fragment_shader = ugli::Shader::new(
        ugli_context,
        ugli::ShaderType::Fragment,
        &[&PreprocessedShader::new::<Lib>(&sources).get_source()],
    ).expect("Could not compile fragment shader");

    ugli::Program::new(ugli_context, &[&vertex_shader, &fragment_shader]).expect("Could not link program")
}

pub struct Material<Lib = ShaderPrelude, U = (), D = ()>
    where
        Lib: ShaderLibrary,
        U: ugli::Uniforms,
        D: ShaderDefineStorage + Clone + PartialEq,
{
    pub uniforms: U,
    pub defines: D,
    last_defines: RefCell<D>,
    program: RefCell<ugli::Program>,
    program_source: String,
    ugli_context: Rc<ugli::Context>,
    phantom_data: PhantomData<Lib>,
}

impl<Lib, U, D> Material<Lib, U, D>
    where
        Lib: ShaderLibrary,
        U: ugli::Uniforms,
        D: ShaderDefineStorage + Clone + PartialEq,
{
    pub fn new(
        context: &Rc<ugli::Context>,
        uniforms: U,
        defines: D,
        program_source: &str,
    ) -> Self {
        let program_source = String::from(program_source);
        let last_defines = RefCell::new(defines.clone());
        let program = RefCell::new(compile_ugli_program::<Lib>(context, &defines, &program_source));
        Self {
            uniforms,
            defines,
            last_defines,
            program,
            program_source,
            ugli_context: context.clone(),
            phantom_data: PhantomData,
        }
    }

    pub fn ugli_program(&self) -> Ref<ugli::Program> {
        if *self.last_defines.borrow() != self.defines {
            *self.last_defines.borrow_mut() = self.defines.clone();
            *self.program.borrow_mut() =
                compile_ugli_program::<Lib>(&self.ugli_context, &self.defines, &self.program_source);
        }
        self.program.borrow()
    }
}