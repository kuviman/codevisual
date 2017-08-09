#[allow(unused_imports)]
#[macro_use]
extern crate vpl;

pub ( crate ) use vpl::*;

extern crate ugli;

#[cfg(not(target_os = "emscripten"))]
extern crate image;

#[cfg(target_os = "emscripten")]
#[macro_use]
extern crate brijs;

#[cfg(target_os = "emscripten")]
extern crate codevisual_core_html;

#[cfg(target_os = "emscripten")]
extern crate codevisual_core_css;

#[cfg(target_os = "emscripten")]
extern crate codevisual_core_js;

extern crate codevisual_window;
extern crate codevisual_material;
#[allow(unused_imports)]
#[macro_use]
extern crate codevisual_derive;

pub use codevisual_window::*;
pub use codevisual_material::*;
pub use codevisual_derive::*;

mod resources;
mod settings;

pub use resources::*;
pub use settings::*;

pub struct Application {
    window: Window,
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
        Application { window: Window::new(title) }
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn ugli_context(&self) -> &Rc<ugli::Context> {
        self.window.ugli_context()
    }
}

pub trait Game {
    type Resources: ResourceContainer;
    fn get_title() -> String;
    fn new(app: Rc<Application>, resources: Self::Resources) -> Self;
    fn update(&mut self, delta_time: f64);
    fn draw(&mut self);
    fn handle_event(&mut self, event: Event);
}

pub fn run<G: Game>() {
    let app = Rc::new(Application::new(&G::get_title()));
    let resource_loader = Rc::new(ResourceLoader::new(app.clone()));
    let resources = Rc::new(RefCell::new(Some(G::Resources::load(&resource_loader))));

    let start = move || {
        if resource_loader.loaded_resource_count.get() != resource_loader.resource_count.get() {
            return false;
        }
        let mut resources_swapper = None;
        std::mem::swap(&mut *resources.borrow_mut(), &mut resources_swapper);
        let mut game = G::new(app.clone(), resources_swapper.unwrap().unwrap());

        #[cfg(target_os = "emscripten")]
        run_js! {
            CodeVisual.internal.before_main_loop();
        }

        #[cfg(target_os = "emscripten")]
        let mut prev_time = brijs::get_now();
        #[cfg(not(target_os = "emscripten"))]
        let mut prev_time = std::time::Instant::now();

        let mut main_loop = || {
            for event in app.window.get_events() {
                game.handle_event(event);
            }

            #[cfg(target_os = "emscripten")]
            let delta_time = {
                let now_time = brijs::get_now();
                let delta_time = now_time - prev_time;
                prev_time = now_time;
                delta_time
            };
            #[cfg(not(target_os = "emscripten"))]
            let delta_time = {
                let now_time = std::time::Instant::now();
                let delta_time = now_time.duration_since(prev_time).subsec_nanos() as f64 / 1e9;
                prev_time = now_time;
                delta_time
            };

            game.update(delta_time.min(0.1)); // TODO: configure

            game.draw();

            #[cfg(target_os = "emscripten")]
            run_js! {
                CodeVisual.internal.update_stats(); 
            }

            app.window.swap_buffers();
        };

        #[cfg(target_os = "emscripten")]
        brijs::set_main_loop(main_loop);

        #[cfg(not(target_os = "emscripten"))]
        while !app.window.should_close() {
            main_loop();
        }

        true
    };

    #[cfg(target_os = "emscripten")]
    brijs::set_main_loop(|| { start(); });

    #[cfg(not(target_os = "emscripten"))]
    assert!(start());
}
