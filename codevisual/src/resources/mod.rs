use ::*;

mod text;

pub use self::text::*;

mod texture;

pub use self::texture::*;

pub struct ResourceLoader {
    app: Rc<App>,
    resource_count: Cell<usize>,
    loaded_count: Arc<ACell<usize>>,
    #[cfg(not(target_os = "emscripten"))] thread_pool: threadpool::ThreadPool,
}

impl Deref for ResourceLoader {
    type Target = Rc<App>;
    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl ResourceLoader {
    pub(crate) fn new(app: &Rc<App>) -> Self {
        Self {
            app: app.clone(),
            resource_count: Cell::new(1),
            loaded_count: Arc::new(ACell::new(1)),
            #[cfg(not(target_os = "emscripten"))]
            thread_pool: threadpool::ThreadPool::new(min(4, num_cpus::get())),
        }
    }
    #[cfg(not(target_os = "emscripten"))]
    pub fn spawn_thread<T: Send + 'static, F: FnOnce() -> T + Send + 'static>(
        &self,
        name: &str,
        f: F,
    ) -> ResourceJob<T> {
        let future = ResourceJob::new();
        let result = future.result.clone();
        let handle = AssetHandle::new(self, name);
        self.thread_pool.execute(move || {
            let f_result = f();
            *result.lock().unwrap() = Some(f_result);
            mem::drop(result);
            handle.confirm();
        });
        future
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

#[cfg(not(target_os = "emscripten"))]
pub struct ResourceJob<T> {
    result: Arc<Mutex<Option<T>>>,
}

#[cfg(not(target_os = "emscripten"))]
impl<T> ResourceJob<T> {
    fn new() -> Self {
        Self {
            result: Arc::new(Mutex::new(None)),
        }
    }
}

#[cfg(not(target_os = "emscripten"))]
impl<T: 'static> ResourceFuture<T> for ResourceJob<T> {
    fn unwrap(self) -> T {
        if let Ok(mutex) = Arc::try_unwrap(self.result) {
            mutex.into_inner().unwrap().unwrap()
        } else {
            panic!("Arc failed to unwrap");
        }
    }
}

pub struct AssetHandle {
    #[allow(dead_code)] timer: Timer,
    #[allow(dead_code)] name: String,
    loaded_count: Arc<ACell<usize>>,
}

impl AssetHandle {
    pub fn new(loader: &ResourceLoader, name: &str) -> Self {
        loader.resource_count.set(loader.resource_count.get() + 1);
        Self {
            timer: Timer::new(),
            name: String::from(name),
            loaded_count: loader.loaded_count.clone(),
        }
    }
    pub fn confirm(self) {
        self.loaded_count.set(self.loaded_count.get() + 1);
        //        eprintln!("{} finished in {:.2} secs", self.name, self.timer.elapsed());
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
