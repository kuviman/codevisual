use ::*;

pub trait Game: 'static {
    fn title() -> String {
        String::from("CodeVisual application")
    }
    fn new(app: &Rc<App>) -> Self;
    #[allow(unused_variables)]
    fn update(&mut self, delta_time: f64) {}
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer);
    #[allow(unused_variables)]
    fn handle_event(&mut self, event: Event) {}
}

pub struct App {
    window: Window,
}

impl App {
    fn new(title: &str) -> Self {
        App {
            window: Window::new(title),
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn ugli_context(&self) -> &Rc<ugli::Context> {
        self.window.ugli_context()
    }
}

pub fn run<G: Game>() {
    let app = Rc::new(App::new(&G::title()));
    let mut game = G::new(&app);

    let mut timer = Timer::new();
    let main_loop = {
        let app = app.clone();
        move || {
            for event in app.window.get_events() {
                game.handle_event(event);
            }

            let delta_time = timer.tick().min(0.1); // TODO: configure
            game.update(delta_time);

            game.draw(&mut app.ugli_context().default_framebuffer());

            app.window.swap_buffers();
        }
    };

    #[cfg(target_os = "emscripten")]
    {
        fn main_loop_wrapper<F: FnMut() + 'static>(mut main_loop: F) {
            main_loop();
            stdweb::web::window().request_animation_frame(move |_| main_loop_wrapper(main_loop));
        }
        main_loop_wrapper(main_loop);
    }

    #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
    {
        let mut main_loop = main_loop;
        while !app.window.should_close() {
            main_loop();
        }
    }
}
