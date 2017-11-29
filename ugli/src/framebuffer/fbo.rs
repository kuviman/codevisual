use ::*;

pub struct FBO {
    pub handle: GLuint,
    phantom_data: PhantomData<*mut ()>,
}

impl Default for FBO {
    fn default() -> Self {
        Self {
            handle: 0,
            phantom_data: PhantomData,
        }
    }
}

impl FBO {
    pub fn new(_: &Context) -> Self {
        Self {
            handle: unsafe {
                let mut handle: GLuint = mem::uninitialized();
                gl::GenFramebuffers(1, &mut handle);
                handle
            },
            phantom_data: PhantomData,
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.handle);
        }
    }
    pub fn check(&self) {
        unsafe {
            assert_eq!(
                gl::CheckFramebufferStatus(gl::FRAMEBUFFER),
                gl::FRAMEBUFFER_COMPLETE,
                "Framebuffer check failed"
            );
        }
    }
}

impl Drop for FBO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.handle);
        }
    }
}
