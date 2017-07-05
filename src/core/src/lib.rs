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
pub mod resource;

pub use resource::*;
pub use settings::*;
pub use events::*;

use std::rc::Rc;

pub struct Application {
    #[cfg(not(target_os = "emscripten"))]
    window: glutin::Window,
    #[cfg(not(target_os = "emscripten"))]
    events_loop: glutin::EventsLoop,
}

#[allow(dead_code)]
const DEFAULT_SIZE: (u32, u32) = (640, 480);

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
        ::emscripten::create_gl_context().unwrap();
        gl::load_with(emscripten::get_proc_address);
        events::init();
        Application {}
    }
    #[cfg(not(target_os = "emscripten"))]
    fn new() -> Self {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title("CodeVisual")
            .with_dimensions(DEFAULT_SIZE.0, DEFAULT_SIZE.1)
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

    #[cfg(target_os = "emscripten")]
    pub fn get_size(&self) -> (u32, u32) {
        ::emscripten::get_canvas_size()
    }

    #[cfg(not(target_os = "emscripten"))]
    pub fn get_size(&self) -> (u32, u32) {
        self.window.get_inner_size_pixels().unwrap_or(DEFAULT_SIZE)
    }

    #[cfg(target_os = "emscripten")]
    pub fn set_cursor_type(&self, cursor_type: CursorType) {
        use CursorType::*;
        run_js!{
            CodeVisual.internal.set_cursor(match cursor_type {
                Default => "initial",
                Pointer => "pointer",
                Drag => "all-scroll",
            });
        }
    }

    #[cfg(not(target_os = "emscripten"))]
    pub fn set_cursor_type(&self, cursor_type: CursorType) {
        use CursorType::*;
        use glutin::MouseCursor as GC;
        self.window
            .set_cursor(match cursor_type {
                            Default => GC::Default,
                            Pointer => GC::Hand,
                            Drag => GC::AllScroll,
                        });
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CursorType {
    Default,
    Pointer,
    Drag,
}

pub trait Game {
    type Resources: Resources;
    fn new(app: Rc<Application>, resources: &Self::Resources) -> Self;
    fn update(&mut self, delta_time: f32);
    fn render<T: draw::Target>(&mut self, target: &mut T);
    fn handle_event(&mut self, event: Event);
}

#[cfg(target_os = "emscripten")]
pub fn run<G: Game>() {
    let app = Rc::new(Application::new());
    let resource_loader = ResourceLoader::new(app.clone());
    let resources = G::Resources::new(&resource_loader);
    emscripten::set_main_loop(|| {
        let resource_count = resource_loader.resource_count.get();
        let loaded_resource_count = resource_loader.loaded_resource_count.get();
        if resource_count == loaded_resource_count {
            let mut game = G::new(app.clone(), &resources);
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
                    // gl::Enable(gl::CULL_FACE);
                    // gl::CullFace(gl::FRONT);
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                }
                game.render(&mut screen);
                unsafe {
                    gl::ColorMask(0, 0, 0, 1);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    gl::ColorMask(1, 1, 1, 1);
                }
                run_js!{
                    CodeVisual.internal.update_stats();
                }
            });
        } else {
            run_js!{
                CodeVisual.internal.set_load_progress(&loaded_resource_count, &resource_count);
            }
        }
    });
}

#[cfg(not(target_os = "emscripten"))]
pub fn run<G: Game>() {
    let app = Rc::new(Application::new());
    let resource_loader = ResourceLoader::new(app.clone());
    let resources = G::Resources::new(&resource_loader);
    assert_eq!(resource_loader.loaded_resource_count.get(),
               resource_loader.resource_count.get());
    let mut game = G::new(app.clone(), &resources);

    use std::time::Instant;
    let mut prev_time = Instant::now();
    while !events::should_close() {
        for event in events::get(&app) {
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
            // gl::Enable(gl::CULL_FACE);
            // gl::CullFace(gl::FRONT);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        game.render(&mut screen);
        unsafe {
            assert!(gl::GetError() == gl::NO_ERROR);
        }
        app.window.swap_buffers().expect("WTF");
    }
}