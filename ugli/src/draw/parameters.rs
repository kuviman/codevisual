use ::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum DepthFunc {
    Less = gl::LESS as _,
    LessOrEqual = gl::LEQUAL as _,
    Greater = gl::GREATER as _,
}

impl Default for DepthFunc {
    fn default() -> DepthFunc {
        DepthFunc::Less
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BlendMode {
    Alpha,
}

impl Default for BlendMode {
    fn default() -> BlendMode {
        BlendMode::Alpha
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CullFace {
    Back = gl::BACK as _,
    Front = gl::FRONT as _,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DrawParameters {
    pub depth_func: Option<DepthFunc>,
    pub blend_mode: Option<BlendMode>,
    pub cull_face: Option<CullFace>,
    pub viewport: Option<Rect<usize>>,
    pub write_depth: bool,
}

impl Default for DrawParameters {
    fn default() -> Self {
        Self {
            depth_func: Some(default()),
            blend_mode: None,
            cull_face: None,
            viewport: None,
            write_depth: true,
        }
    }
}

impl DrawParameters {
    pub(crate) fn apply(&self, framebuffer_size: Vec2<usize>) {
        unsafe {
            match self.depth_func {
                Some(depth_test) => gl::DepthFunc(depth_test as _),
                None => gl::DepthFunc(gl::ALWAYS),
            }
            match self.blend_mode {
                Some(blend_mode) => {
                    gl::Enable(gl::BLEND);
                    match blend_mode {
                        BlendMode::Alpha => gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA),
                    }
                }
                None => gl::Disable(gl::BLEND),
            }
            match self.cull_face {
                Some(cull_face) => {
                    gl::Enable(gl::CULL_FACE);
                    gl::CullFace(cull_face as _);
                }
                None => gl::Disable(gl::CULL_FACE)
            }
            if let Some(rect) = self.viewport {
                gl::Viewport(rect.bottom_left.x as GLint,
                             rect.bottom_left.y as GLint,
                             rect.width() as GLsizei,
                             rect.height() as GLsizei);
            } else {
                gl::Viewport(0, 0, framebuffer_size.x as GLsizei, framebuffer_size.y as GLsizei);
            }
            gl::DepthMask(gl_bool(self.write_depth));
        }
    }
}