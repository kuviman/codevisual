use ::*;

mod text;

pub use self::text::*;

mod texture;

pub use self::texture::*;

pub struct ResourceLoader {
    app: Rc<Application>,
    resource_count: Cell<usize>,
    loaded_count: Rc<Cell<usize>>,
}

impl Deref for ResourceLoader {
    type Target = Rc<Application>;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl ResourceLoader {
    pub ( crate ) fn new(app: &Rc<Application>) -> Self {
        Self {
            app: app.clone(),
            resource_count: Cell::new(1),
            loaded_count: Rc::new(Cell::new(1)),
        }
    }
    pub fn ready(&self) -> bool {
        self.resource_count.get() == self.loaded_count.get()
    }
    pub fn get_total_count(&self) -> usize {
        self.resource_count.get()
    }
    pub fn get_loaded_count(&self) -> usize {
        self.loaded_count.get()
    }
}

pub struct AssetHandle {
    loaded_count: Rc<Cell<usize>>,
}

impl AssetHandle {
    pub fn new(loader: &ResourceLoader) -> Self {
        loader.resource_count.set(loader.resource_count.get() + 1);
        Self {
            loaded_count: loader.loaded_count.clone(),
        }
    }
    pub fn confirm(self) {
        self.loaded_count.set(self.loaded_count.get() + 1);
    }
}

pub trait ResourceFuture<T>: 'static {
    fn unwrap(self) -> T;
}

pub trait Resource: Sized {
    type Future: ResourceFuture<Self>;
}

pub trait Asset: Resource {
    fn load(loader: &Rc<ResourceLoader>, path: &str) -> Self::Future;
}

pub trait ResourceContainer: Resource {
    fn load(loader: &Rc<ResourceLoader>) -> Self::Future;
}

impl ResourceFuture<()> for () {
    fn unwrap(self) {}
}

impl Resource for () {
    type Future = ();
}

impl ResourceContainer for () {
    fn load(_: &Rc<ResourceLoader>) {}
}
