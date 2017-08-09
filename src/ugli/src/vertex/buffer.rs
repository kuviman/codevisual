use ::*;

pub struct VertexBuffer<T: VertexData> {
    pub ( crate ) handle: GLuint,
    data: Vec<T>,
}

impl<T: VertexData> Drop for VertexBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.handle);
        }
    }
}

impl<T: VertexData> VertexBuffer<T> {
    fn new(_: &Context, data: Vec<T>, usage: GLenum) -> Self {
        let buffer = Self {
            handle: unsafe {
                let mut handle: GLuint = std::mem::uninitialized();
                gl::GenBuffers(1, &mut handle);
                handle
            },
            data,
        };
        buffer.bind();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(buffer.data.as_slice()) as GLsizeiptr,
                buffer.data.as_ptr() as *const c_void,
                usage,
            );
        }
        buffer
    }

    pub fn new_static(context: &Context, data: Vec<T>) -> Self {
        Self::new(context, data, gl::STATIC_DRAW)
    }

    pub fn new_dynamic(context: &Context, data: Vec<T>) -> Self {
        Self::new(context, data, gl::DYNAMIC_DRAW)
    }

    pub fn slice<'a, R>(&'a self, range: R) -> VertexBufferSlice<'a, T>
        where
            R: RangeArgument<usize>,
    {
        VertexBufferSlice {
            buffer: self,
            range: Range {
                start: *range.start().unwrap_or(&0),
                end: *range.end().unwrap_or(&self.data.len()),
            },
        }
    }

    pub fn slice_mut<'a, R>(&'a mut self, range: R) -> VertexBufferSliceMut<'a, T>
        where
            R: RangeArgument<usize>,
    {
        let end = *range.end().unwrap_or(&self.data.len());
        VertexBufferSliceMut {
            buffer: self,
            range: Range {
                start: *range.start().unwrap_or(&0),
                end,
            },
        }
    }

    pub ( crate ) fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.handle);
        }
    }
}

pub struct VertexBufferSlice<'a, T: VertexData + 'a> {
    pub ( crate ) buffer: &'a VertexBuffer<T>,
    pub ( crate ) range: Range<usize>,
}

impl<'a, T: VertexData + 'a> Deref for VertexBufferSlice<'a, T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.buffer.data[self.range.clone()]
    }
}

pub struct VertexBufferSliceMut<'a, T: VertexData + 'a> {
    buffer: &'a mut VertexBuffer<T>,
    range: Range<usize>,
}

impl<'a, T: VertexData + 'a> Deref for VertexBufferSliceMut<'a, T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.buffer.data[self.range.clone()]
    }
}

impl<'a, T: VertexData + 'a> DerefMut for VertexBufferSliceMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer.data[self.range.clone()]
    }
}

impl<'a, T: VertexData> Drop for VertexBufferSliceMut<'a, T> {
    fn drop(&mut self) {
        self.buffer.bind();
        let data = &self.buffer.data[self.range.clone()];
        unsafe {
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                (self.range.start * std::mem::size_of::<T>()) as GLintptr,
                std::mem::size_of_val(data) as GLsizeiptr,
                data.as_ptr() as *const c_void,
            );
        }
    }
}
