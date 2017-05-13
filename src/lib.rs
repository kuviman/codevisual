extern crate time;
extern crate serde_json;
extern crate gl;

pub mod platform;

pub fn init() {
    platform::init();
}