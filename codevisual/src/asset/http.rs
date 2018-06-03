use *;

pub struct HttpAssetManager {
    context: Rc<ugli::Context>,
}

impl HttpAssetManager {
    pub fn new(context: &Rc<ugli::Context>) -> Self {
        Self {
            context: context.clone(),
        }
    }
}

impl AssetLoader<ugli::Texture> for HttpAssetManager {
    type Future = SimpleAssetFuture<ugli::Texture>;
    fn load_asset(&self, path: &str) -> Self::Future {
        let future = SimpleAssetFuture::new();
        let image = stdweb::web::html_element::ImageElement::new();
        let handler = {
            let image = image.clone();
            let handle = future.get_handle();
            let context = self.context.clone();
            let path = path.to_owned();
            move |success: bool| {
                let mut lock = handle.lock().unwrap();
                *lock = Some(if success {
                    Ok(ugli::Texture::from_image(&context, image))
                } else {
                    Err(format_err!("Failed to load image from {:?}", path))
                });
            }
        };
        // TODO: https://github.com/koute/stdweb/issues/171
        js! {
            @(no_return)
            var handler = @{stdweb::Once(handler)};
            var image = @{image.clone()};
            image.onload = function() { handler(true); };
            image.onerror = function() { handler(false); };
        }
        image.set_src(path);
        future
    }
}

impl AssetManager for HttpAssetManager {
    fn load<T>(&self, path: &str) -> <Self as AssetLoader<T>>::Future
    where
        Self: AssetLoader<T>,
    {
        self.load_asset(path)
    }
}
