use ::*;

pub struct Sound {
    id: Rc<Cell<i32>>,
}

impl Sound {
    pub fn play(&self, volume: f64) {
        js! {
            CodeVisual.internal.play_sound(@(self.id.get()), @(volume));
        };
    }
}

impl Resource for Sound {
    type Future = Sound;
}

impl ResourceFuture<Sound> for Sound {
    fn unwrap(self) -> Sound {
        self
    }
}

impl Asset for Sound {
    fn load(loader: &Rc<ResourceLoader>, path: &str) -> Self::Future {
        let handle = AssetHandle::new(loader, path);
        let sound_id = Rc::new(Cell::new(0));
        let callback = webby::CallbackOnce::from({
            let sound_id = sound_id.clone();
            move |id: i32| {
                sound_id.set(id);
                handle.confirm();
            }
        });
        js! {
            CodeVisual.internal.load_sound(@(path), @(callback));
        };
        Sound { id: sound_id }
    }
}