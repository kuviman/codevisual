use ::*;

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
    fn preprocess<'a>(&'a self, source: &'a str) -> Result<Vec<&'a str>, Error> {
        let mut result = Vec::new();
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
                    result.extend(self.preprocess(file)?);
                } else {
                    bail!("{:?} not found in shader library", file);
                }
            } else {
                result.push(line);
                result.push("\n");
            }
        }
        Ok(result)
    }
    pub fn process<'a>(
        &'a self,
        shader_type: ugli::ShaderType,
        source: &'a str,
    ) -> Result<Vec<&'a str>, Error> {
        let mut result = vec![
            #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
            "#version 150\n",
            #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
            "precision highp int;\nprecision highp float;\n",
            match shader_type {
                ugli::ShaderType::Vertex => "#define VERTEX_SHADER\n",
                ugli::ShaderType::Fragment => "#define FRAGMENT_SHADER\n",
            },
        ];
        result.extend(self.preprocess("#include <prelude>")?);
        result.extend(self.preprocess(source)?);
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
