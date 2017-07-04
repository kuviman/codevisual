#![allow(unused_imports)]
use super::*;
use commons::*;
use std;

pub struct TextResource {
    value: Rc<RefCell<String>>,
    loaded: Rc<Cell<bool>>,
}

impl TextResource {
    pub fn get(&self) -> Ref<String> {
        assert!(self.loaded.get());
        self.value.borrow()
    }
}

pub fn load_text(loader: &ResourceLoader, path: &str) -> TextResource {
    let resource = TextResource {
        value: Rc::new(RefCell::new(String::new())),
        loaded: Rc::new(Cell::new(false)),
    };
    #[cfg(target_os = "emscripten")]
    {
        let value = resource.value.clone();
        let loaded = resource.loaded.clone();
        loader.resource_count.set(loader.resource_count.get() + 1);
        let loaded_resource_count = loader.loaded_resource_count.clone();
        ::emscripten::wget(path, move |data| {
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