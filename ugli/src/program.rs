use ::*;

pub struct Program {
    pub ( crate ) handle: GLuint,
    uniforms: RefCell<HashMap<String, GLint>>,
    attributes: RefCell<HashMap<String, GLint>>,
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.handle);
        }
    }
}

#[derive(Debug)]
pub enum ProgramCreationError {
    Unknown,
    LinkError { description: String },
}

impl Error for ProgramCreationError {
    fn description(&self) -> &str {
        use ProgramCreationError::*;
        match *self {
            Unknown => "Unknown",
            LinkError { ref description } => description,
        }
    }
}

display_error_description!(ProgramCreationError);

impl Program {
    pub fn new(_: &Context, shaders: &[&Shader]) -> Result<Self, ProgramCreationError> {
        let program = Self {
            handle: {
                let handle = unsafe { gl::CreateProgram() };
                if handle == 0 {
                    return Err(ProgramCreationError::Unknown);
                }
                handle
            },
            uniforms: RefCell::new(HashMap::new()),
            attributes: RefCell::new(HashMap::new()),
        };
        unsafe {
            for shader in shaders {
                gl::AttachShader(program.handle, shader.handle);
            }
            gl::LinkProgram(program.handle);
            for shader in shaders {
                gl::DetachShader(program.handle, shader.handle);
            }
            let mut link_status: GLint = mem::uninitialized();
            gl::GetProgramiv(program.handle, gl::LINK_STATUS, &mut link_status);
            if link_status == gl::FALSE as GLint {
                let mut info_log_length: GLint = mem::uninitialized();
                gl::GetProgramiv(program.handle, gl::INFO_LOG_LENGTH, &mut info_log_length);
                let mut info_log_bytes =
                    vec![mem::uninitialized::<u8>(); info_log_length as usize];
                gl::GetProgramInfoLog(
                    program.handle,
                    info_log_bytes.len() as GLsizei,
                    std::ptr::null_mut(),
                    info_log_bytes.as_mut_ptr() as *mut _,
                );
                return Err(ProgramCreationError::LinkError {
                    description: String::from_utf8(info_log_bytes).unwrap(),
                });
            }
        }
        Ok(program)
    }
    pub ( crate ) fn bind(&self) {
        unsafe {
            gl::UseProgram(self.handle);
        }
    }
    pub ( crate ) fn get_uniform_location(&self, name: &str) -> GLint {
        let mut uniforms = self.uniforms.borrow_mut();
        if let Some(&location) = uniforms.get(name) {
            location
        } else {
            let location = unsafe {
                gl::GetUniformLocation(
                    self.handle,
                    std::ffi::CString::new(name).unwrap().as_ptr(),
                )
            };
            uniforms.insert(String::from(name), location);
            location
        }
    }
    pub ( crate ) fn get_attribute_location(&self, name: &str) -> GLint {
        let mut attributes = self.attributes.borrow_mut();
        if let Some(&location) = attributes.get(name) {
            location
        } else {
            let location = unsafe {
                gl::GetAttribLocation(
                    self.handle,
                    std::ffi::CString::new(name).unwrap().as_ptr(),
                )
            };
            attributes.insert(String::from(name), location);
            location
        }
    }
}