use super::super::*;

use std;
use gl;
use gl::types::*;

pub struct BufferElement<'a, D: 'a + Data> {
    buffer: &'a mut Buffer<D>,
    index: usize,
}

impl<D: Data> Buffer<D> {
    pub fn index_mut<'a>(&'a mut self, index: usize) -> BufferElement<'a, D> {
        BufferElement {
            buffer: self,
            index,
        }
    }
}

impl<'a, D: Data> std::ops::Deref for BufferElement<'a, D> {
    type Target = D;
    fn deref(&self) -> &Self::Target {
        &self.buffer.data[self.index]
    }
}

impl<'a, D: Data> std::ops::DerefMut for BufferElement<'a, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer.data[self.index]
    }
}

impl<'a, D: Data> Drop for BufferElement<'a, D> {
    fn drop(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer.handle);
            gl::BufferSubData(gl::ARRAY_BUFFER,
                              (self.index * std::mem::size_of::<D>()) as GLintptr,
                              std::mem::size_of::<D>() as GLsizeiptr,
                              &**self as *const _ as *const _);
        }
    }
}

impl<D: Data> std::ops::Index<usize> for Buffer<D> {
    type Output = D;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}