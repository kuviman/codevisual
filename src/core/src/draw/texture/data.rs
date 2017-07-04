use commons::*;
use std;
use gl;
use gl::types::*;

pub struct TextureData<'a> {
    width: usize,
    height: usize,
    buffer: Vec<GLubyte>,
    phantom_data: PhantomData<&'a i32>,
}

impl<'a> TextureData<'a> {
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        assert!(x < self.width);
        assert!(y < self.height);
        Color::rgba(self.buffer[(y * self.width + x) * 4] as f32 / 255.0,
                    self.buffer[(y * self.width + x) * 4 + 1] as f32 / 255.0,
                    self.buffer[(y * self.width + x) * 4 + 2] as f32 / 255.0,
                    self.buffer[(y * self.width + x) * 4 + 3] as f32 / 255.0)
    }
}

impl super::Texture {
    pub fn get_data<'a>(&'a self) -> TextureData<'a> {
        let (width, height) = self.get_size();
        unsafe {
            let mut buffer =
                vec![std::mem::uninitialized::<GLubyte>(); (width * height * 4) as usize];
            let mut framebuffer: GLuint = std::mem::uninitialized();
            gl::GenFramebuffers(1, &mut framebuffer);
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER,
                                     gl::COLOR_ATTACHMENT0,
                                     gl::TEXTURE_2D,
                                     self.handle,
                                     0);
            assert!(gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE);
            gl::ReadPixels(0,
                           0,
                           width as GLsizei,
                           height as GLsizei,
                           gl::RGBA,
                           gl::UNSIGNED_BYTE,
                           buffer.as_mut_ptr() as *mut _);
            gl::DeleteFramebuffers(1, &framebuffer);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            TextureData {
                width,
                height,
                buffer,
                phantom_data: PhantomData,
            }
        }
    }
}