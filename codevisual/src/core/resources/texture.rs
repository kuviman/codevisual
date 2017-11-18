#[cfg(target_os = "emscripten")]
mod _impl {
    use ::*;

    pub type Future = Rc<RefCell<Option<ugli::Texture2d>>>;

    impl ResourceFuture<ugli::Texture2d> for Future {
        fn unwrap(self) -> ugli::Texture2d {
            Rc::try_unwrap(self).unwrap().into_inner().unwrap()
        }
    }

    impl Resource for ugli::Texture2d {
        type Future = Future;
    }

    impl Asset for ugli::Texture2d {
        fn load(loader: &Rc<ResourceLoader>, path: &str) -> Self::Future {
            let handle = AssetHandle::new(loader, path);
            let future = Rc::new(RefCell::new(None));
            {
                let mut texture = ugli::Texture2d::new_uninitialized(
                    loader.app.ugli_context(),
                    vec2(1, 1),
                );
                let texture_handle = texture._get_handle();
                let future = future.clone();
                fn make_mut<F: FnOnce((i32, i32)) + 'static>(f: F) -> Box<FnMut((i32, i32)) + 'static> {
                    let mut f = Some(f);
                    Box::new(move |arg: (i32, i32)| {
                        mem::replace(&mut f, None).unwrap()(arg);
                    })
                };
                let callback = move |size: (i32, i32)| {
                    texture._set_size(vec2(size.0 as usize, size.1 as usize));
                    if texture.is_pot() {
                        texture.gen_mipmaps();
                    }
                    *future.borrow_mut() = Some(texture);
                    handle.confirm();
                };
                let mut callback = make_mut(callback);
                let callback = move |arg| callback(arg);
                let callback = webby::Callback::from(callback);
                run_js! {
                    CodeVisual.internal.load_texture(path, &texture_handle, callback);
                }
            }
            future
        }
    }
}

#[cfg(not(target_os = "emscripten"))]
mod _impl {
    use ::*;

    pub struct Future {
        context: Rc<ugli::Context>,
        job: ResourceJob<image::RgbaImage>,
    }

    impl ResourceFuture<ugli::Texture2d> for Future {
        fn unwrap(self) -> ugli::Texture2d {
            let image = self.job.unwrap();
            ugli::Texture2d::from_image(&self.context, image)
        }
    }

    impl Resource for ugli::Texture2d {
        type Future = Future;
    }

    impl Asset for ugli::Texture2d {
        fn load(loader: &Rc<ResourceLoader>, path: &str) -> Self::Future {
            let handle = AssetHandle::new(loader, path);
            let path = String::from(path);
            Future {
                context: loader.app.ugli_context().clone(),
                job: loader.spawn_thread("texture loader", move || {
                    let image = image::open(&path)
                        .expect(&format!("Could not load texture from `{}`", path));
                    let image = match image {
                        image::DynamicImage::ImageRgba8(image) => image,
                        _ => image.to_rgba(),
                    };
                    handle.confirm();
                    image
                }),
            }
        }
    }
}