extern crate vpl;
use vpl::*;

extern crate ugli;

#[macro_use]
extern crate lazy_static;

#[cfg(target_os = "emscripten")]
#[macro_use]
extern crate brijs;

#[cfg(not(target_os = "emscripten"))]
extern crate glutin;

mod cursor;
pub use cursor::*;

mod events;
pub use events::*;

pub struct Window {
    #[cfg(not(target_os = "emscripten"))]
    glutin_window: glutin::GlWindow,
    #[cfg(not(target_os = "emscripten"))]
    glutin_events_loop: RefCell<glutin::EventsLoop>,
    should_close: Cell<bool>,
    mouse_pos: Cell<Vec2>,
    ugli_context: Rc<ugli::Context>,
}

impl Window {
    pub fn new(title: &str) -> Self {
        #[cfg(target_os = "emscripten")]
        let window = {
            let ugli_context = Rc::new(brijs::create_gl_context().unwrap());
            Self {
                ugli_context,
                should_close: Cell::new(false),
                mouse_pos: Cell::new(vec2(0.0, 0.0)),
            }
        };
        #[cfg(not(target_os = "emscripten"))]
        let window = {
            use glutin::GlContext;
            let glutin_events_loop = glutin::EventsLoop::new();
            let glutin_window = glutin::GlWindow::new(
                glutin::WindowBuilder::new().with_title(title),
                glutin::ContextBuilder::new().with_vsync(true),
                &glutin_events_loop,
            ).unwrap();
            unsafe { glutin_window.make_current() }.unwrap();
            let ugli_context = Rc::new(
                ugli::Context::init(
                    |symbol| glutin_window.get_proc_address(symbol) as *const _,
                ).unwrap(),
            );
            Self {
                glutin_window,
                glutin_events_loop: RefCell::new(glutin_events_loop),
                ugli_context,
                should_close: Cell::new(false),
                mouse_pos: Cell::new(vec2(0.0, 0.0)),
            }
        };
        window
    }

    pub fn swap_buffers(&self) {
        #[cfg(not(target_os = "emscripten"))]
        {
            use glutin::GlContext;
            self.glutin_window.swap_buffers().unwrap();
        }
    }

    pub fn get_size(&self) -> Vec2<usize> {
        #[cfg(target_os = "emscripten")] return brijs::get_canvas_size();
        #[cfg(not(target_os = "emscripten"))]
        return {
            let (width, height) = self.glutin_window.get_inner_size_pixels().unwrap();
            vec2(width as usize, height as usize)
        };
    }

    pub fn ugli_context(&self) -> &Rc<ugli::Context> {
        self.ugli_context._set_size(self.get_size());
        &self.ugli_context
    }

    pub fn should_close(&self) -> bool {
        self.should_close.get()
    }
}
