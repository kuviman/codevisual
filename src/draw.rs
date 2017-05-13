use gl;

pub fn clear(r: f32, g: f32, b: f32) {
    unsafe {
        gl::ClearColor(r, g, b, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}