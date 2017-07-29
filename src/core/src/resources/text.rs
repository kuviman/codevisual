use ::*;

pub struct StringResourceFuture {
    value: Rc<RefCell<String>>,
    loaded: Rc<Cell<bool>>,
}

impl ResourceFuture<String> for StringResourceFuture {
    fn unwrap(self) -> String {
        assert!(self.loaded.get());
        Rc::try_unwrap(self.value).unwrap().into_inner()
    }
}

impl Resource for String {
    type Future = StringResourceFuture;
}

impl Asset for String {
    fn load(loader: &ResourceLoader, path: &str) -> Self::Future {
        let resource = Self::Future {
            value: Rc::new(RefCell::new(String::new())),
            loaded: Rc::new(Cell::new(false)),
        };
        #[cfg(target_os = "emscripten")]
        {
            let value = Rc::new(RefCell::new(Some(resource.value.clone())));
            let loaded = resource.loaded.clone();
            loader.resource_count.set(loader.resource_count.get() + 1);
            let loaded_resource_count = loader.loaded_resource_count.clone();
            brijs::wget(path, move |data| {
                let mut value_swapper = None;
                std::mem::swap(&mut value_swapper, &mut *value.borrow_mut());
                let value = value_swapper.unwrap();
                value.borrow_mut().push_str(data);
                loaded.set(true);
                loaded_resource_count.set(loaded_resource_count.get() + 1);
            });
        }
        #[cfg(not(target_os = "emscripten"))]
        {
            use std::io::Read;
            std::fs::File::open(path)
                .unwrap()
                .read_to_string(&mut *resource.value.borrow_mut())
                .unwrap();
            resource.loaded.set(true);
        }
        resource
    }
}
