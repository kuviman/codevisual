use ::*;

pub trait Access {}
pub struct HasAccess;
impl Access for HasAccess {}
pub struct NoAccess;
impl Access for NoAccess {}

pub trait Attachment {
    fn attach(&self);
    fn get_size(&self) -> Option<Vec2<usize>>;
}

impl<T: Deref<Target = Texture2d>> Attachment for T {
    fn attach(&self) {
        unsafe {
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                self.handle,
                0,
            );
        }
    }
    fn get_size(&self) -> Option<Vec2<usize>> {
        Some((**self).get_size())
    }
}

pub trait Color: Attachment {
    type ReadAccess: Access;
    type WriteAccess: Access;
}

pub struct DefaultColor;
impl Attachment for DefaultColor {
    fn attach(&self) {}
    fn get_size(&self) -> Option<Vec2<usize>> {
        None
    }
}
impl Color for DefaultColor {
    type ReadAccess = HasAccess;
    type WriteAccess = HasAccess;
}

pub trait Texture: Color {
    fn get_texture(&self) -> &Texture2d;
}

impl<'a> Color for &'a Texture2d {
    type ReadAccess = HasAccess;
    type WriteAccess = NoAccess;
}

impl<'a> Texture for &'a Texture2d {
    fn get_texture(&self) -> &Texture2d {
        self
    }
}

impl<'a> Color for &'a mut Texture2d {
    type ReadAccess = HasAccess;
    type WriteAccess = HasAccess;
}

impl<'a> Texture for &'a mut Texture2d {
    fn get_texture(&self) -> &Texture2d {
        self
    }
}
