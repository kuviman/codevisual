use *;

pub struct Context {
    window: Window,
    shader_lib: ShaderLib,
    default_asset_manager: DefaultAssetManager,
    default_font: Font,
}

impl Context {
    pub fn new(title: &str) -> Self {
        let window = Window::new(title);
        let ugli_context = window.ugli_context().clone();
        let shader_lib = ShaderLib::new(window.ugli_context());
        let default_font = {
            let data = include_bytes!("font/default.ttf") as &[u8];
            Font::new_with(window.ugli_context(), &shader_lib, data.to_owned()).unwrap()
        };
        Context {
            window,
            shader_lib,
            default_asset_manager: DefaultAssetManager::new(&ugli_context),
            default_font,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn ugli_context(&self) -> &Rc<ugli::Context> {
        self.window.ugli_context()
    }

    pub fn shader_lib(&self) -> &ShaderLib {
        &self.shader_lib
    }

    pub fn default_asset_manager(&self) -> &DefaultAssetManager {
        &self.default_asset_manager
    }

    pub fn default_font(&self) -> &Font {
        &self.default_font
    }
}
pub fn run(context: Rc<Context>, app: impl App) {
    let app = Rc::new(RefCell::new(app));
    context.window.set_event_handler(Box::new({
        let app = app.clone();
        move |event| {
            app.borrow_mut().handle_event(event);
        }
    }));

    let mut timer = Timer::new();
    let main_loop = {
        let context = context.clone();
        move || {
            let delta_time = timer.tick().min(0.1); // TODO: configure
            app.borrow_mut().update(delta_time);

            app.borrow_mut()
                .draw(&mut ugli::Framebuffer::default(context.ugli_context()));

            context.window.swap_buffers();
        }
    };

    #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
    js! {
        var main_loop = @{main_loop};
        function main_loop_wrapper() {
            main_loop();
            window.requestAnimationFrame(main_loop_wrapper);
        }
        main_loop_wrapper();
    }

    #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
    {
        let mut main_loop = main_loop;
        while !context.window.should_close() {
            main_loop();
        }
    }
}
