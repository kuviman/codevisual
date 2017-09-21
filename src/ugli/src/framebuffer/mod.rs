use ::*;

mod fbo;

pub ( crate ) use self::fbo::*;

mod read;

pub use self::read::*;

pub enum ColorAttachmentRead<'a> {
    None,
    Texture(&'a Texture2d),
}

pub enum DepthAttachmentRead<'a> {
    None,
    Renderbuffer(&'a Renderbuffer<DepthComponent>),
    Texture(&'a DepthTexture),
}

pub struct FramebufferRead<'a> {
    pub ( crate ) fbo: FBO,
    color: ColorAttachmentRead<'a>,
    size: Vec2<usize>,
}

impl<'a> FramebufferRead<'a> {
    pub fn new(context: &Context, color: ColorAttachmentRead<'a>, depth: DepthAttachmentRead<'a>) -> Self {
        let fbo = FBO::new(context);
        fbo.bind();
        let mut size = None;
        match color {
            ColorAttachmentRead::None => {}
            ColorAttachmentRead::Texture(ref texture) => {
                unsafe {
                    gl::FramebufferTexture2D(
                        gl::FRAMEBUFFER,
                        gl::COLOR_ATTACHMENT0,
                        gl::TEXTURE_2D,
                        texture.handle,
                        0,
                    );
                }
                size = Some(texture.get_size());
            }
        }
        match depth {
            DepthAttachmentRead::None => {}
            DepthAttachmentRead::Renderbuffer(ref renderbuffer) => {
                unsafe {
                    gl::FramebufferRenderbuffer(
                        gl::FRAMEBUFFER,
                        gl::DEPTH_ATTACHMENT,
                        gl::RENDERBUFFER,
                        renderbuffer.handle
                    );
                }
                // TODO: update/check size
            }
            DepthAttachmentRead::Texture(ref texture) => {
                unsafe {
                    gl::FramebufferTexture2D(
                        gl::FRAMEBUFFER,
                        gl::DEPTH_ATTACHMENT,
                        gl::TEXTURE_2D,
                        texture.handle,
                        0,
                    );
                }
                size = Some(texture.get_size());
            }
        }
        fbo.check();
        Self { fbo, color, size: size.unwrap() }
    }
    pub fn new_color(context: &Context, color: ColorAttachmentRead<'a>) -> Self {
        Self::new(context, color, DepthAttachmentRead::None)
    }
    pub fn get_size(&self) -> Vec2<usize> {
        self.size
    }
}

pub enum ColorAttachment<'a> {
    None,
    Texture(&'a mut Texture2d),
}

pub enum DepthAttachment<'a> {
    None,
    Renderbuffer(&'a mut Renderbuffer<DepthComponent>),
    Texture(&'a mut DepthTexture),
}

pub struct Framebuffer<'a> {
    read: FramebufferRead<'a>,
}

impl<'a> Framebuffer<'a> {
    pub fn new(context: &Context, color: ColorAttachment<'a>, depth: DepthAttachment<'a>) -> Self {
        Self {
            read: FramebufferRead::new(
                context,
                match color {
                    ColorAttachment::None => ColorAttachmentRead::None,
                    ColorAttachment::Texture(texture) => ColorAttachmentRead::Texture(texture),
                },
                match depth {
                    DepthAttachment::None => DepthAttachmentRead::None,
                    DepthAttachment::Renderbuffer(renderbuffer) => DepthAttachmentRead::Renderbuffer(renderbuffer),
                    DepthAttachment::Texture(texture) => DepthAttachmentRead::Texture(texture),
                })
        }
    }
    pub fn new_color(context: &Context, color: ColorAttachment<'a>) -> Self {
        Self::new(context, color, DepthAttachment::None)
    }
}

impl<'a> Deref for Framebuffer<'a> {
    type Target = FramebufferRead<'a>;
    fn deref(&self) -> &Self::Target {
        &self.read
    }
}

impl Context {
    pub fn default_framebuffer(&self) -> Framebuffer {
        Framebuffer {
            read: FramebufferRead {
                fbo: FBO::default(),
                color: ColorAttachmentRead::None,
                size: self.get_size(),
            }
        }
    }
}
