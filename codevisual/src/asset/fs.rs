use *;

pub struct FileSystemAssetManager {
    context: Rc<ugli::Context>,
}

impl FileSystemAssetManager {
    pub fn new(context: &Rc<ugli::Context>) -> Self {
        Self {
            context: context.clone(),
        }
    }
}

impl AssetLoader<ugli::Texture> for FileSystemAssetManager {
    type Future = Box<AssetFuture<Output = ugli::Texture>>;
    fn load_asset(&self, path: &str) -> Self::Future {
        let future = SimpleAssetFuture::new();
        let handle = future.get_handle();
        let path = path.to_owned();
        let context = self.context.clone();
        std::thread::spawn(move || {
            fn load_sync(path: &str) -> Result<image::RgbaImage, Error> {
                let image = image::open(path)?;
                Ok(match image {
                    image::DynamicImage::ImageRgba8(image) => image,
                    _ => image.to_rgba(),
                })
            }
            let mut lock = handle.lock().unwrap();
            *lock = Some(load_sync(&path));
        });
        Box::new(MapAssetFuture::new(future, move |image| {
            Ok(ugli::Texture::from_image(&context, image))
        }))
    }
}

impl AssetManager for FileSystemAssetManager {
    fn load<T>(&self, path: &str) -> <Self as AssetLoader<T>>::Future
    where
        Self: AssetLoader<T>,
    {
        self.load_asset(path)
    }
}
