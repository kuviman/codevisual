extern crate codevisual;

struct Test {
    current_time: f32,
}

impl Test {
    fn new() -> Self {
        Self { current_time: 0.0 }
    }
}

impl codevisual::Game for Test {
    fn update(&mut self, delta_time: f32) {
        self.current_time += delta_time;
    }
    fn render(&mut self) {
        println!("TIME = {}", self.current_time);
    }
}

fn main() {
    codevisual::init().unwrap();
    codevisual::run(Test::new());
}