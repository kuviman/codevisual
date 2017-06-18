use std;
use std::os::raw::c_void;

use common::*;

use gl::types::*;
use gl;

pub trait Attribute: Copy + Sized {
    fn get_gl_size() -> GLsizei;
    fn get_gl_type() -> GLenum;
}

impl Attribute for f32 {
    fn get_gl_size() -> GLsizei {
        1
    }
    fn get_gl_type() -> GLenum {
        gl::FLOAT
    }
}
impl Attribute for Vec2<f32> {
    fn get_gl_size() -> GLsizei {
        2
    }
    fn get_gl_type() -> GLenum {
        gl::FLOAT
    }
}
impl Attribute for Vec3<f32> {
    fn get_gl_size() -> GLsizei {
        3
    }
    fn get_gl_type() -> GLenum {
        gl::FLOAT
    }
}
impl Attribute for Color {
    fn get_gl_size() -> GLsizei {
        4
    }
    fn get_gl_type() -> GLenum {
        gl::FLOAT
    }
}

pub trait AttributeConsumer {
    fn consume<A: Attribute>(&mut self, name: &str, value: &A);
}

pub trait Data: Sized {
    fn walk_attributes<F: AttributeConsumer>(&self, f: &mut F);
}

pub struct EmptyData;
impl Data for EmptyData {
    fn walk_attributes<F: AttributeConsumer>(&self, _: &mut F) {}
}

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
    pub fn index_mut<'a>(&'a mut self, index: usize) -> BufferElement<'a, D> {
        BufferElement {
            buffer: self,
            index,
        }
    }
}

pub struct BufferElement<'a, D: 'a + Data> {
    buffer: &'a mut Buffer<D>,
    index: usize,
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