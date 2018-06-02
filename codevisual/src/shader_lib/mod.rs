use *;

pub struct ShaderLib {
    context: Rc<ugli::Context>,
    files: HashMap<String, String>,
}

impl ShaderLib {
    pub fn new(context: &Rc<ugli::Context>) -> Self {
        let mut lib = Self {
            context: context.clone(),
            files: HashMap::new(),
        };
        lib.files.insert(
            String::from("prelude"),
            String::from(include_str!("include/prelude.glsl")),
        );
        lib
    }
    fn preprocess(&self, source: &str) -> Result<String, Error> {
        let mut result = String::new();
        for line in source.lines() {
            if line.starts_with("#include") {
                let mut iter = line.trim().split_whitespace();
                iter.next();
                let file = iter.next().expect("Expected path to include");
                assert!(iter.next().is_none(), "Unexpected token");
                assert!(
                    file.starts_with('<') && file.ends_with('>'),
                    "include path should be enclosed in angular brackets"
                );
                let file = file.trim_left_matches('<').trim_right_matches('>');
                if let Some(file) = self.files.get(file) {
                    result.push_str(&self.preprocess(file)?);
                } else {
                    bail!("{:?} not found in shader library", file);
                }
            } else {
                result.push_str(line);
                result.push_str("\n");
            }
        }
        Ok(result)
    }
    pub fn process(&self, shader_type: ugli::ShaderType, source: &str) -> Result<String, Error> {
        let mut result = String::new();
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        result.push_str("#version 150\n");
        #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
        result.push_str("precision highp int;\nprecision highp float;\n");
        result.push_str(match shader_type {
            ugli::ShaderType::Vertex => "#define VERTEX_SHADER\n",
            ugli::ShaderType::Fragment => "#define FRAGMENT_SHADER\n",
        });
        result.push_str(&self.preprocess("#include <prelude>")?);
        result.push_str(&self.preprocess(source)?);
        Ok(result)
    }
    pub fn compile(&self, source: &str) -> Result<ugli::Program, Error> {
        Ok(ugli::Program::new(
            &self.context,
            &[
                &ugli::Shader::new(
                    &self.context,
                    ugli::ShaderType::Vertex,
                    &self.process(ugli::ShaderType::Vertex, source)?,
                )?,
                &ugli::Shader::new(
                    &self.context,
                    ugli::ShaderType::Fragment,
                    &self.process(ugli::ShaderType::Fragment, source)?,
                )?,
            ],
        )?)
    }
}
