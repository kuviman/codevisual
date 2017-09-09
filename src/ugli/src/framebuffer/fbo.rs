use ::*;

pub struct FBO {
    pub handle: GLuint,
}

impl Default for FBO {
    fn default() -> Self {
        Self { handle: 0 }
    }
}

impl FBO {
    pub fn new(_: &Context) -> Self {
        Self {
            handle: unsafe {
                let mut handle: GLuint = std::mem::uninitialized();
                gl::GenFramebuffers(1, &mut handle);
                handle
            },
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
