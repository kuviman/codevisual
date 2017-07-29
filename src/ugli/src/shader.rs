use ::*;

pub enum ShaderType {
    Vertex,
    Fragment,
}

pub struct Shader {
    pub(crate) handle: GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.handle);
        }
    }
}

#[derive(Debug)]
pub enum ShaderCreationError {
    Unknown,
    CompilationError { description: String },
}

impl Error for ShaderCreationError {
    fn description(&self) -> &str {
        use ShaderCreationError::*;
        match *self {
            Unknown => "Unknown",
            CompilationError { ref description } => description,
        }
    }
}

display_error_description!(ShaderCreationError);

impl Shader {
    pub fn new(
        _: &Context,
        shader_type: ShaderType,
        sources: &[&str],
    ) -> Result<Self, ShaderCreationError> {
        let shader = Self {
            handle: {
                let handle = unsafe {
                    gl::CreateShader(match shader_type {
                        ShaderType::Vertex => gl::VERTEX_SHADER,
                        ShaderType::Fragment => gl::FRAGMENT_SHADER,
                    })
                };
                if handle == 0 {
                    return Err(ShaderCreationError::Unknown);
                }
                handle
            },
        };
        let source_ptrs: Vec<*const GLchar> = sources
            .into_iter()
            .map(|source| source.as_ptr() as *const GLchar)
            .collect();
        let lengths: Vec<GLint> = sources
            .into_iter()
            .map(|source| source.len() as GLint)
            .collect();
        unsafe {
            gl::ShaderSource(
                shader.handle,
                sources.len() as GLsizei,
                source_ptrs.as_ptr(),
                lengths.as_ptr(),
            );
            gl::CompileShader(shader.handle);
            let mut compile_status: GLint = std::mem::uninitialized();
            gl::GetShaderiv(shader.handle, gl::COMPILE_STATUS, &mut compile_status);
            if compile_status == gl::FALSE as GLint {
                let mut info_log_length: GLint = std::mem::uninitialized();
                gl::GetShaderiv(shader.handle, gl::INFO_LOG_LENGTH, &mut info_log_length);
                let mut info_log_bytes =
                    vec![std::mem::uninitialized::<u8>(); info_log_length as usize];
                gl::GetShaderInfoLog(
                    shader.handle,
                    info_log_bytes.len() as GLsizei,
                    std::ptr::null_mut(),
                    info_log_bytes.as_mut_ptr() as *mut _,
                );
                return Err(ShaderCreationError::CompilationError {
                    description: String::from_utf8(info_log_bytes).unwrap(),
                });
            }
        }
        Ok(shader)
    }
}
