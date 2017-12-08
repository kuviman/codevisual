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
    material: codevisual::Material,
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
    fn new(app: &Rc<codevisual::App>, _: ()) -> Self {
        let context = app.ugli_context();

        Tutorial {
            material: codevisual::Material::new(
                context,
                (), (),
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
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(
            framebuffer,
            Some(Color::rgb(0.0, 0.0, 0.0)),
            None,
        );
        ugli::draw(
            framebuffer,
            &self.material.ugli_program(),
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
    codevisual::run::<Tutorial>();
}
