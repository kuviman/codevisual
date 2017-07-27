use ::*;

mod raw {
    use ::*;
    pub struct AttributeType {
        pub gl_size: GLsizei,
        pub gl_type: GLenum,
    }
}

pub trait VertexAttribute {
    fn get_gl_type() -> raw::AttributeType;
}

impl VertexAttribute for f32 {
    fn get_gl_type() -> raw::AttributeType {
        raw::AttributeType {
            gl_size: 1,
            gl_type: gl::FLOAT,
        }
    }
}

impl VertexAttribute for Vec2<f32> {
    fn get_gl_type() -> raw::AttributeType {
        raw::AttributeType {
            gl_size: 2,
            gl_type: gl::FLOAT,
        }
    }
}

impl VertexAttribute for Vec3<f32> {
    fn get_gl_type() -> raw::AttributeType {
        raw::AttributeType {
            gl_size: 3,
            gl_type: gl::FLOAT,
        }
    }
}

impl VertexAttribute for Color {
    fn get_gl_type() -> raw::AttributeType {
        raw::AttributeType {
            gl_size: 4,
            gl_type: gl::FLOAT,
        }
    }
}