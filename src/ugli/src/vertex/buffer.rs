use ::*;

struct RawBuffer {
    handle: GLuint,
    usage: GLenum,
    size: Cell<usize>,
}

impl RawBuffer {
    fn new(_: &Context, usage: GLenum) -> Self {
        Self {
            handle: unsafe {
                let mut handle: GLuint = mem::uninitialized();
                gl::GenBuffers(1, &mut handle);
                handle
            },
            usage,
            size: Cell::new(0),
        }
    }
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.handle);
        }
    }
    fn set_data<T>(&self, data: &Vec<T>) {
        self.bind();
        let size = mem::size_of::<T>() * data.capacity();
        let data = data.as_ptr();
        self.size.set(size);
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size as GLsizeiptr,
                data as *const c_void,
                self.usage,
            );
        }
    }
}

impl Drop for RawBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.handle);
        }
    }
}

pub struct VertexBuffer<T: Vertex> {
    buffer: RawBuffer,
    data: Vec<T>,
    need_update: Cell<bool>,
}

impl<T: Vertex> Deref for VertexBuffer<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.data
    }
}

impl<T: Vertex> DerefMut for VertexBuffer<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        self.need_update.set(true);
        &mut self.data
    }
}

impl<T: Vertex> VertexBuffer<T> {
    fn new(context: &Context, data: Vec<T>, usage: GLenum) -> Self {
        let buffer = RawBuffer::new(context, usage);
        buffer.set_data(&data);
        Self {
            buffer,
            data,
            need_update: Cell::new(false),
        }
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

    pub ( crate ) fn bind(&self) {
        if self.need_update.get() {
            self.buffer.set_data(&self.data);
            self.need_update.set(false);
        }
        self.buffer.bind();
    }
}

pub struct VertexBufferSlice<'a, T: Vertex + 'a> {
    pub ( crate ) buffer: &'a VertexBuffer<T>,
    pub ( crate ) range: Range<usize>,
}

impl<'a, T: Vertex + 'a> Deref for VertexBufferSlice<'a, T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.buffer.data[self.range.clone()]
    }
}

pub trait IntoVertexBufferSlice<'a, T: Vertex + 'a> {
    fn into_slice(self) -> VertexBufferSlice<'a, T>;
}

impl<'a, T: Vertex + 'a> IntoVertexBufferSlice<'a, T> for VertexBufferSlice<'a, T> {
    fn into_slice(self) -> VertexBufferSlice<'a, T> {
        self
    }
}

impl<'a, T: Vertex + 'a> IntoVertexBufferSlice<'a, T> for &'a VertexBufferSlice<'a, T> {
    fn into_slice(self) -> VertexBufferSlice<'a, T> {
        VertexBufferSlice {
            buffer: self.buffer,
            range: self.range.clone(),
        }
    }
}

impl<'a, T: Vertex + 'a> IntoVertexBufferSlice<'a, T> for &'a VertexBuffer<T> {
    fn into_slice(self) -> VertexBufferSlice<'a, T> {
        self.slice(..)
    }
}