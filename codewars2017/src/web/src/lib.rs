#[macro_use]
extern crate brijs;

extern crate prelude;

pub ( crate ) use prelude::*;

pub const JS_SOURCE: &str = include_str!(concat!(env!("OUT_DIR"), "/lib.js"));

pub fn init() {
    brijs::run_script(JS_SOURCE);
}

pub fn init_play_pause_button(paused: Rc<Cell<bool>>) {
    run_js! {
        CodeWars.set_paused(&paused.get());
        CodeWars.set_play_button_callback(brijs::Callback::from(move |_: ()| {
            paused.set(!paused.get());
            run_js!{ CodeWars.set_paused(&paused.get()); };
        }));
    }
}

pub fn set_loaded_percent(percent: f32) {
    run_js! {
        CodeWars.set_loaded_percent(&percent);
    }
}

pub fn set_playback_position(tick: usize, tick_count: usize) {
    run_js! {
        CodeWars.set_playback_position(&tick, &tick_count);
    }
}

pub fn set_timeline_callback<F: FnMut(f32) + 'static>(mut callback: F) {
    run_js! {
        CodeWars.set_timeline_callback(brijs::Callback::from(move |pos: i32| {
            callback(pos as f32 / 1000.0);
        }));
    }
}

pub fn stream_download<F: FnMut(&str) + 'static>(path: &str, mut callback: F) {
    let callback = brijs::Callback::from(move |addr: i32| {
        let line = unsafe { std::ffi::CStr::from_ptr(addr as *mut _).to_string_lossy() };
        callback(&line);
    });
    run_js! {
        CodeWars.stream_download(path, callback);
    }
}