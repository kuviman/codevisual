use super::super::*;
use commons::*;

use std;
use gl;
use gl::types::*;

pub struct BufferElementMut<'a, D: 'a + Data> {
    buffer: &'a mut Buffer<D>,
    index: usize,
}

impl<D: Data> Buffer<D> {
    pub fn index_mut<'a>(&'a mut self, index: usize) -> BufferElementMut<'a, D> {
        BufferElementMut {
            buffer: self,
            index,
        }
    }
}

impl<'a, D: Data> std::ops::Deref for BufferElementMut<'a, D> {
    type Target = D;
    fn deref(&self) -> &Self::Target {
        &self.buffer.data[self.index]
    }
}

impl<'a, D: Data> std::ops::DerefMut for BufferElementMut<'a, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer.data[self.index]
    }
}

impl<'a, D: Data> Drop for BufferElementMut<'a, D> {
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

pub struct BufferSliceMut<'a, D: 'a + Data> {
    buffer: &'a mut Buffer<D>,
    start: usize,
    end: usize,
}

impl<'a, D: Data> std::ops::Deref for BufferSliceMut<'a, D> {
    type Target = [D];
    fn deref(&self) -> &Self::Target {
        &self.buffer.data[self.start..self.end]
    }
}

impl<'a, D: Data> std::ops::DerefMut for BufferSliceMut<'a, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer.data[self.start..self.end]
    }
}

impl<'a, D: Data> Drop for BufferSliceMut<'a, D> {
    fn drop(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer.handle);
            gl::BufferSubData(gl::ARRAY_BUFFER,
                              (self.start * std::mem::size_of::<D>()) as GLintptr,
                              ((self.end - self.start) * std::mem::size_of::<D>()) as GLsizeiptr,
                              self.as_ptr() as *const _ as *const _);
        }
    }
}

impl<D: Data> Buffer<D> {
    pub fn slice_mut<'a, R: RangeArgument<usize>>(&'a mut self, range: R) -> BufferSliceMut<'a, D> {
        let start = *range.start().unwrap_or(&0);
        let end = *range.end().unwrap_or(&self.len());
        BufferSliceMut {
            buffer: self,
            start,
            end,
        }
    }
}