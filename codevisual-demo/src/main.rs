#[macro_use]
extern crate codevisual;
extern crate codevisual_debug_overlay;
#[macro_use]
extern crate ugli;

use codevisual::prelude::*;

#[derive(Vertex)]
struct Vertex {
    a_v: [f32; 2],
    a_vt: Vec2<f32>,
}

struct Demo {
    context: Rc<codevisual::Context>,
    texture_future: Box<codevisual::AssetFuture<Output = ugli::Texture>>,
    texture: Option<ugli::Texture>,
    vertices: ugli::VertexBuffer<Vertex>,
    program: ugli::Program,
}

impl Demo {
    fn new(context: &Rc<codevisual::Context>) -> Self {
        use codevisual::AssetManager;
        Demo {
            context: context.clone(),
            texture_future: Box::new(context.default_asset_manager().load("rs-logo.png")),
            texture: None,
            vertices: ugli::VertexBuffer::new_static(
                context.ugli_context(),
                vec![
                    Vertex {
                        a_v: [-1.0, 1.0],
                        a_vt: vec2(0.0, 0.0),
                    },
                    Vertex {
                        a_v: [1.0, 1.0],
                        a_vt: vec2(1.0, 0.0),
                    },
                    Vertex {
                        a_v: [1.0, -1.0],
                        a_vt: vec2(1.0, 1.0),
                    },
                    Vertex {
                        a_v: [-1.0, -1.0],
                        a_vt: vec2(0.0, 1.0),
                    },
                ],
            ),
            program: context
                .shader_lib()
                .compile(include_str!("program.glsl"))
                .unwrap(),
        }
    }
}

impl codevisual::App for Demo {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        if self.texture.is_none() {
            if self.texture_future.is_loaded().unwrap() {
                self.texture = Some(self.texture_future.unwrap().unwrap());
            }
        }
        ugli::clear(framebuffer, Some(Color::BLUE), None);
        let center = vec2(framebuffer.get_size().x as _, framebuffer.get_size().y as _) / 2.0;
        self.context.default_font().draw_aligned(
            framebuffer,
            "CodeVisual Demo",
            center,
            0.5,
            32.0,
            Color::WHITE,
        );

        if let Some(ref texture) = self.texture {
            ugli::draw(
                framebuffer,
                &self.program,
                ugli::DrawMode::TriangleFan,
                &self.vertices,
                uniforms! {
                    u_color: Color::WHITE,
                    u_texture: texture,
                },
                ugli::DrawParameters {
                    blend_mode: Some(default()),
                    ..default()
                },
            );
        }
    }
}

fn main() {
    #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
    {
        if let Ok(path) = std::env::var("CARGO_MANIFEST_DIR") {
            std::env::set_current_dir(std::path::Path::new(&path).join("static")).unwrap();
        }
    }
    let context = Rc::new(codevisual::Context::new("CodeVisual Demo"));
    let app = Demo::new(&context);
    let app = codevisual_debug_overlay::App::new(&context, app);
    codevisual::run(context, app);
}
