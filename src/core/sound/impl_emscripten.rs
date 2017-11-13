use ::*;

pub struct Sound {
    id: Rc<Cell<i32>>,
}

impl Sound {
    pub fn play(&self, volume: f64) {
        run_js! {
            CodeVisual.internal.play_sound(&self.id.get(), &volume);
        }
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
        let mut handle = Some(AssetHandle::new(loader, path));
        let sound_id = Rc::new(Cell::new(0));
        let callback = web::Callback::from({
            let sound_id = sound_id.clone();
            move |id: i32| {
                sound_id.set(id);
                mem::replace(&mut handle, None).unwrap().confirm();
            }
        });
        run_js! {
            CodeVisual.internal.load_sound(path, callback);
        }
        Sound { id: sound_id }
    }
}