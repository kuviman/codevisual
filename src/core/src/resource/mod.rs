use commons::*;

pub struct ResourceLoader {
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
    pub(crate) fn loaded(&self) -> bool {
        self.resource_count.get() == self.loaded_resource_count.get()
    }
}

pub trait Resources {
    fn new(app: &ResourceLoader) -> Self;
}