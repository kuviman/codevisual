use ::*;

use super::attachment::{self, Access, HasAccess};

pub struct ColorData<'a> {
    width: usize,
    height: usize,
    buffer: Vec<GLubyte>,
    phantom_data: PhantomData<&'a i32>,
}

impl<'a> ColorData<'a> {
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        assert!(x < self.width);
        assert!(y < self.height);
        Color::rgba(
            self.buffer[(y * self.width + x) * 4] as f32 / 255.0,
            self.buffer[(y * self.width + x) * 4 + 1] as f32 / 255.0,
            self.buffer[(y * self.width + x) * 4 + 2] as f32 / 255.0,
            self.buffer[(y * self.width + x) * 4 + 3] as f32 / 255.0,
        )
    }
}

impl<WriteAccess, Color> Framebuffer<HasAccess, WriteAccess, Color>
    where WriteAccess: Access,
          Color: attachment::Color<ReadAccess = HasAccess, WriteAccess = WriteAccess>
{
    pub fn read_color<'a>(&'a self) -> ColorData<'a> {
        self.fbo.bind();
        unsafe {
            let mut buffer =
                vec![std::mem::uninitialized::<GLubyte>(); self.size.x * self.size.y * 4];
            gl::ReadPixels(0,
                           0,
                           self.size.x as GLsizei,
                           self.size.y as GLsizei,
                           gl::RGBA,
                           gl::UNSIGNED_BYTE,
                           buffer.as_mut_ptr() as *mut _);
            ColorData {
                width: self.size.x,
                height: self.size.y,
                buffer,
                phantom_data: PhantomData,
            }
        }
    }
}
