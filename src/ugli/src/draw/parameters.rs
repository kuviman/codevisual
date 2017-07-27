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

pub struct DrawParameters {
    pub depth_test: DepthTest,
    pub blend_mode: BlendMode,
}

impl Default for DrawParameters {
    fn default() -> Self {
        Self {
            depth_test: DepthTest::On,
            blend_mode: BlendMode::Off,
        }
    }
}

impl DrawParameters {
    pub(crate) fn apply(&self) {
        self.depth_test.apply();
        self.blend_mode.apply();
    }
}