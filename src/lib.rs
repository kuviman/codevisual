extern crate time;
extern crate serde_json;
extern crate gl;

pub mod platform;

pub struct Application {
    platform: platform::Platform,
}

pub trait Game {
    fn update(&mut self, delta_time: f32);
    fn render(&mut self);
}

impl Application {
    pub fn run<G: Game>(self, mut game: G) {
        let mut prev_time = time::precise_time_s();
        self.platform
            .run_main_loop(|| {
                               let now_time = time::precise_time_s();
                               let delta_time = (now_time - prev_time) as f32;
                               prev_time = now_time;
                               game.update(delta_time);
                               game.render();
                               true
                           });
    }
}

pub fn init() -> Result<Application, String> {
    let platform = try!(platform::init());
    Ok(Application { platform })
}