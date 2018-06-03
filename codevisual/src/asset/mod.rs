use *;

#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
mod fs;
#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
mod http;
mod simple;

#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
pub use self::fs::*;
#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
pub use self::http::*;
pub use self::simple::*;

pub trait AssetFuture {
    type Output;
    fn progress(&self) -> Result<f64, Error> {
        Ok(if self.is_loaded()? { 1.0 } else { 0.0 })
    }
    fn is_loaded(&self) -> Result<bool, Error>;
    fn unwrap(&self) -> Result<Self::Output, Error>;
}

impl<T: AssetFuture + ?Sized> AssetFuture for Box<T> {
    type Output = T::Output;
    fn progress(&self) -> Result<f64, Error> {
        (**self).progress()
    }
    fn is_loaded(&self) -> Result<bool, Error> {
        (**self).is_loaded()
    }
    fn unwrap(&self) -> Result<Self::Output, Error> {
        (**self).unwrap()
    }
}

pub trait AssetLoader<T> {
    type Future: AssetFuture<Output = T>;
    fn load_asset(&self, path: &str) -> Self::Future;
}

pub trait AssetManager {
    fn load<T>(&self, path: &str) -> <Self as AssetLoader<T>>::Future
    where
        Self: AssetLoader<T>,
    {
        self.load_asset(path)
    }
}

#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
pub type DefaultAssetManager = FileSystemAssetManager;

#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
pub type DefaultAssetManager = HttpAssetManager;
