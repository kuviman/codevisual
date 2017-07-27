use ::*;

mod fbo;
pub(crate) use self::fbo::*;

pub(crate) mod attachment;
use self::attachment::{Access, HasAccess};

mod read;
pub use self::read::*;

pub struct Framebuffer<ReadAccess, WriteAccess, Color>
    where ReadAccess: Access,
          WriteAccess: Access,
          Color: attachment::Color<ReadAccess = ReadAccess, WriteAccess = WriteAccess>
{
    pub(crate) fbo: FBO,
    #[allow(dead_code)]
    color: Color,
    size: Vec2<usize>,
}

impl<ReadAccess, WriteAccess, Color> Framebuffer<ReadAccess, WriteAccess, Color>
    where ReadAccess: Access,
          WriteAccess: Access,
          Color: attachment::Color<ReadAccess = ReadAccess, WriteAccess = WriteAccess>
{
    pub fn new_color(context: &Context, color: Color) -> Self {
        let fbo = FBO::new(context);
        fbo.bind();
        color.attach();
        fbo.check();
        let size = color.get_size().unwrap();
        Self { fbo, color, size }
    }
    pub fn get_size(&self) -> Vec2<usize> {
        self.size
    }
}

pub type DefaultFramebuffer = Framebuffer<HasAccess, HasAccess, attachment::DefaultColor>;

pub fn default_framebuffer(context: &Context) -> DefaultFramebuffer {
    DefaultFramebuffer {
        fbo: FBO::default(),
        color: attachment::DefaultColor,
        size: context.get_size(),
    }
}