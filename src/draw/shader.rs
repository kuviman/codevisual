use gl;
use gl::types::*;
use std;

pub struct Shader {
    pub handle: GLuint,
}

impl Shader {
    pub fn compile(vertex_shader: &str, fragment_shader: &str) -> Result<Self, String> {
        ::init().unwrap();

        unsafe fn compile_shader(shader_type: GLuint, source: &str) -> Result<GLuint, String> {
            let handle = gl::CreateShader(shader_type);
            gl::ShaderSource(handle,
                             1,
                             [source.as_ptr()].as_ptr(),
                             [source.len() as GLint].as_ptr());
            gl::CompileShader(handle);
            let mut compile_status: GLint = std::mem::uninitialized();
            gl::GetShaderiv(handle, gl::COMPILE_STATUS, &mut compile_status);
            if compile_status == gl::FALSE as GLint {
                let mut info_log_length: GLint = std::mem::uninitialized();
                gl::GetShaderiv(handle, gl::INFO_LOG_LENGTH, &mut info_log_length);
                let mut info_log_bytes =
                    vec![std::mem::uninitialized::<GLchar>(); info_log_length as usize];
                gl::GetShaderInfoLog(handle,
                                     info_log_bytes.len() as GLsizei,
                                     std::ptr::null_mut(),
                                     info_log_bytes.as_mut_ptr());
                return Err(String::from_utf8(info_log_bytes).unwrap());
            }
            Ok(handle)
        }

        unsafe {
            let vertex_shader_handle = try!(compile_shader(gl::VERTEX_SHADER, vertex_shader));
            let fragment_shader_handle = try!(compile_shader(gl::FRAGMENT_SHADER, fragment_shader));
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
                    vec![std::mem::uninitialized::<GLchar>(); info_log_length as usize];
                gl::GetProgramInfoLog(handle,
                                      info_log_bytes.len() as GLsizei,
                                      std::ptr::null_mut(),
                                      info_log_bytes.as_mut_ptr());
                return Err(String::from_utf8(info_log_bytes).unwrap());
            }
            Ok(Self { handle })
        }
    }
}