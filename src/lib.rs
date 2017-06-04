extern crate time;
extern crate serde_json;
extern crate gl;
extern crate cgmath;

pub mod platform;
pub mod draw;
pub mod common;

pub struct Application {
    platform: platform::Platform,
}

static mut APPLICATION_INSTANCE: Option<Application> = None;

pub type Error = String;

impl Application {
    pub fn get_instance() -> &'static Self {
        init().unwrap();
        unsafe { APPLICATION_INSTANCE.as_ref().unwrap() }
    }
    pub fn get_size(&self) -> (u32, u32) {
        self.platform.get_size()
    }
}

pub trait Game {
    fn update(&mut self, delta_time: f32);
    fn render<T: draw::Target>(&mut self, target: &mut T);
}

pub fn run<G: Game>(mut game: G) {
    let app = Application::get_instance();

    let mut prev_time = time::precise_time_s();
    app.platform
        .run_main_loop(|| {
            let now_time = time::precise_time_s();
            let delta_time = (now_time - prev_time) as f32;
            prev_time = now_time;
            game.update(delta_time.min(0.1)); // TODO: configure
            let mut screen = draw::Screen;
            unsafe {
                // TODO: find place for it
                gl::Enable(gl::BLEND);
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            }
            game.render(&mut screen);
            true
        });
}

pub fn init() -> Result<(), ::Error> {
    unsafe {
        if let Some(_) = APPLICATION_INSTANCE {
            return Ok(());
        }
    }
    std::panic::set_hook(Box::new(platform::panic_hook));
    let platform = platform::init()?;
    unsafe {
        APPLICATION_INSTANCE = Some(Application { platform });
    }
    Ok(())
}