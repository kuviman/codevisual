use ::*;

pub struct App {
    window: Window,
    shader_lib: ShaderLib,
    default_font: Font,
}

impl App {
    fn new(title: &str) -> Self {
        let window = Window::new(title);
        let shader_lib = ShaderLib::new(window.ugli_context());
        let default_font = {
            let data = include_bytes!("font/default.ttf") as &[u8];
            Font::new_with(window.ugli_context(), &shader_lib, data.to_owned()).unwrap()
        };
        App {
            window,
            shader_lib,
            default_font,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn ugli_context(&self) -> &Rc<ugli::Context> {
        self.window.ugli_context()
    }

    pub fn shader_lib(&self) -> &ShaderLib {
        &self.shader_lib
    }

    pub fn default_font(&self) -> &Font {
        &self.default_font
    }
}

impl App {
    pub fn run<G: Game>() {
        let app = Rc::new(App::new(&G::title()));
        let game = Rc::new(RefCell::new(G::new(&app)));
        app.window.set_event_handler(Box::new({
            let game = game.clone();
            move |event| {
                game.borrow_mut().handle_event(event);
            }
        }));

        let mut timer = Timer::new();
        let main_loop = {
            let app = app.clone();
            move || {
                let delta_time = timer.tick().min(0.1); // TODO: configure
                game.borrow_mut().update(delta_time);

                game.borrow_mut()
                    .draw(&mut ugli::Framebuffer::default(app.ugli_context()));

                app.window.swap_buffers();
            }
        };

        #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
        js! {
            var main_loop = @{main_loop};
            function main_loop_wrapper() {
                main_loop();
                window.requestAnimationFrame(main_loop_wrapper);
            }
            main_loop_wrapper();
        }

        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        {
            let mut main_loop = main_loop;
            while !app.window.should_close() {
                main_loop();
            }
        }
    }
}
