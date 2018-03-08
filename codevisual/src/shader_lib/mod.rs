use ::*;

pub struct ShaderLib;

impl ShaderLib {
    pub fn new() -> Self {
        Self {}
    }
    pub fn process(context: &Rc<ugli::Context>, vertex: &str, fragment: &str) -> ugli::Program {
        fn prepare(source: &str) -> Vec<&str> {
            vec![
                #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
                "#version 150\n",
                #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
                "precision highp int;\nprecision highp float;\n",
                source,
            ]
        }
        ugli::Program::new(
            context,
            &[
                &ugli::Shader::new(context, ugli::ShaderType::Vertex, &prepare(vertex)).unwrap(),
                &ugli::Shader::new(context, ugli::ShaderType::Fragment, &prepare(fragment))
                    .unwrap(),
            ],
        ).unwrap()
    }
}
