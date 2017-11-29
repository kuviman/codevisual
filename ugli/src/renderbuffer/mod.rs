use ::*;

pub struct Renderbuffer<T: Pixel = Color> {
    pub ( crate ) handle: GLuint,
    phantom_data: PhantomData<*mut T>,
}

impl<T: Pixel> Drop for Renderbuffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteRenderbuffers(1, &self.handle);
        }
    }
}

impl<T: Pixel> Renderbuffer<T> {
    pub fn new(size: Vec2<usize>) -> Self {
        unsafe {
            let mut handle: GLuint = mem::uninitialized();
            gl::GenRenderbuffers(1, &mut handle);
            gl::BindRenderbuffer(gl::RENDERBUFFER, handle);
            gl::RenderbufferStorage(gl::RENDERBUFFER, T::GL_FRAMEBUFFER_FORMAT, size.x as GLsizei, size.y as GLsizei);
            Self {
                handle,
                phantom_data: PhantomData,
            }
        }
    }
}