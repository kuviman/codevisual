extern crate gl;
#[macro_use]
extern crate lazy_static;

#[cfg(not(target_os = "emscripten"))]
extern crate glutin;

#[cfg(not(target_os = "emscripten"))]
extern crate image;

pub extern crate codevisual_commons as commons;

#[cfg(target_os = "emscripten")]
#[macro_use]
extern crate emscripten;

#[cfg(target_os = "emscripten")]
extern crate codevisual_html;

#[cfg(target_os = "emscripten")]
extern crate codevisual_css;

#[cfg(target_os = "emscripten")]
extern crate codevisual_js;

pub mod draw;
mod settings;
mod events;

pub use settings::*;
pub use events::*;

use std::sync::{RwLock, RwLockReadGuard};

pub struct Application {
    #[cfg(not(target_os = "emscripten"))]
    window: glutin::Window,
    #[cfg(not(target_os = "emscripten"))]
    events_loop: glutin::EventsLoop,
}

lazy_static!{
    static ref APPLICATION_INSTANCE: RwLock<Application> = RwLock::new(Application::new());
}

pub type Error = String;

impl Application {
    #[cfg(target_os = "emscripten")]
    fn new() -> Self {
        // fn panic_hook(info: &std::panic::PanicInfo) {
        //     use std::string::ToString;
        //     let mut json_info = serde_json::Value::Object(serde_json::Map::new());
        //     if let Some(location) = info.location() {
        //         let mut json_location =
        //             serde_json::Value::Object(serde_json::Map::new());
        //         json_location["file"] =
        //             serde_json::Value::String(location.file().to_string());
        //         json_location["line"] =
        //             serde_json::Value::String(location.line().to_string());
        //         json_info["location"] = json_location;
        //     }
        //     if let Some(error) = info.payload().downcast_ref::<String>() {
        //         json_info["error"] = serde_json::Value::String(error.clone());
        //     } else if let Some(error) = info.payload().downcast_ref::<&str>() {
        //         json_info["error"] = serde_json::Value::String(error.to_string());
        //     } else {
        //         json_info["error"] = serde_json::Value::String(String::from("Something went wrong",),);
        //     }
        //     run_js!{
        //         CodeVisual.internal.show_error(&json_info);
        //     }
        // }
        // std::panic::set_hook(Box::new(panic_hook));

        ::emscripten::run_script(codevisual_js::SOURCE);
        run_js!{
                        CodeVisual.internal.init(codevisual_html::SOURCE, codevisual_css::SOURCE);
                    }
        ::emscripten::create_gl_context().expect("Could not create OpenGL context");
        gl::load_with(emscripten::get_proc_address);
        events::init();
        Application {}
    }
    #[cfg(not(target_os = "emscripten"))]
    fn new() -> Self {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title("CodeVisual")
            .with_dimensions(640, 480)
            .with_vsync()
            .build(&events_loop)
            .unwrap();
        unsafe { window.make_current() }.unwrap();
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        unsafe {
            gl::Viewport(0, 0, 640, 480);
        }
        Application {
            window,
            events_loop,
        }
    }

    pub fn get_instance() -> RwLockReadGuard<'static, Application> {
        APPLICATION_INSTANCE.read().unwrap()
    }

    #[cfg(target_os = "emscripten")]
    pub fn get_size(&self) -> (u32, u32) {
        ::emscripten::get_canvas_size()
    }

    #[cfg(not(target_os = "emscripten"))]
    pub fn get_size(&self) -> (u32, u32) {
        self.window
            .get_inner_size_pixels()
            .unwrap_or((640, 480))
    }
}

pub trait Game {
    fn update(&mut self, delta_time: f32);
    fn render<T: draw::Target>(&mut self, target: &mut T);
    fn handle_event(&mut self, event: Event);
}

#[cfg(target_os = "emscripten")]
pub fn run<G: Game>(game: &mut G) {
    Application::get_instance();
    run_js!{
        CodeVisual.internal.before_main_loop();
    }
    let mut prev_time = emscripten::get_now();
    emscripten::set_main_loop(|| {
        for event in events::get() {
            game.handle_event(event);
        }

        let now_time = emscripten::get_now();
        let delta_time = now_time - prev_time;
        prev_time = now_time;
        game.update(delta_time.min(0.1) as f32); // TODO: configure
        let mut screen = draw::Screen;
        unsafe {
            // TODO: find place for it
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::FRONT);
            // gl::Enable(gl::BLEND);
            // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        game.render(&mut screen);
        run_js!{
            CodeVisual.internal.update_stats();
        }
    });
}

#[cfg(not(target_os = "emscripten"))]
pub fn run<G: Game>(game: &mut G) {
    use std::time::Instant;
    let app = Application::get_instance();

    let mut prev_time = Instant::now();
    while !events::should_close() {
        for event in events::get() {
            game.handle_event(event);
        }

        let now_time = Instant::now();
        let delta_time = now_time.duration_since(prev_time).subsec_nanos() as f64 / 1e9;
        prev_time = now_time;
        game.update(delta_time.min(0.1) as f32); // TODO: configure
        let mut screen = draw::Screen;
        unsafe {
            // TODO: find place for it
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::FRONT);
            // gl::Enable(gl::BLEND);
            // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        game.render(&mut screen);
        app.window.swap_buffers().expect("WTF");
    }
}