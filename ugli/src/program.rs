use ::*;

#[derive(Debug)]
pub struct Program {
    pub(crate) handle: GLuint,
    pub(crate) attributes: HashMap<String, AttributeInfo>,
    pub(crate) uniforms: HashMap<String, UniformInfo>,
    phantom_data: PhantomData<*mut ()>,
}

#[derive(Debug)]
pub struct AttributeInfo {
    pub(crate) location: GLuint,
}

#[derive(Debug)]
pub struct UniformInfo {
    pub(crate) location: GLint,
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.handle);
        }
    }
}

impl Program {
    pub fn new(_: &Context, shaders: &[&Shader]) -> Result<Self, String> {
        let mut program = Program {
            handle: {
                let handle = unsafe { gl::CreateProgram() };
                assert_ne!(handle, 0);
                handle
            },
            uniforms: HashMap::new(),
            attributes: HashMap::new(),
            phantom_data: PhantomData,
        };
        unsafe {
            for shader in shaders {
                gl::AttachShader(program.handle, shader.handle);
            }
            gl::LinkProgram(program.handle);
            for shader in shaders {
                gl::DetachShader(program.handle, shader.handle);
            }
        }

        // Check for errors
        unsafe {
            let mut link_status: GLint = mem::uninitialized();
            gl::GetProgramiv(program.handle, gl::LINK_STATUS, &mut link_status);
            if link_status == gl::FALSE as GLint {
                let mut info_log_length: GLint = mem::uninitialized();
                gl::GetProgramiv(program.handle, gl::INFO_LOG_LENGTH, &mut info_log_length);
                let mut info_log_bytes = vec![mem::uninitialized::<u8>(); info_log_length as usize];
                gl::GetProgramInfoLog(
                    program.handle,
                    info_log_bytes.len() as GLsizei,
                    std::ptr::null_mut(),
                    info_log_bytes.as_mut_ptr() as *mut _,
                );
                return Err(String::from_utf8(info_log_bytes).unwrap());
            }
        }
        // Get attributes
        unsafe {
            let mut attribute_count: GLint = mem::uninitialized();
            let mut attribute_max_length: GLint = mem::uninitialized();
            gl::GetProgramiv(program.handle, gl::ACTIVE_ATTRIBUTES, &mut attribute_count);
            gl::GetProgramiv(
                program.handle,
                gl::ACTIVE_ATTRIBUTE_MAX_LENGTH,
                &mut attribute_max_length,
            );
            let mut buf = vec![mem::uninitialized::<u8>(); attribute_max_length as usize];
            let mut length: GLsizei = mem::uninitialized();
            let mut size: GLint = mem::uninitialized();
            let mut typ: GLenum = mem::uninitialized();
            for index in 0..attribute_count {
                gl::GetActiveAttrib(
                    program.handle,
                    index as GLuint,
                    buf.len() as GLsizei,
                    &mut length,
                    &mut size,
                    &mut typ,
                    buf.as_mut_ptr() as *mut GLchar,
                );
                let name = std::str::from_utf8(&buf[..length as usize])
                    .unwrap()
                    .to_owned();
                // TODO: save & check type info
                let location = gl::GetAttribLocation(program.handle, buf.as_ptr() as *const _);
                assert!(location >= 0);
                program.attributes.insert(
                    name,
                    AttributeInfo {
                        location: location as GLuint,
                    },
                );
            }
        }
        // Get uniforms
        unsafe {
            let mut uniform_count: GLint = mem::uninitialized();
            let mut uniform_max_length: GLint = mem::uninitialized();
            gl::GetProgramiv(program.handle, gl::ACTIVE_UNIFORMS, &mut uniform_count);
            gl::GetProgramiv(
                program.handle,
                gl::ACTIVE_UNIFORM_MAX_LENGTH,
                &mut uniform_max_length,
            );
            let mut buf = vec![mem::uninitialized::<u8>(); uniform_max_length as usize];
            let mut length: GLsizei = mem::uninitialized();
            let mut size: GLint = mem::uninitialized();
            let mut typ: GLenum = mem::uninitialized();
            for index in 0..uniform_count {
                gl::GetActiveUniform(
                    program.handle,
                    index as GLuint,
                    buf.len() as GLsizei,
                    &mut length,
                    &mut size,
                    &mut typ,
                    buf.as_mut_ptr() as *mut GLchar,
                );
                let name = std::str::from_utf8(&buf[..length as usize])
                    .unwrap()
                    .to_owned();
                // TODO: save & check type info
                let location = gl::GetUniformLocation(program.handle, buf.as_ptr() as *const _);
                assert!(location >= 0);
                program.uniforms.insert(
                    name,
                    UniformInfo {
                        location: location as GLint,
                    },
                );
            }
        }
        Ok(program)
    }
    pub(crate) fn bind(&self) {
        unsafe {
            gl::UseProgram(self.handle);
        }
    }
}
