use ::*;

mod cursor;
mod events;

pub use self::cursor::*;
pub use self::events::*;

pub struct Window {
    #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))] glutin_window: glutin::GlWindow,
    #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
    glutin_events_loop: RefCell<glutin::EventsLoop>,
    pressed_keys: RefCell<HashSet<Key>>,
    should_close: Cell<bool>,
    mouse_pos: Cell<Vec2>,
    ugli_context: Rc<ugli::Context>,
}

impl Window {
    pub fn new(title: &str) -> Self {
        #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
        js! {
            var canvas = Module.canvas;
            window.setInterval(function() {
                canvas.width = canvas.clientWidth;
                canvas.height = canvas.clientHeight;
            }, 300);
        }
        #[cfg(target_os = "emscripten")]
        let window = {
            println!("Starting {}", title);
            let ugli_context =
                Rc::new(ugli::Context::create_webgl(emscripten::Selector::Canvas).unwrap());
            Self {
                ugli_context,
                should_close: Cell::new(false),
                mouse_pos: Cell::new(vec2(0.0, 0.0)),
                pressed_keys: RefCell::new(HashSet::new()),
            }
        };
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        let window = {
            use glutin::GlContext;
            let glutin_events_loop = glutin::EventsLoop::new();
            let glutin_window = glutin::GlWindow::new(
                glutin::WindowBuilder::new().with_title(title), //.with_visibility(false),
                glutin::ContextBuilder::new().with_vsync(true),
                &glutin_events_loop,
            ).unwrap();
            unsafe { glutin_window.make_current() }.unwrap();
            let ugli_context = Rc::new(ugli::Context::create_from_glutin(&glutin_window));
            Self {
                glutin_window,
                glutin_events_loop: RefCell::new(glutin_events_loop),
                ugli_context,
                should_close: Cell::new(false),
                mouse_pos: Cell::new(vec2(0.0, 0.0)),
                pressed_keys: RefCell::new(HashSet::new()),
            }
        };
        window
    }

    #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
    pub fn show(&self) {
        self.glutin_window.show();
    }

    pub fn swap_buffers(&self) {
        // ugli::sync();
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        {
            use glutin::GlContext;
            self.glutin_window.swap_buffers().unwrap();
        }
    }

    pub fn get_size(&self) -> Vec2<usize> {
        #[cfg(target_os = "emscripten")]
        return emscripten::get_canvas_size();
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        return {
            let (width, height) = self.glutin_window.get_inner_size().unwrap_or((1, 1));
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

    pub fn get_events(&self) -> Vec<Event> {
        let result = self.internal_get_events();
        for event in &result {
            match *event {
                Event::KeyDown { key } => {
                    self.pressed_keys.borrow_mut().insert(key);
                }
                Event::KeyUp { key } => {
                    self.pressed_keys.borrow_mut().remove(&key);
                }
                Event::MouseMove { position } => {
                    self.mouse_pos.set(position);
                }
                _ => {}
            }
        }
        result
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.pressed_keys.borrow().contains(&key)
    }
}
