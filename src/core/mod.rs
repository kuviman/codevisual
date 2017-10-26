use ::*;

mod window;
mod material;
mod resources;
mod settings;
mod profiler;

pub use self::material::*;
pub use self::window::*;
pub use self::resources::*;
pub use self::settings::*;
pub use self::profiler::*;

pub struct Application {
    window: Window,
    profiler: Rc<Profiler>,
}

impl Application {
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
                run_js! {
                CodeVisual.internal.show_error(&error);
            }
            }
            std::panic::set_hook(Box::new(panic_hook));
        }
        #[cfg(target_os = "emscripten")]
        {
            run_js! {
                CodeVisual.internal.init();
            }
        }
        Application {
            window: Window::new(title),
            profiler: Rc::new(Profiler::new()),
        }
    }

    pub fn profiler(&self) -> Rc<Profiler> {
        // TODO: Do we need Rc (to detach from self)?
        self.profiler.clone()
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn ugli_context(&self) -> &Rc<ugli::Context> {
        self.window.ugli_context()
    }
}

pub trait Game {
    type Resources: ResourceContainer;
    fn get_title() -> String {
        String::from("CodeVisual application")
    }
    fn get_help_html() -> String {
        String::from("No help")
    }
    fn new(app: &Rc<Application>, resources: Self::Resources) -> Self;
    fn update(&mut self, delta_time: f64) {
        #![allow(unused_variables)]
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer);
    fn handle_event(&mut self, event: Event) {
        #![allow(unused_variables)]
    }
}

pub fn run<G: Game>() {
    let app = Rc::new(Application::new(&G::get_title()));
    #[cfg(target_os = "emscripten")]
    run_js! {
        CodeVisual.internal.set_help_html(&G::get_help_html());
    }
    #[cfg(not(target_os = "emscripten"))]
    let app_clone = app.clone();
    //    app.add_setting(Setting::Bool {
    //        name: String::from("_sync_draw"),
    //        default: false,
    //        setter: Box::new(|sync| {
    //            ugli::sync_draw(sync);
    //        }),
    //    });
    let resource_loader = Rc::new(ResourceLoader::new(&app));
    let resources_future = Rc::new(RefCell::new(Some(G::Resources::load(&resource_loader))));

    let start = move || {
        if !resource_loader.ready() {
            #[cfg(target_os = "emscripten")]
            run_js! {
                CodeVisual.internal.set_load_progress(&resource_loader.get_loaded_count(), &resource_loader.get_total_count());
            }
            return false;
        }
        let resources_future = mem::replace(&mut *resources_future.borrow_mut(), None).unwrap();
        let mut game = G::new(&app, resources_future.unwrap());

        #[cfg(target_os = "emscripten")]
        run_js! {
            CodeVisual.internal.before_main_loop();
        }

        #[cfg(not(target_os = "emscripten"))]
        app.window.show();

        let mut timer = Timer::new();
        let main_loop = || {
            app.profiler.scoped("main_loop", || {
                app.profiler.scoped("Game::handle_events", || {
                    for event in app.window.get_events() {
                        game.handle_event(event);
                    }
                });

                let delta_time = timer.tick().min(0.1); // TODO: configure

                app.profiler.scoped("Game::update", || {
                    game.update(delta_time);
                });

                app.profiler.scoped("Game::draw", || {
                    game.draw(&mut app.ugli_context().default_framebuffer());
                });

                app.profiler.scoped("CodeVisual::update_stats", || {
                    #[cfg(target_os = "emscripten")]
                    run_js! {
                        CodeVisual.internal.update_stats();
                    };
                });

                app.profiler.scoped("Window::swap_buffers", || {
                    app.window.swap_buffers();
                });
            });
            app.profiler.tick();
        };

        #[cfg(target_os = "emscripten")]
        web::set_main_loop(main_loop);

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
    web::set_main_loop(|| { start(); });

    #[cfg(not(target_os = "emscripten"))]
    while !start() && !app_clone.window.should_close() {
        // TODO: Loading screen
        thread::sleep(std::time::Duration::from_millis(100));
        app_clone.window.get_events();
    };
}
