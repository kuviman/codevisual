use ::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum TerrainType {
    PLAIN,
    SWAMP,
    FOREST,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WeatherType {
    CLOUD,
    CLEAR,
    RAIN,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct TickInfo {
    pub tickIndex: usize,
    pub tickCount: Option<usize>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub randomSeed: Option<i64>,
    pub terrainByCellXY: Option<Vec<Vec<TerrainType>>>,
    pub weatherByCellXY: Option<Vec<Vec<WeatherType>>>,
}

#[derive(Debug)]
pub struct GameLog {
    pub ticks: Arc<RwLock<Vec<TickInfo>>>,
}

impl codevisual::ResourceFuture<Self> for GameLog {
    fn unwrap(self) -> Self {
        self
    }
}

impl codevisual::Resource for GameLog {
    type Future = Self;
}

impl codevisual::Asset for GameLog {
    fn load(loader: &Rc<codevisual::ResourceLoader>, path: &str) -> GameLog {
        let game_log = GameLog {
            ticks: Arc::new(RwLock::new(Vec::new())),
        };
        #[cfg(target_os = "emscripten")]
        {
            let loader = loader.clone();
            loader.add_one();
            let ticks = game_log.ticks.clone();
            let mut loaded = false;
            let parse_line = move |addr: i32| {
                let line = unsafe { CString::from_raw(addr as *mut _).into_string().unwrap() };
                let tick_info: TickInfo = serde_json::from_str(&line).unwrap();
                let mut ticks = ticks.write().unwrap();
                ticks.push(tick_info);
                if !loaded {
                    loader.confirm_one();
                    loaded = true;
                }
                run_js! {
                    CodeWars.set_loaded_percent(&(100.0 * ticks.len() as f32 / ticks[0].tickCount.unwrap() as f32));
                }
            };
            run_js! {
                CodeWars.stream_download(path, brijs::Callback::from(parse_line));
            }
        }
        #[cfg(not(target_os = "emscripten"))]
        {
            use std::io::{Read, BufRead};
            let mut reader = std::io::BufReader::new(std::fs::File::open(path).unwrap());
            let ticks = game_log.ticks.clone();
            let mut parse_line = move || {
                let mut line = String::new();
                if reader.read_line(&mut line).unwrap() == 0 {
                    return false;
                }
                let tick_info: TickInfo = serde_json::from_str(&line).unwrap();
                ticks.write().unwrap().push(tick_info);
                true
            };
            parse_line();
            std::thread::spawn(move || {
                while parse_line() {}
            });
        }
        game_log
    }
}