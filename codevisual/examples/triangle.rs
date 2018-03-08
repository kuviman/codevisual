#[macro_use]
extern crate codevisual;
#[macro_use]
extern crate ugli;

use codevisual::prelude::*;

#[derive(Vertex)]
struct Vertex {
    a_position: Vec2<f32>,
    a_color: Color,
}

struct TriangleExample {
    program: ugli::Program,
    vertices: ugli::VertexBuffer<Vertex>,
}

const SHADER_SOURCE: &str = r#"
varying vec4 v_color;

#ifdef VERTEX_SHADER
attribute vec2 a_position;
attribute vec4 a_color;
void main() {
    v_color = a_color;
    gl_Position = vec4(a_position, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
void main() {
    gl_FragColor = v_color;
}
#endif
"#;

impl codevisual::Game for TriangleExample {
    fn new(app: &Rc<codevisual::App>) -> Self {
        let context = app.ugli_context();

        Self {
            program: app.shader_lib().compile(SHADER_SOURCE),
            vertices: ugli::VertexBuffer::new_static(
                context,
                vec![
                    Vertex {
                        a_position: vec2(-0.5, -0.5),
                        a_color: Color::rgb(1.0, 0.0, 0.0),
                    },
                    Vertex {
                        a_position: vec2(0.5, -0.5),
                        a_color: Color::rgb(0.0, 1.0, 0.0),
                    },
                    Vertex {
                        a_position: vec2(0.0, 0.5),
                        a_color: Color::rgb(0.0, 0.0, 1.0),
                    },
                ],
            ),
        }
    }
    fn title() -> String {
        String::from("CodeVisual Example - Triangle")
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::rgb(0.0, 0.0, 0.0)), None);
        ugli::draw(
            framebuffer,
            &self.program,
            ugli::DrawMode::Triangles,
            &self.vertices,
            (),
            ugli::DrawParameters {
                depth_func: None,
                ..Default::default()
            },
        );
    }
}

fn main() {
    codevisual::run::<TriangleExample>();
}
