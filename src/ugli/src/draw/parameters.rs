use ::*;

pub enum DepthTest {
    Off,
    On,
}

impl DepthTest {
    fn apply(&self) {
        use DepthTest::*;
        match *self {
            Off => unsafe {
                gl::Disable(gl::DEPTH_TEST);
            },
            On => unsafe {
                gl::Enable(gl::DEPTH_TEST);
            },
        }
    }
}

pub enum BlendMode {
    Off,
    Alpha,
}

impl BlendMode {
    fn apply(&self) {
        use BlendMode::*;
        match *self {
            Off => unsafe {
                gl::Disable(gl::BLEND);
            },
            Alpha => unsafe {
                gl::Enable(gl::BLEND);
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            },
        }
    }
}

pub enum CullFace {
    None,
    Back,
    Front,
}

impl CullFace {
    fn apply(&self) {
        use CullFace::*;
        match *self {
            None => unsafe {
                gl::Disable(gl::CULL_FACE);
            },
            Back => unsafe {
                gl::Enable(gl::CULL_FACE);
                gl::CullFace(gl::BACK);
            },
            Front => unsafe {
                gl::Enable(gl::CULL_FACE);
                gl::CullFace(gl::FRONT);
            },
        }
    }
}

pub struct DrawParameters {
    pub depth_test: DepthTest,
    pub blend_mode: BlendMode,
    pub cull_face: CullFace,
    pub viewport: Option<Rect<usize>>,
}

impl Default for DrawParameters {
    fn default() -> Self {
        Self {
            depth_test: DepthTest::On,
            blend_mode: BlendMode::Off,
            cull_face: CullFace::None,
            viewport: None,
        }
    }
}

impl DrawParameters {
    pub ( crate ) fn apply(&self) {
        self.depth_test.apply();
        self.blend_mode.apply();
        self.cull_face.apply();
        if let Some(rect) = self.viewport {
            unsafe {
                gl::Viewport(rect.bottom_left.x as GLint,
                             rect.bottom_left.y as GLint,
                             rect.width() as GLsizei,
                             rect.height() as GLsizei);
            }
        }
    }
}
