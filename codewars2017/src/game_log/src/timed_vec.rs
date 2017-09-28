use ::*;

#[cfg(target_os = "emscripten")]
type AtomicCell<T: Copy> = Cell<T>;

#[cfg(not(target_os = "emscripten"))]
#[derive(Debug)]
struct AtomicCell<T: Copy> {
    inner: Mutex<T>,
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

#[derive(Debug)]
pub struct TimedVec<T> {
    data: Vec<(usize, T)>,
    last_pos: AtomicCell<usize>,
}

impl<T> TimedVec<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            last_pos: AtomicCell::new(0),
        }
    }
    pub fn push(&mut self, tick: usize, value: T) {
        if let Some(last) = self.data.last() {
            assert!(last.0 < tick);
        }
        self.data.push((tick, value));
    }
    pub fn get(&self, tick: usize) -> Option<&T> {
        let mut pos = self.last_pos.get();
        while pos + 1 < self.data.len() && self.data[pos + 1].0 <= tick {
            pos += 1;
        }
        while pos > 0 && self.data[pos].0 > tick {
            pos -= 1;
        }
        self.last_pos.set(pos);
        self.data.get(pos).map(|data| &data.1)
    }
}