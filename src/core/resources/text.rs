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
                brijs::wget(path, move |data| {
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

    pub struct Future {
        join_handle: thread::JoinHandle<String>,
    }

    impl ResourceFuture<String> for Future {
        fn unwrap(self) -> String {
            self.join_handle.join().unwrap()
        }
    }

    impl Resource for String {
        type Future = Future;
    }

    impl Asset for String {
        fn load(loader: &Rc<ResourceLoader>, path: &str) -> Future {
            let handle = AssetHandle::new(loader, path);
            let path = String::from(path);
            Future {
                join_handle: thread::spawn(move || {
                    use std::io::Read;
                    let mut data = String::new();
                    std::fs::File::open(&path)
                        .expect(&format!("Could not read text file `{}`", path))
                        .read_to_string(&mut data)
                        .unwrap();
                    handle.confirm();
                    data
                }),
            }
        }
    }
}