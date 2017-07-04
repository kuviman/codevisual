mod text;
pub use self::text::*;

use commons::*;

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

pub trait Resources {
    fn new(app: &ResourceLoader) -> Self;
}