use commons::*;
use super::*;
use gl::types::*;
use gl;

pub struct TextureTarget<'a> {
    fb: GLuint,
    size: (GLint, GLint),
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> ::draw::Target for TextureTarget<'a> {}

impl<'a> Drop for TextureTarget<'a> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.fb);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::Viewport(0, 0, self.size.0, self.size.1);
        }
    }
}

impl Texture {
    pub fn as_target<'a>(&'a mut self) -> TextureTarget<'a> {
        unsafe {
            let mut fb: GLuint = std::mem::uninitialized();
            gl::GenFramebuffers(1, &mut fb);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fb);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER,
                                     gl::COLOR_ATTACHMENT0,
                                     gl::TEXTURE_2D,
                                     self.handle,
                                     0);
            assert_eq!(gl::CheckFramebufferStatus(gl::FRAMEBUFFER),
                       gl::FRAMEBUFFER_COMPLETE);
            let (w, h) = self.get_size();
            let mut params = [0; 4];
            gl::GetIntegerv(gl::VIEWPORT, params.as_mut_ptr());
            gl::Viewport(0, 0, w as GLsizei, h as GLsizei);
            TextureTarget {
                fb,
                size: (params[2], params[3]),
                phantom_data: PhantomData,
            }
        }
    }
}