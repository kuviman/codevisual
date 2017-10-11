use ::*;

#[derive(Debug)]
pub struct AtomicCell<T: Copy> {
    #[cfg(target_os = "emscripten")]
    inner: Cell<T>,
    #[cfg(not(target_os = "emscripten"))]
    inner: Mutex<T>,
}

#[cfg(target_os = "emscripten")]
impl<T: Copy> AtomicCell<T> {
    pub fn new(value: T) -> Self {
        Self { inner: Cell::new(value) }
    }
    pub fn get(&self) -> T {
        self.inner.get()
    }
    pub fn set(&self, value: T) {
        self.inner.set(value);
    }
}

#[cfg(not(target_os = "emscripten"))]
impl<T: Copy> AtomicCell<T> {
    pub fn new(value: T) -> Self {
        Self { inner: Mutex::new(value) }
    }
    pub fn get(&self) -> T {
        *self.inner.lock().unwrap()
    }
    pub fn set(&self, value: T) {
        *self.inner.lock().unwrap() = value;
    }
}