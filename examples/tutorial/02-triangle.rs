#![allow(unused_variables)]

#[macro_use]
extern crate codevisual;
use codevisual::prelude::*;
use codevisual::ugli;

#[derive(Vertex)]
struct Vertex {
    a_position: Vec2<f32>,
    a_color: Color,
}

struct Tutorial {
    app: Rc<codevisual::Application>,
    shader: codevisual::Shader,
    vertices: ugli::VertexBuffer<Vertex>,
}

const SHADER_SOURCE: &str = r#"
varying vec4 v_color;

#ifdef VERTEX
attribute vec2 a_position;
attribute vec4 a_color;
void main() {
    v_color = a_color;
    gl_Position = vec4(a_position, 0.0, 1.0);
}
#endif
#ifdef FRAGMENT
void main() {
    gl_FragColor = v_color;
}
#endif
"#;

impl codevisual::Game for Tutorial {
    type Resources = ();
    fn new(app: Rc<codevisual::Application>, resources: ()) -> Self {
        let context = app.ugli_context();

        Tutorial {
            app: app.clone(),
            shader: codevisual::Shader::compile::<codevisual::ShaderPrelude>(
                context,
                &(),
                SHADER_SOURCE,
            ),
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
    fn get_title() -> String {
        String::from("CodeVisual Tutorial 02 - Triangle")
    }
    fn update(&mut self, delta_time: f64) {}
    fn draw(&mut self) {
        ugli::clear(
            &mut ugli::default_framebuffer(self.app.ugli_context()),
            Some(Color::rgb(0.0, 0.0, 0.0)),
            None,
        );
        ugli::draw(
            &mut ugli::default_framebuffer(self.app.ugli_context()),
            &self.shader.ugli_program(),
            ugli::DrawMode::Triangles,
            &ugli::plain(&self.vertices.slice(..)),
            &(),
            &ugli::DrawParameters {
                depth_test: ugli::DepthTest::Off,
                ..Default::default()
            },
        );
    }
    fn handle_event(&mut self, event: codevisual::Event) {}
}

fn main() {
    codevisual::run::<Tutorial>();
}
