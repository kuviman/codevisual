use ::*;

pub struct Context {
    size: Cell<Vec2<usize>>,
    pub ( crate ) quad: RefCell<Option<Quad>>,
    pub ( crate ) cube: RefCell<Option<Cube>>,
}

#[derive(Debug)]
pub enum ContextCreationError {
    Unknown,
}

impl Error for ContextCreationError {
    fn description(&self) -> &str {
        use ContextCreationError::*;
        match *self {
            Unknown => "Unknown",
        }
    }
}

display_error_description!(ContextCreationError);

impl Context {
    pub fn init<F>(get_proc_address: F) -> Result<Self, ContextCreationError>
        where
            F: Fn(&str) -> *const c_void,
    {
        gl::load_with(get_proc_address);
        #[cfg(not(target_os = "emscripten"))]
        unsafe {
            gl::Enable(gl::PROGRAM_POINT_SIZE);
            #[cfg(target_os = "windows")]
            gl::Enable(0x8861); // GL_POINT_SPRITE
        }
        Ok(Context {
            size: Cell::new(vec2(1, 1)),
            quad: RefCell::new(None),
            cube: RefCell::new(None),
        })
    }
    pub fn _set_size(&self, size: Vec2<usize>) {
        self.size.set(size);
    }
    pub ( crate ) fn get_size(&self) -> Vec2<usize> {
        self.size.get()
    }
}
