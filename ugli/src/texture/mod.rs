use ::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum WrapMode {
    Repeat = gl::REPEAT as _,
    Clamp = gl::CLAMP_TO_EDGE as _,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Filter {
    Nearest = gl::NEAREST as _,
    Linear = gl::LINEAR as _,
}

#[derive(Debug)]
pub struct Texture<P: Pixel> {
    pub(crate) handle: GLuint,
    size: Cell<Vec2<usize>>,
    phantom_data: PhantomData<*mut P>,
}

impl<P: Pixel> Drop for Texture<P> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.handle);
        }
    }
}

pub type Texture2d = Texture<Color>;
pub type DepthTexture = Texture<DepthComponent>;

impl<P: Pixel> Texture<P> {
    fn new_raw(context: &Context, size: Vec2<usize>) -> Self {
        assert!(P::possible_texture(context));
        unsafe {
            let mut handle: GLuint = mem::uninitialized();
            gl::GenTextures(1, &mut handle);
            gl::BindTexture(gl::TEXTURE_2D, handle);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            let mut texture = Self {
                handle,
                size: Cell::new(size),
                phantom_data: PhantomData,
            };
            texture.set_filter(Filter::Linear);
            texture.set_wrap_mode(WrapMode::Clamp);
            texture
        }
    }

    pub fn is_pot(&self) -> bool {
        let size = self.size.get();
        size.x & (size.x - 1) == 0 && size.y & (size.y - 1) == 0
    }

    pub fn new_uninitialized(context: &Context, size: Vec2<usize>) -> Self {
        let texture = Self::new_raw(context, size);
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                P::GL_TEXTURE_FORMAT as GLint,
                size.x as GLsizei,
                size.y as GLsizei,
                0,
                P::GL_TEXTURE_FORMAT,
                P::GL_TEXTURE_TYPE,
                std::ptr::null(),
            );
        }
        texture
    }
    pub fn set_wrap_mode(&mut self, wrap_mode: WrapMode) {
        assert!(self.is_pot() || wrap_mode == WrapMode::Clamp);
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_mode as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_mode as GLint);
        }
    }

    pub fn set_filter(&mut self, filter: Filter) {
        assert!(self.is_pot() || filter == Filter::Nearest || filter == Filter::Linear);
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter as GLint);
        }
    }

    pub fn get_size(&self) -> Vec2<usize> {
        self.size.get()
    }

    pub fn _set_size(&self, size: Vec2<usize>) {
        self.size.set(size);
    }

    pub fn _get_handle(&self) -> GLuint {
        self.handle
    }
}

impl Texture2d {
    pub fn gen_mipmaps(&mut self) {
        assert!(self.is_pot());
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as GLint,
            );
        }
    }

    pub fn new_with<F: FnMut(Vec2<usize>) -> Color>(
        context: &Context,
        size: Vec2<usize>,
        mut f: F,
    ) -> Self {
        let texture = Texture2d::new_raw(context, size);
        let mut data: Vec<u8> = Vec::with_capacity(size.x * size.y * 4);
        for y in 0..size.y {
            for x in 0..size.x {
                let color = f(vec2(x, y));
                data.push((color.red * 255.0) as u8);
                data.push((color.green * 255.0) as u8);
                data.push((color.blue * 255.0) as u8);
                data.push((color.alpha * 255.0) as u8);
            }
        }
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                size.x as GLsizei,
                size.y as GLsizei,
                0,
                gl::RGBA as GLenum,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );
        }
        texture
    }

    // TODO: use like Matrix<Color>?
    pub unsafe fn sub_image(&mut self, pos: Vec2<usize>, size: Vec2<usize>, data: &[u8]) {
        gl::BindTexture(gl::TEXTURE_2D, self.handle);
        gl::TexSubImage2D(
            gl::TEXTURE_2D,
            0,
            pos.x as GLint,
            pos.y as GLint,
            size.x as GLsizei,
            size.y as GLsizei,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const _,
        );
    }

    #[cfg(not(target_os = "emscripten"))]
    pub fn from_image(context: &Context, image: image::RgbaImage) -> Self {
        let size = vec2(image.width() as usize, image.height() as usize);
        let mut texture = Texture2d::new_raw(context, size);
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                size.x as GLsizei,
                size.y as GLsizei,
                0,
                gl::RGBA as GLenum,
                gl::UNSIGNED_BYTE,
                image.into_raw().as_ptr() as *const _,
            );
        }
        texture.gen_mipmaps();
        texture
    }
}
