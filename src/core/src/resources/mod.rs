use ::*;

mod text;
pub use self::text::*;

mod texture;
pub use self::texture::*;

pub struct ResourceLoader {
    #[allow(dead_code)]
    app: Rc<::Application>,
    pub(crate) resource_count: Cell<usize>,
    pub(crate) loaded_resource_count: Rc<Cell<usize>>,
}

impl ResourceLoader {
    pub(crate) fn new(app: Rc<::Application>) -> Self {
        Self {
            app,
            resource_count: Cell::new(0),
            loaded_resource_count: Rc::new(Cell::new(0)),
        }
    }
}

pub trait ResourceFuture<T>: 'static {
    fn unwrap(self) -> T;
}

pub trait Resource: Sized {
    type Future: ResourceFuture<Self>;
}

pub trait Asset: Resource {
    fn load(loader: &ResourceLoader, path: &str) -> Self::Future;
}

pub trait ResourceContainer: Resource {
    fn load(loader: &ResourceLoader) -> Self::Future;
}

impl ResourceFuture<()> for () {
    fn unwrap(self) {}
}
impl Resource for () {
    type Future = ();
}
impl ResourceContainer for () {
    fn load(_: &ResourceLoader) {}
}
