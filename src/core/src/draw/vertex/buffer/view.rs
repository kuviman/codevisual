use super::super::*;

use commons::*;
use std;

pub trait BufferView {
    type Data: Data;
    fn get_original_buffer(&self) -> &Buffer<Self::Data>;
    fn as_slice(&self) -> &[Self::Data];
    fn slice<'a, R: RangeArgument<usize>>(&'a self, range: R) -> BufferSlice<'a, Self::Data> {
        BufferSlice {
            original_buffer: self.get_original_buffer(),
            data: get_slice(self.as_slice(), range),
        }
    }
}

impl<D: Data> BufferView for Buffer<D> {
    type Data = D;
    fn get_original_buffer(&self) -> &Buffer<D> {
        self
    }
    fn as_slice(&self) -> &[D] {
        self.data.as_slice()
    }
}

pub struct BufferSlice<'a, D: 'a + Data> {
    original_buffer: &'a Buffer<D>,
    data: &'a [D],
}

impl<'a, D: 'a + Data> BufferView for BufferSlice<'a, D> {
    type Data = D;
    fn get_original_buffer(&self) -> &Buffer<Self::Data> {
        self.original_buffer
    }
    fn as_slice(&self) -> &[D] {
        self.data
    }
}

impl<D: Data> std::ops::Index<usize> for Buffer<D> {
    type Output = D;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}