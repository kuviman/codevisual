use ::*;
use raw::*;

#[derive(Debug, Clone)]
pub struct Loader {
    #[cfg(target_os = "emscripten")]
    game_log: Rc<RefCell<Option<GameLog>>>,
    #[cfg(not(target_os = "emscripten"))]
    game_log: Arc<RwLock<Option<GameLog>>>,
}

pub struct ReadGuard<'a> {
    #[cfg(target_os = "emscripten")]
    guard: Ref<'a, Option<GameLog>>,
    #[cfg(not(target_os = "emscripten"))]
    guard: std::sync::RwLockReadGuard<'a, Option<GameLog>>,
}

impl<'a> Deref for ReadGuard<'a> {
    type Target = GameLog;

    fn deref(&self) -> &Self::Target {
        self.guard.as_ref().unwrap()
    }
}

pub struct WriteGuard<'a> {
    #[cfg(target_os = "emscripten")]
    guard: RefMut<'a, Option<GameLog>>,
    #[cfg(not(target_os = "emscripten"))]
    guard: std::sync::RwLockWriteGuard<'a, Option<GameLog>>,
}

impl<'a> Deref for WriteGuard<'a> {
    type Target = GameLog;

    fn deref(&self) -> &Self::Target {
        self.guard.as_ref().expect("GameLog is not yet created")
    }
}

impl<'a> DerefMut for WriteGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.as_mut().expect("GameLog is not yet created")
    }
}

#[cfg(target_os = "emscripten")]
impl Loader {
    pub fn new() -> Self {
        Self {
            game_log: Rc::new(RefCell::new(None)),
        }
    }
    pub fn read(&self) -> ReadGuard {
        ReadGuard {
            guard: self.game_log.borrow()
        }
    }
    pub fn write(&self) -> WriteGuard {
        WriteGuard {
            guard: self.game_log.borrow_mut()
        }
    }
}

#[cfg(not(target_os = "emscripten"))]
impl Loader {
    pub fn new() -> Self {
        Self {
            game_log: Arc::new(RwLock::new(None)),
        }
    }
    pub fn read(&self) -> ReadGuard {
        ReadGuard {
            guard: self.game_log.read().expect("Failed to lock GameLog for read"),
        }
    }
    pub fn write(&self) -> WriteGuard {
        WriteGuard {
            guard: self.game_log.write().expect("Failed to lock GameLog for write"),
        }
    }
}

impl codevisual::ResourceFuture<Self> for Loader {
    fn unwrap(self) -> Self {
        self
    }
}

impl codevisual::Resource for Loader {
    type Future = Self;
}

impl codevisual::Asset for Loader {
    fn load(loader: &Rc<codevisual::ResourceLoader>, path: &str) -> Self {
        let sync = Self::new();
        #[cfg(target_os = "emscripten")]
        {
            let loader = loader.clone();
            loader.add_one();
            let sync = sync.clone();
            let mut loaded = false;
            let mut confirmed = false;
            let mut ticks = 0;
            let mut parse_line = move |line: &str| {
                let tick_info: TickInfo = serde_json::from_str(line).unwrap();
                if loaded {
                    sync.write().add_tick(tick_info);
                } else {
                    loaded = true;
                    *sync.game_log.borrow_mut() = Some(GameLog::new(tick_info));
                }
                ticks += 1;
                if (ticks > 1000 || sync.read().is_loaded()) && !confirmed {
                    confirmed = true;
                    loader.confirm_one();
                }
                codewars2017_web::set_loaded_percent(100.0 * ticks as f32 / sync.read().tick_count as f32);
            };
            codewars2017_web::stream_download(path, parse_line);
        }
        #[cfg(not(target_os = "emscripten"))]
        {
            use std::io::{Read, BufRead};
            let mut reader = std::io::BufReader::new(
                std::fs::File::open(path).expect(&format!("Game log not found at `{}`", path)));
            let mut parse_line = {
                let sync = sync.clone();
                move || {
                    let mut line = String::new();
                    if reader.read_line(&mut line).unwrap() == 0 {
                        sync.write().finish();
                        return None;
                    }
                    let tick_info: TickInfo = serde_json::from_str(&line).unwrap();
                    Some(tick_info)
                }
            };
            let sync = sync.clone();
            *sync.game_log.write().unwrap() = Some(GameLog::new(parse_line().unwrap()));
            std::thread::spawn(move || {
                while let Some(tick_info) = parse_line() {
                    sync.write().add_tick(tick_info);
                }
            });
        }
        sync
    }
}