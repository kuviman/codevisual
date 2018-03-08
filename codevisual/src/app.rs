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
    shader_lib: ShaderLib,
}

impl App {
    fn new(title: &str) -> Self {
        let window = Window::new(title);
        let shader_lib = ShaderLib::new(window.ugli_context());
        App { window, shader_lib }
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
