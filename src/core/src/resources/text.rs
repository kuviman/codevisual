use ::*;

pub type StringResourceFuture = Rc<RefCell<Option<String>>>;

impl ResourceFuture<String> for StringResourceFuture {
    fn unwrap(self) -> String {
        Rc::try_unwrap(self).unwrap().into_inner().unwrap()
    }
}

impl Resource for String {
    type Future = StringResourceFuture;
}

impl Asset for String {
    fn load(loader: &Rc<ResourceLoader>, path: &str) -> StringResourceFuture {
        let future = Rc::new(RefCell::new(None));
        let handle = AssetHandle::new(loader, path);
        #[cfg(target_os = "emscripten")]
        {
            let future = future.clone();
            brijs::wget(path, move |data| {
                *future.borrow_mut() = Some(String::from(data));
                handle.confirm();
            });
        }
        #[cfg(not(target_os = "emscripten"))]
        {
            use std::io::Read;
            let mut data = String::new();
            std::fs::File::open(path)
                .expect(&format!("Could not read text file `{}`", path))
                .read_to_string(&mut data)
                .unwrap();
            *future.borrow_mut() = Some(String::from(data));
            handle.confirm();
        }
        future
    }
}
