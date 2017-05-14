extern crate time;
extern crate serde_json;
extern crate gl;

pub mod platform;
pub mod draw;
pub mod common;

pub struct Application {
    platform: platform::Platform,
}

static mut APPLICATION_INSTANCE: Option<Application> = None;

impl Application {
    fn get_instance() -> &'static Self {
        init().unwrap();
        unsafe { APPLICATION_INSTANCE.as_ref().unwrap() }
    }
}

pub trait Game {
    fn update(&mut self, delta_time: f32);
    fn render(&mut self) -> Vec<draw::Command>;
}

pub fn run<G: Game>(mut game: G) {
    let app = Application::get_instance();

    let mut prev_time = time::precise_time_s();
    app.platform
        .run_main_loop(|| {
            let now_time = time::precise_time_s();
            let delta_time = (now_time - prev_time) as f32;
            prev_time = now_time;
            game.update(delta_time);
            for command in game.render() {
                draw::immediate(command);
            }
            true
        });
}

pub fn init() -> Result<(), String> {
    unsafe {
        if let Some(_) = APPLICATION_INSTANCE {
            return Ok(());
        }
    }
    let platform = try!(platform::init());
    unsafe {
        APPLICATION_INSTANCE = Some(Application { platform });
    }
    Ok(())
}