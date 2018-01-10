#[cfg(target_os = "emscripten")]
mod _impl {
    use ::*;

    pub type Future = Rc<RefCell<Option<String>>>;

    impl ResourceFuture<String> for Future {
        fn unwrap(self) -> String {
            Rc::try_unwrap(self).unwrap().into_inner().unwrap()
        }
    }

    impl Resource for String {
        type Future = Future;
    }

    impl Asset for String {
        fn load(loader: &Rc<ResourceLoader>, path: &str) -> Future {
            let handle = AssetHandle::new(loader, path);
            let future = Rc::new(RefCell::new(None));
            {
                let future = future.clone();
                emscripten::async_wget_data(path, move |data| {
                    let data = std::str::from_utf8(data.unwrap()).unwrap();
                    *future.borrow_mut() = Some(String::from(data));
                    handle.confirm();
                });
            }
            future
        }
    }
}

#[cfg(not(target_os = "emscripten"))]
mod _impl {
    use ::*;

    impl Resource for String {
        type Future = ResourceJob<String>;
    }

    impl Asset for String {
        fn load(loader: &Rc<ResourceLoader>, path: &str) -> Self::Future {
            let file_path = String::from(path);
            loader.spawn_thread(path, move || {
                use std::io::Read;
                let mut data = String::new();
                std::fs::File::open(&file_path)
                    .expect(&format!("Could not read text file `{}`", file_path))
                    .read_to_string(&mut data)
                    .unwrap();
                data
            })
        }
    }
}
