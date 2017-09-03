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
            brijs::wget(path, move |content| {
                use std::io::{Read, BufRead};
                for line in content.lines() {
                    let tick_info: TickInfo = serde_json::from_str(&line).unwrap();
                    println!("Parsed tick {}", tick_info.tickIndex);
                    if let Some(_) = tick_info.tickCount {
                        println!("{:?}", tick_info);
                    }
                    ticks.write().unwrap().push(tick_info);
                }
                loader.confirm_one();
            });
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