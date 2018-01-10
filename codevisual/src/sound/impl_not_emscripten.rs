use ::*;
use rodio::Source;

pub struct Sound {
    sound: rodio::source::Buffered<rodio::Decoder<std::fs::File>>,
}

impl Sound {
    #[allow(deprecated)]
    pub fn play(&self, volume: f64) {
        if let Some(end) = rodio::get_default_endpoint() {
            rodio::play_raw(
                &end,
                self.sound.clone().convert_samples().amplify(volume as _),
            );
        }
    }
}

impl Resource for Sound {
    type Future = ResourceJob<Sound>;
}

impl Asset for Sound {
    fn load(loader: &Rc<ResourceLoader>, path: &str) -> Self::Future {
        let handle = AssetHandle::new(loader, path);
        let path = String::from(path);
        loader.spawn_thread("sound loader", move || {
            let file = std::fs::File::open(path).unwrap();
            let sound = rodio::Decoder::new(file).unwrap().buffered();
            handle.confirm();
            Sound { sound }
        })
    }
}
