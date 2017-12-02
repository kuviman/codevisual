use ::*;

pub struct Context {
    #[cfg(target_os = "emscripten")]
    pub(crate) webgl_context: emscripten::webgl::Context,
    size: Cell<Vec2<usize>>,
    phantom_data: PhantomData<*mut ()>,
}

#[cfg(target_os = "emscripten")]
impl Context {
    pub fn create_webgl(target: emscripten::Selector) -> emscripten::HtmlResult<Self> {
        let webgl_context = emscripten::webgl::Context::create(
            target, &emscripten::webgl::ContextAttributes {
                alpha: false,
                antialias: false,
                preserve_drawing_buffer: false,
                ..default()
            })?;
        webgl_context.make_current()?;
        let context = Context {
            webgl_context,
            size: Cell::new(vec2(1, 1)),
            phantom_data: PhantomData,
        };
        context.init(emscripten::get_proc_address);
        Ok(context)
    }
    pub fn webgl_context(&self) -> &emscripten::webgl::Context {
        &self.webgl_context
    }
}

#[cfg(not(target_os = "emscripten"))]
impl Context {
    pub fn create_from_glutin<C: glutin::GlContext>(glutin_context: &C) -> Self {
        let context = Context {
            size: Cell::new(vec2(1, 1)),
            phantom_data: PhantomData,
        };
        context.init(|symbol| glutin_context.get_proc_address(symbol) as *const c_void);
        context
    }
}

impl Context {
    pub fn init<F: Fn(&str) -> *const c_void>(&self, get_proc_address: F) {
        gl::load_with(get_proc_address);
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            #[cfg(not(target_os = "emscripten"))]
                gl::Enable(gl::PROGRAM_POINT_SIZE);
            #[cfg(target_os = "windows")]
                gl::Enable(0x8861); // GL_POINT_SPRITE
        }
    }
    pub fn _set_size(&self, size: Vec2<usize>) {
        self.size.set(size);
    }
    pub(crate) fn get_size(&self) -> Vec2<usize> {
        self.size.get()
    }
}
