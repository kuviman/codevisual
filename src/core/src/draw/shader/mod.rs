use gl;
use gl::types::*;
use std;
use std::error::Error;

pub struct Shader {
    pub handle: GLuint,
}

#[derive(Debug)]
pub struct ShaderCompilationError {
    description: String,
}

impl Error for ShaderCompilationError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl std::fmt::Display for ShaderCompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Error::description(self))
    }
}

impl Shader {
    pub fn compile(_: &::Application,
                   vertex_shader: &str,
                   fragment_shader: &str)
                   -> Result<Self, ShaderCompilationError> {
        unsafe fn compile_shader(shader_type: GLuint,
                                 sources: &[&str])
                                 -> Result<GLuint, ShaderCompilationError> {
            let handle = gl::CreateShader(shader_type);
            let source_ptrs: Vec<*const GLchar> = sources
                .into_iter()
                .map(|source| source.as_ptr() as *const _)
                .collect();
            let lengths: Vec<GLint> = sources
                .into_iter()
                .map(|source| source.len() as GLint)
                .collect();
            gl::ShaderSource(handle,
                             sources.len() as GLsizei,
                             source_ptrs.as_ptr(),
                             lengths.as_ptr());
            gl::CompileShader(handle);
            let mut compile_status: GLint = std::mem::uninitialized();
            gl::GetShaderiv(handle, gl::COMPILE_STATUS, &mut compile_status);
            if compile_status == gl::FALSE as GLint {
                let mut info_log_length: GLint = std::mem::uninitialized();
                gl::GetShaderiv(handle, gl::INFO_LOG_LENGTH, &mut info_log_length);
                let mut info_log_bytes =
                    vec![std::mem::uninitialized::<u8>(); info_log_length as usize];
                gl::GetShaderInfoLog(handle,
                                     info_log_bytes.len() as GLsizei,
                                     std::ptr::null_mut(),
                                     info_log_bytes.as_mut_ptr() as *mut _);
                return Err(ShaderCompilationError {
                               description: String::from_utf8(info_log_bytes).unwrap(),
                           });
            }
            Ok(handle)
        }

        #[cfg(target_os = "emscripten")]
        let fragment_sources = vec!["precision mediump float;", fragment_shader];

        #[cfg(not(target_os = "emscripten"))]
        let fragment_sources = vec![fragment_shader];

        unsafe {
            let vertex_shader_handle = try!(compile_shader(gl::VERTEX_SHADER, &[vertex_shader]));
            let fragment_shader_handle = try!(compile_shader(gl::FRAGMENT_SHADER,
                                                             fragment_sources.as_slice()));
            let handle = gl::CreateProgram();
            gl::AttachShader(handle, vertex_shader_handle);
            gl::AttachShader(handle, fragment_shader_handle);
            gl::LinkProgram(handle);
            let mut link_status: GLint = std::mem::uninitialized();
            gl::GetProgramiv(handle, gl::LINK_STATUS, &mut link_status);
            if link_status == gl::FALSE as GLint {
                let mut info_log_length: GLint = std::mem::uninitialized();
                gl::GetProgramiv(handle, gl::INFO_LOG_LENGTH, &mut info_log_length);
                let mut info_log_bytes =
                    vec![std::mem::uninitialized::<u8>(); info_log_length as usize];
                gl::GetProgramInfoLog(handle,
                                      info_log_bytes.len() as GLsizei,
                                      std::ptr::null_mut(),
                                      info_log_bytes.as_mut_ptr() as *mut _);
                return Err(ShaderCompilationError {
                               description: String::from_utf8(info_log_bytes).unwrap(),
                           });
            }
            Ok(Self { handle })
        }
    }
}