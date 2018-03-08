use ::*;

pub struct ShaderLib;

impl ShaderLib {
    pub fn new() -> Self {
        Self {}
    }
    pub fn process_separate(
        context: &Rc<ugli::Context>,
        vertex: &str,
        fragment: &str,
    ) -> ugli::Program {
        fn prepare(
            context: &Rc<ugli::Context>,
            vertex_type: ugli::ShaderType,
            source: &str,
        ) -> ugli::Shader {
            let sources = vec![
                #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
                "#version 150\n",
                #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
                "precision highp int;\nprecision highp float;\n",
                match vertex_type {
                    ugli::ShaderType::Vertex => "#define VERTEX_SHADER\n",
                    ugli::ShaderType::Fragment => "#define FRAGMENT_SHADER\n",
                },
                source,
            ];
            ugli::Shader::new(context, vertex_type, &sources).unwrap()
        }
        ugli::Program::new(
            context,
            &[
                &prepare(context, ugli::ShaderType::Vertex, vertex),
                &prepare(context, ugli::ShaderType::Fragment, fragment),
            ],
        ).unwrap()
    }
    pub fn process(context: &Rc<ugli::Context>, source: &str) -> ugli::Program {
        Self::process_separate(context, source, source)
    }
}
