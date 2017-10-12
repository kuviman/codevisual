use ::*;

mod defines;
mod library;

pub use self::defines::*;
pub use self::library::*;

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
    let mut sources: Vec<&str> = Vec::new();
    defines.as_glsl(&mut defines_sources);
    for define in &defines_sources {
        sources.push(define);
    }
    sources.push(PRELUDE_INCLUDE);
    sources.push(source);

    fn compile_shader(ugli_context: &ugli::Context, source: &str, typ: ugli::ShaderType) -> ugli::Shader {
        match ugli::Shader::new(
            ugli_context,
            typ,
            &[match typ {
                ugli::ShaderType::Vertex => "#define VERTEX\n",
                ugli::ShaderType::Fragment => "#define FRAGMENT\n"
            }, source]) {
            Ok(shader) => shader,
            Err(error) => {
                eprintln!("Shader source:");
                eprintln!("{}", source);
                panic!("Could not compile {:?} shader: {:?}", typ, error);
            }
        }
    }

    let source = PreprocessedShader::new::<Lib>(&sources).get_source();
    let vertex_shader = compile_shader(ugli_context, &source, ugli::ShaderType::Vertex);
    let fragment_shader = compile_shader(ugli_context, &source, ugli::ShaderType::Fragment);

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