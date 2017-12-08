use ::*;

pub trait Game: 'static {
    type Resources: ResourceContainer;
    fn get_title() -> String {
        String::from("CodeVisual application")
    }
    fn get_help_html() -> String {
        String::from("No help")
    }
    fn new(app: &Rc<App>, resources: Self::Resources) -> Self;
    fn update(&mut self, delta_time: f64) {
        #![allow(unused_variables)]
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer);
    fn handle_event(&mut self, event: Event) {
        #![allow(unused_variables)]
    }
}

pub struct App {
    window: Window,
}

impl App {
    fn new(title: &str) -> Self {
        #[cfg(target_os = "emscripten")]
            {
                fn panic_hook(info: &std::panic::PanicInfo) {
                    let error: String = if let Some(error) = info.payload().downcast_ref::<String>() {
                        error.clone()
                    } else if let Some(error) = info.payload().downcast_ref::<&str>() {
                        error.to_string()
                    } else {
                        String::from("Something went wrong")
                    };
                    eprintln!("{:?}", error);
                    js! {
                    CodeVisual.internal.show_error(@(error));
                };
                }
                std::panic::set_hook(Box::new(panic_hook));
            }
        #[cfg(target_os = "emscripten")]
            js! {
            CodeVisual.internal.init();
        };
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
    let app = Rc::new(App::new(&G::get_title()));
    #[cfg(not(target_os = "emscripten"))]
    let app_clone = app.clone();

    let resource_loader = Rc::new(ResourceLoader::new(&app));
    let resources_future = Rc::new(RefCell::new(Some(G::Resources::load(&resource_loader))));

    #[cfg(target_os = "emscripten")]
        js! {
        CodeVisual.internal.set_help_html(@(G::get_help_html()));
    };

    let start = move || {
        if !resource_loader.ready() {
            #[cfg(target_os = "emscripten")]
                js! {
                CodeVisual.internal.set_load_progress(@(resource_loader.get_loaded_count()), @(resource_loader.get_total_count()));
            };
            return false;
        }
        let resources_future = mem::replace(&mut *resources_future.borrow_mut(), None).unwrap();
        let mut game = G::new(&app, resources_future.unwrap());

        #[cfg(target_os = "emscripten")]
            js! {
            CodeVisual.internal.before_main_loop();
        };

        #[cfg(not(target_os = "emscripten"))]
            app.window.show();

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

                #[cfg(target_os = "emscripten")]
                    js! {
                    CodeVisual.internal.update_stats();
                };
            }
        };

        #[cfg(target_os = "emscripten")]
            emscripten::set_main_loop(main_loop, emscripten::MainLoopFPS::UsingAnimationFrame, true);

        #[cfg(not(target_os = "emscripten"))]
            {
                let mut main_loop = main_loop;
                while !app.window.should_close() {
                    main_loop();
                }
            }

        true
    };

    #[cfg(target_os = "emscripten")]
        emscripten::set_main_loop(move || { start(); }, emscripten::MainLoopFPS::UsingAnimationFrame, true);

    #[cfg(not(target_os = "emscripten"))]
        while !start() && !app_clone.window.should_close() {
        // TODO: Loading screen
        thread::sleep(std::time::Duration::from_millis(100));
        app_clone.window.get_events();
    };
}
