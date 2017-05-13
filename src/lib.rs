extern crate time;
extern crate serde_json;
extern crate gl;

pub mod platform;
pub mod draw;

pub struct Application {
    platform: platform::Platform,
}

static mut APPLICATION_INSTANCE: Option<Application> = None;

impl Application {
    fn get_instance() -> &'static Self {
        unsafe {
            if let None = APPLICATION_INSTANCE {
                init().unwrap();
            }
            APPLICATION_INSTANCE.as_ref().unwrap()
        }
    }
}

pub trait Game {
    fn update(&mut self, delta_time: f32);
    fn render(&mut self);
}

pub fn run<G: Game>(mut game: G) {
    let mut prev_time = time::precise_time_s();
    Application::get_instance()
        .platform
        .run_main_loop(|| {
                           let now_time = time::precise_time_s();
                           let delta_time = (now_time - prev_time) as f32;
                           prev_time = now_time;
                           game.update(delta_time);
                           game.render();
                           true
                       });
}

pub fn init() -> Result<(), String> {
    let platform = try!(platform::init());
    unsafe {
        APPLICATION_INSTANCE = Some(Application { platform });
    }
    Ok(())
}