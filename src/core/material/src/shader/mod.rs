use ::*;

mod library;
pub use self::library::*;

mod defines;
pub use self::defines::*;

pub struct Shader {
    ugli_program: ugli::Program,
}

impl Shader {
    pub fn compile<Lib>(
        ugli_context: &ugli::Context,
        defines: &ShaderDefineStorage,
        source: &str,
    ) -> Shader
    where
        Lib: ShaderLibrary,
    {
        #[cfg(target_os = "emscripten")]
        const PRELUDE_INCLUDE: &str = "#define EMSCRIPTEN\n#include <prelude>";
        #[cfg(not(target_os = "emscripten"))]
        const PRELUDE_INCLUDE: &str = "#include <prelude>";

        let mut defines_sources = Vec::new();
        let mut sources = vec!["#define VERTEX"];
        defines.into_code(&mut defines_sources);
        for define in &defines_sources {
            sources.push(define);
        }
        sources.push(PRELUDE_INCLUDE);
        sources.push(source);
        let vertex_shader = ugli::Shader::new(
            ugli_context,
            ugli::ShaderType::Vertex,
            PreprocessedShader::new::<Lib>(&sources).get_sources(),
        ).unwrap();

        sources[0] = "#define FRAGMENT";
        let fragment_shader = ugli::Shader::new(
            ugli_context,
            ugli::ShaderType::Fragment,
            PreprocessedShader::new::<Lib>(&sources).get_sources(),
        ).unwrap();

        Shader {
            ugli_program: ugli::Program::new(ugli_context, &[&vertex_shader, &fragment_shader])
                .unwrap(),
        }
    }

    pub fn ugli_program(&self) -> &ugli::Program {
        &self.ugli_program
    }
}
