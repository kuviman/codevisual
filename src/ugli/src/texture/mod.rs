use ::*;

pub enum WrapMode {
    Repeat,
    Clamp,
}

pub enum Filter {
    Nearest,
    Linear,
}

pub struct Texture2d {
    pub ( crate ) handle: GLuint,
    size: Cell<Vec2<usize>>,
}

impl Drop for Texture2d {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.handle);
        }
    }
}

impl Debug for Texture2d {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "Texture2d {{ size: {:?} }}", self.size.get())
    }
}

impl Texture2d {
    fn new_raw(size: Vec2<usize>) -> Self {
        unsafe {
            let mut handle: GLuint = std::mem::uninitialized();
            gl::GenTextures(1, &mut handle);
            gl::BindTexture(gl::TEXTURE_2D, handle);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            let mut texture = Self {
                handle,
                size: Cell::new(size),
            };
            texture.set_filter(Filter::Linear);
            texture.set_wrap_mode(WrapMode::Clamp);
            texture
        }
    }

    pub fn set_wrap_mode(&mut self, wrap_mode: WrapMode) {
        let wrap_mode = match wrap_mode {
            WrapMode::Clamp => gl::CLAMP_TO_EDGE,
            WrapMode::Repeat => gl::REPEAT,
        } as GLint;
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_mode);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_mode);
        }
    }

    pub fn set_filter(&mut self, filter: Filter) {
        let filter = match filter {
            Filter::Nearest => gl::NEAREST,
            Filter::Linear => gl::LINEAR,
        } as GLint;
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter);
        }
    }

    pub fn gen_mipmaps(&mut self) {
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

    pub fn new_uninitialized(_: &Context, size: Vec2<usize>) -> Self {
        let texture = Texture2d::new_raw(size);
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
                std::ptr::null(),
            );
        }
        texture
    }

    pub fn new_with<F: FnMut(Vec2<usize>) -> Color>(_: &Context, size: Vec2<usize>, mut f: F) -> Self {
        let texture = Texture2d::new_raw(size);
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

    #[cfg(not(target_os = "emscripten"))]
    pub fn from_image(_: &Context, image: image::RgbaImage) -> Self {
        let size = vec2(image.width() as usize, image.height() as usize);
        let mut texture = Texture2d::new_raw(size);
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
