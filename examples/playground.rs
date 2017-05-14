pub extern crate codevisual;

use codevisual::common::*;
use codevisual::draw;

struct Test {
    current_time: f32,
    geometry: draw::GeometryBuffer,
    shader: draw::Shader,
}

#[repr(C)]
struct Vertex(f32, f32);

extern crate gl;
use gl::types::*;

impl draw::Vertex for Vertex {
    fn get_attributes() -> Vec<draw::VertexAttribute> {
        vec![draw::VertexAttribute {
                 name: std::ffi::CString::new("a_pos").unwrap(),
                 size: 2,
                 raw_size: std::mem::size_of::<GLfloat>() as GLsizei * 2,
                 gl_type: gl::FLOAT,
                 normalized: gl::FALSE,
             }]
    }
}

impl Test {
    fn new() -> Self {
        Self {
            current_time: 0.0,
            shader: codevisual::draw::Shader::compile(include_str!("vertex.glsl"),
                                                      include_str!("fragment.glsl"))
                    .unwrap(),
            geometry: codevisual::draw::GeometryBuffer::new(&[Vertex(0.0, 0.0),
                                                              Vertex(1.0, 0.0),
                                                              Vertex(1.0, 1.0),
                                                              Vertex(0.0, 1.0)]),
        }
    }
}

impl codevisual::Game for Test {
    fn update(&mut self, delta_time: f32) {
        self.current_time += delta_time;
    }
    fn render(&mut self) -> Vec<draw::Command> {
        vec![draw::Command::Clear { color: Color::rgb(self.current_time.fract(), 0.8, 1.0) },
             draw::Command::Object {
                 geometry: &self.geometry,
                 shader: &self.shader,
             }]
    }
}

fn main() {
    codevisual::run(Test::new());

    // Hack (WTF??)
    unsafe {
        codevisual::platform::ffi::emscripten_GetProcAddress(std::ffi::CString::new("abacaba")
                                                                 .unwrap()
                                                                 .into_raw() as
                                                             *const _);
    }
}