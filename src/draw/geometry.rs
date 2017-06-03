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

pub struct Geometry<V: vertex::Data, I: vertex::Data = vertex::EmptyData> {
    handle: GLuint,
    pub mode: Mode,
    element_count: usize,
    instance_count: usize,
    data: PhantomData<V>,
    idata: PhantomData<I>,
}

impl<V: vertex::Data, I: vertex::Data> Geometry<V, I> {
    pub fn new(mode: Mode, vertices: &[V]) -> Result<Self, ::Error> {
        Self::check_element_count(mode, vertices)?;
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
               instance_count: 0,
               data: PhantomData,
               idata: PhantomData,
           })
    }

    pub fn new_instanced(mode: Mode, vertices: &[V], instances: &[I]) -> Result<Self, ::Error> {
        Self::check_element_count(mode, vertices)?;
        ::init()?;
        let handle = unsafe {
            let mut handle: GLuint = std::mem::uninitialized();
            gl::GenBuffers(1, &mut handle);
            gl::BindBuffer(gl::ARRAY_BUFFER, handle);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (std::mem::size_of_val(vertices) + std::mem::size_of_val(instances)) as
                           GLsizeiptr,
                           std::ptr::null(),
                           gl::STATIC_DRAW);
            gl::BufferSubData(gl::ARRAY_BUFFER,
                              0,
                              std::mem::size_of_val(vertices) as GLsizeiptr,
                              vertices.as_ptr() as *const c_void);
            gl::BufferSubData(gl::ARRAY_BUFFER,
                              std::mem::size_of_val(vertices) as GLintptr,
                              std::mem::size_of_val(instances) as GLsizeiptr,
                              instances.as_ptr() as *const c_void);
            handle
        };
        Ok(Self {
               handle,
               mode,
               element_count: vertices.len(),
               instance_count: instances.len(),
               data: PhantomData,
               idata: PhantomData,
           })
    }

    fn check_element_count(mode: Mode, vertices: &[V]) -> Result<(), ::Error> {
        Ok(()) // TODO: todo
    }

    pub fn len(&self) -> usize {
        self.element_count
    }

    pub fn get_instance_count(&self) -> usize {
        self.instance_count
    }

    pub fn get_handle(&self) -> GLuint {
        self.handle
    }
}