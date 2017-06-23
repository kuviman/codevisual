use super::*;

use std;
use std::os::raw::c_void;
use gl;
use gl::types::*;

mod indexing;
pub use self::indexing::*;

mod slicing;
pub use self::slicing::*;

pub struct Buffer<D: Data> {
    pub(crate) handle: GLuint,
    data: Vec<D>,
}

impl<D: Data> Buffer<D> {
    pub fn new(data: Vec<D>) -> Self {
        ::Application::get_instance();
        let handle = unsafe {
            let mut handle: GLuint = std::mem::uninitialized();
            gl::GenBuffers(1, &mut handle);
            gl::BindBuffer(gl::ARRAY_BUFFER, handle);
            gl::BufferData(gl::ARRAY_BUFFER,
                           std::mem::size_of_val(data.as_slice()) as GLsizeiptr,
                           data.as_slice().as_ptr() as *const c_void,
                           gl::STATIC_DRAW);
            handle
        };
        Self { handle, data }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
