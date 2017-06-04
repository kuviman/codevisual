pub mod color;
pub use self::color::*;

use cgmath;

pub type Vec2 = cgmath::Vector2<f32>;
pub type Vec3 = cgmath::Vector3<f32>;
pub type Vec4 = cgmath::Vector4<f32>;

pub type Mat3 = cgmath::Matrix3<f32>;
pub type Mat4 = cgmath::Matrix4<f32>;

pub use cgmath::{vec2, vec3, vec4, dot, ortho, perspective};
pub use cgmath::InnerSpace as cgInnerSpace;