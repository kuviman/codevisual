use gl;
use gl::types::*;
use std;
use std::os::raw::c_void;
use std::marker::PhantomData;

use super::vertex;

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Points,
    Lines,
    LineStrip,
    Triangles,
    TriangleFan,
    TriangleStrip,
}

pub struct Geometry<V: vertex::Data> {
    handle: GLuint,
    pub mode: Mode,
    element_count: usize,
    data: PhantomData<V>,
}

impl<V: vertex::Data> Geometry<V> {
    pub fn new(mode: Mode, vertices: &[V]) -> Result<Self, ::Error> {
        // TODO: check element count for mode
        ::init()?;
        let handle = unsafe {
            let mut handle: GLuint = std::mem::uninitialized();
            gl::GenBuffers(1, &mut handle);
            gl::BindBuffer(gl::ARRAY_BUFFER, handle);
            gl::BufferData(gl::ARRAY_BUFFER,
                           std::mem::size_of_val(vertices) as GLsizeiptr,
                           vertices.as_ptr() as *const c_void,
                           gl::STATIC_DRAW);
            handle
        };
        Ok(Self {
               handle,
               mode,
               element_count: vertices.len(),
               data: PhantomData,
           })
    }

    pub fn len(&self) -> usize {
        self.element_count
    }

    pub fn get_handle(&self) -> GLuint {
        self.handle
    }
}