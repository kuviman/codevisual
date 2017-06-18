#[cfg(target_os = "emscripten")]
extern crate emscripten_sys;
extern crate serde;
extern crate serde_json;
extern crate gl;
#[macro_use]
extern crate lazy_static;

#[cfg(target_os = "emscripten")]
#[macro_use]
mod emscripten;
pub mod draw;
pub mod common;
mod settings;
mod events;

pub use settings::*;
pub use events::*;

trait IntoJson {
    fn into(self) -> String;
}

impl<'a, T: ?Sized + serde::Serialize> IntoJson for &'a T {
    fn into(self) -> String {
        ::serde_json::to_string(self).expect("Could not convert to JSON")
    }
}

pub struct Application {}

static mut APPLICATION_INSTANCE: Option<Application> = None;

pub type Error = String;

impl Application {
    pub fn get_instance() -> &'static Self {
        Self::get_instance_mut()
    }
    pub(crate) fn get_instance_mut() -> &'static mut Self {
        unsafe {
            if let None = APPLICATION_INSTANCE {
                #[cfg(target_os = "emscripten")]
                {
                    fn panic_hook(info: &std::panic::PanicInfo) {
                        use std::string::ToString;
                        let mut json_info = serde_json::Value::Object(serde_json::Map::new());
                        if let Some(location) = info.location() {
                            let mut json_location =
                                serde_json::Value::Object(serde_json::Map::new());
                            json_location["file"] =
                                serde_json::Value::String(location.file().to_string());
                            json_location["line"] =
                                serde_json::Value::String(location.line().to_string());
                            json_info["location"] = json_location;
                        }
                        if let Some(error) = info.payload().downcast_ref::<String>() {
                            json_info["error"] = serde_json::Value::String(error.clone());
                        } else if let Some(error) = info.payload().downcast_ref::<&str>() {
                            json_info["error"] = serde_json::Value::String(error.to_string());
                        } else {
                            json_info["error"] = serde_json::Value::String(String::from("Something went wrong",),);
                        }
                        run_js!{
                            CodeVisual.internal.show_error(&json_info);
                        }
                    }
                    std::panic::set_hook(Box::new(panic_hook));

                    ::emscripten::run_script(include_str!(concat!(env!("OUT_DIR"),
                                                                  "/codevisual-lib.js")));
                    run_js!{
                        CodeVisual.internal.init(
                            include_str!(concat!(env!("OUT_DIR"), "/codevisual-lib.html")),
                            include_str!(concat!(env!("OUT_DIR"), "/codevisual-lib.css")));
                    }
                    ::emscripten::create_gl_context().expect("Could not create OpenGL context");
                    gl::load_with(emscripten::get_proc_address);
                }
                events::init();

                APPLICATION_INSTANCE = Some(Application {});
            }
            APPLICATION_INSTANCE.as_mut().unwrap()
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        #[cfg(target_os = "emscripten")]
        ::emscripten::get_canvas_size()
    }
}

pub trait Game {
    fn update(&mut self, delta_time: f32);
    fn render<T: draw::Target>(&mut self, target: &mut T);
    fn handle_event(&mut self, event: Event);
}

pub fn run<G: Game>(game: &mut G) {
    Application::get_instance();

    #[cfg(target_os = "emscripten")]
    {
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
}