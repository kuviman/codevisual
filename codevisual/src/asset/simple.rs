use *;

pub struct SimpleAssetFuture<T> {
    handle: Arc<Mutex<Option<Result<T, Error>>>>,
}

impl<T> SimpleAssetFuture<T> {
    pub fn new() -> Self {
        Self {
            handle: Arc::new(Mutex::new(None)),
        }
    }
    pub fn get_handle(&self) -> Arc<Mutex<Option<Result<T, Error>>>> {
        self.handle.clone()
    }
}

impl<T> AssetFuture for SimpleAssetFuture<T> {
    type Output = T;
    fn is_loaded(&self) -> Result<bool, Error> {
        let lock = self.handle.lock().unwrap();
        match *lock {
            Some(Ok(_)) => Ok(true),
            Some(Err(ref e)) => bail!("{}", e),
            None => Ok(false),
        }
    }
    fn unwrap(&self) -> Result<T, Error> {
        let mut lock = self.handle.lock().unwrap();
        mem::replace(&mut *lock, None).unwrap()
    }
}

pub struct MapAssetFuture<I, F> {
    inner: I,
    f: RefCell<Option<F>>,
}

impl<I, F> MapAssetFuture<I, F> {
    pub fn new(inner: I, f: F) -> Self {
        Self {
            inner,
            f: RefCell::new(Some(f)),
        }
    }
}

impl<I: AssetFuture, U, F> AssetFuture for MapAssetFuture<I, F>
where
    F: FnOnce(I::Output) -> Result<U, Error>,
{
    type Output = U;
    fn is_loaded(&self) -> Result<bool, Error> {
        self.inner.is_loaded()
    }
    fn unwrap(&self) -> Result<U, Error> {
        let f = mem::replace(&mut *self.f.borrow_mut(), None).unwrap();
        f(self.inner.unwrap()?)
    }
}
