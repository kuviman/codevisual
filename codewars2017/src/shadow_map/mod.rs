use ::*;

pub struct ShadowMap {
    app: Rc<codevisual::Application>,
    map: Option<(ugli::Texture2d, ugli::DepthTexture)>,
}

pub struct ShadowMapFramebuffer<'a> {
    framebuffer: ugli::Framebuffer<'a>,
    texture: *const ugli::DepthTexture,
}

impl<'a> Deref for ShadowMapFramebuffer<'a> {
    type Target = ugli::Framebuffer<'a>;
    fn deref(&self) -> &Self::Target {
        &self.framebuffer
    }
}

impl<'a> DerefMut for ShadowMapFramebuffer<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.framebuffer
    }
}

impl<'a> ShadowMapFramebuffer<'a> {
    pub fn get_texture(self) -> &'a ugli::DepthTexture {
        unsafe { &*self.texture }
    }
}

impl ShadowMap {
    pub fn new(app: &Rc<codevisual::Application>, settings: &Rc<Settings>) -> Self {
        Self {
            app: app.clone(),
            map: None,
        }
    }

    pub fn get_framebuffer(&mut self, size: Vec2<usize>) -> ShadowMapFramebuffer {
        let need_size = size;
        if self.map.as_ref().map_or(true, |map| map.0.get_size() != need_size) {
            // TODO: need only depth, but fails on MacOS
            self.map = Some((
                ugli::Texture2d::new_uninitialized(
                    self.app.ugli_context(), need_size),
                ugli::DepthTexture::new_uninitialized(
                    self.app.ugli_context(), need_size)));
        }
        let map = self.map.as_mut().unwrap();
        let (color, depth) = (&mut map.0, &mut map.1);
        let depth_ptr = depth as *const _;
        let mut framebuffer = ugli::Framebuffer::new(
            self.app.ugli_context(),
            ugli::ColorAttachment::Texture(color),
            ugli::DepthAttachment::Texture(depth));
        ugli::clear(&mut framebuffer, None, Some(1.0));
        ShadowMapFramebuffer {
            framebuffer,
            texture: depth_ptr,
        }
    }
}

pub struct ShadowCastMaterial {
    pub material: Material,
    pub shadow_material: Material,
}

impl ShadowCastMaterial {
    pub fn new(context: &Rc<ugli::Context>,
               settings: &Rc<Settings>,
               program_source: &str) -> Self {
        Self {
            material: Material::new(context, settings, program_source),
            shadow_material: Material::new(context, settings, &format!(
                "{}\n{}", include_str!("shadow_material.glsl"), program_source)),
        }
    }
}

impl Deref for ShadowCastMaterial {
    type Target = Material;
    fn deref(&self) -> &Self::Target {
        &self.material
    }
}