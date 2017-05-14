use gl;
use gl::types::*;
use std;
use std::os::raw::c_void;

pub trait Vertex
    where Self: Sized
{
    fn get_attributes() -> Vec<VertexAttribute>;
}

#[derive(Debug)]
pub struct VertexAttribute {
    pub name: std::ffi::CString,
    pub size: GLint,
    pub raw_size: GLsizei,
    pub gl_type: GLenum,
    pub normalized: GLboolean,
}

pub struct GeometryBuffer {
    pub mode: GLenum,
    pub handle: GLuint,
    pub element_count: GLsizei,
    pub attributes: Vec<VertexAttribute>,
}

impl GeometryBuffer {
    pub fn new<T: Vertex>(data: &[T]) -> Self {
        ::init().unwrap();
        unsafe {
            let mut handle: GLuint = std::mem::uninitialized();
            gl::GenBuffers(1, &mut handle);
            gl::BindBuffer(gl::ARRAY_BUFFER, handle);
            gl::BufferData(gl::ARRAY_BUFFER,
                           std::mem::size_of_val(data) as GLsizeiptr,
                           data.as_ptr() as *const c_void,
                           gl::STATIC_DRAW);
            Self {
                mode: gl::TRIANGLE_FAN,
                element_count: 4,
                handle,
                attributes: T::get_attributes(),
            }
        }
    }
}