use ::*;

pub struct TextureResourceFuture {
    texture: Rc<ugli::Texture2d>,
    loaded: Rc<Cell<bool>>,
}

impl ResourceFuture<ugli::Texture2d> for TextureResourceFuture {
    fn unwrap(self) -> ugli::Texture2d {
        assert!(self.loaded.get());
        Rc::try_unwrap(self.texture).unwrap()
    }
}

impl Resource for ugli::Texture2d {
    type Future = TextureResourceFuture;
}

impl Asset for ugli::Texture2d {
    fn load(loader: &ResourceLoader, path: &str) -> Self::Future {
        #[cfg(target_os = "emscripten")]
        {
            let texture = Rc::new(ugli::Texture2d::new(loader.app.get_window().ugli_context(),
                                                       vec2(1, 1)));
            let loaded = Rc::new(Cell::new(false));
            {
                let texture_handle = texture._get_handle();
                let texture = Rc::new(RefCell::new(Some(texture.clone())));
                let loaded = loaded.clone();
                loader.resource_count.set(loader.resource_count.get() + 1);
                let loaded_resource_count = loader.loaded_resource_count.clone();
                let callback = brijs::Callback::from(move |size: (i32, i32)| {
                    loaded.set(true);
                    loaded_resource_count.set(loaded_resource_count.get() + 1);
                    let mut texture_swp = None;
                    std::mem::swap(&mut texture_swp, &mut *texture.borrow_mut());
                    let texture = texture_swp.unwrap();
                    texture._set_size(vec2(size.0 as usize, size.1 as usize));
                });
                run_js!{
                    CodeVisual.internal.load_texture(path, &texture_handle, callback);
                }
            }
            Self::Future { texture, loaded }
        }
        #[cfg(not(target_os = "emscripten"))]
        {
            let image = image::open(path).unwrap().to_rgba();
            let texture = ugli::Texture2d::from_image(loader.app.get_window().ugli_context(),
                                                      image);
            Self::Future {
                texture: Rc::new(texture),
                loaded: Rc::new(Cell::new(true)),
            }
        }
    }
}