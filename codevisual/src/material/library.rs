use ::*;

pub trait ShaderLibrary {
    fn get(path: &str) -> Option<&str>;
}

pub struct ShaderPrelude;

impl ShaderLibrary for ShaderPrelude {
    fn get(path: &str) -> Option<&str> {
        match path {
            "prelude" => Some(include_str!("glsl/prelude.glsl")),
            "noise2d" => Some(include_str!("glsl/noise2d.glsl")),
            _ => None,
        }
    }
}

impl<A, B> ShaderLibrary for (A, B)
where
    A: ShaderLibrary,
    B: ShaderLibrary,
{
    fn get(path: &str) -> Option<&str> {
        if let Some(result) = A::get(path) {
            Some(result)
        } else {
            B::get(path)
        }
    }
}

pub struct PreprocessedShader<'a> {
    sources: Vec<&'a str>,
    included_headers: HashSet<&'a str>,
}

impl<'a> PreprocessedShader<'a> {
    pub fn new<Lib: ShaderLibrary>(sources: &[&'a str]) -> Self {
        let mut result = Self {
            sources: Vec::new(),
            included_headers: HashSet::new(),
        };
        for source in sources {
            result.preprocess::<Lib>(source);
        }
        result
    }

    pub fn preprocess<Lib: ShaderLibrary>(&mut self, source: &'a str) {
        for line in source.lines() {
            if line.starts_with("#include <") && line.ends_with('>') {
                let path = &line["#include <".len()..line.len() - ">".len()];
                if self.included_headers.insert(path) {
                    self.preprocess::<Lib>(
                        Lib::get(path).expect(&format!("{:?} not found in library", path)),
                    );
                }
            } else {
                self.sources.push(line);
                self.sources.push("\n");
                if line.starts_with("uniform sampler2D ") && line.ends_with(';') {
                    let name = &line["uniform sampler2D ".len()..line.len() - ";".len()];
                    self.sources.push("uniform ivec2 ");
                    self.sources.push(name);
                    self.sources.push("_size;\n");
                }
            }
        }
    }

    pub fn get_source(&self) -> String {
        std::slice::SliceConcatExt::concat(self.sources.as_slice())
    }
}
