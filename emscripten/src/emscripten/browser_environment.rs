use ::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MainLoopFPS {
    Fixed(u32),
    UsingAnimationFrame,
}

type MainLoopCallback = RefCell<Box<FnMut()>>;

static mut MAIN_LOOP_CALLBACK: Option<(*const MainLoopCallback, MainLoopFPS, bool)> = None;

fn call_main_loop() {
    let stored = unsafe { Rc::from_raw(MAIN_LOOP_CALLBACK.unwrap().0) };
    let callback = stored.clone();
    mem::forget(stored);
    let mut callback: RefMut<Box<FnMut()>> = callback.borrow_mut();
    let callback: &mut FnMut() = &mut **callback;
    callback();
}

pub fn set_main_loop<F: FnMut() + 'static>(callback: F, fps: MainLoopFPS, simulate_infinite_loop: bool) {
    unsafe {
        let callback: Rc<MainLoopCallback> = Rc::new(RefCell::new(Box::new(callback)));
        match MAIN_LOOP_CALLBACK {
            Some((ref mut prev_callback, prev_fps, prev_sil)) => {
                if (prev_fps, prev_sil) != (fps, simulate_infinite_loop) {
                    unimplemented!("Changing main loop parameters not implemented yet");
                }
                mem::drop(Rc::<MainLoopCallback>::from_raw(*prev_callback));
                *prev_callback = Rc::into_raw(callback);
            }
            None => {
                MAIN_LOOP_CALLBACK = Some((Rc::into_raw(callback), fps, simulate_infinite_loop));
                emscripten_set_main_loop(
                    Some(wrapper),
                    match fps {
                        MainLoopFPS::Fixed(fps) => fps as c_int,
                        MainLoopFPS::UsingAnimationFrame => 0,
                    },
                    if simulate_infinite_loop { 1 } else { 0 });
            }
        }
    }
    unsafe extern "C" fn wrapper() {
        call_main_loop();
    }
}

pub fn get_canvas_size() -> Vec2<usize> {
    let mut width: c_int = unsafe { mem::uninitialized() };
    let mut height: c_int = unsafe { mem::uninitialized() };
    let mut is_fullscreen: c_int = unsafe { mem::uninitialized() };
    unsafe { emscripten_get_canvas_size(&mut width, &mut height, &mut is_fullscreen) };
    vec2(width as usize, height as usize)
}