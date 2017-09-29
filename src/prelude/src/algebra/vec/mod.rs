use ::*;

mod vec2;

pub use self::vec2::*;

mod vec3;

pub use self::vec3::*;

mod vec4;

pub use self::vec4::*;

macro_rules! vec_impl_ops {
    ($name:ident : $($f:ident),*) => {
        impl<T: Add<Output=T>> Add for $name<T> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self {
                    $($f: self.$f + rhs.$f,)*
                }
            }
        }
        
        impl<T: AddAssign> AddAssign for $name<T> {
            fn add_assign(&mut self, rhs: Self) {
                $(self.$f += rhs.$f;)*
            }
        }
        
        impl<T: Sub<Output=T>> Sub for $name<T> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                Self {
                    $($f: self.$f - rhs.$f,)*
                }
            }
        }
        
        impl<T: SubAssign> SubAssign for $name<T> {
            fn sub_assign(&mut self, rhs: Self) {
                $(self.$f -= rhs.$f;)*
            }
        }
        
        impl<T: Neg<Output=T>> Neg for $name<T> {
            type Output = Self;
            fn neg(self) -> Self {
                Self {
                    $($f: -self.$f,)*
                }
            }
        }
        
        impl<T: Copy + Mul<Output=T>> Mul<T> for $name<T> {
            type Output = $name<T>;
            fn mul(self, rhs: T) -> $name<T> {
                $name {
                    $($f: self.$f * rhs,)*
                }
            }
        }
        
        impl<T: Copy + MulAssign> MulAssign<T> for $name<T> {
            fn mul_assign(&mut self, rhs: T) {
                $(self.$f *= rhs;)*
            }
        }
        
        impl<T: Copy + Div<Output=T>> Div<T> for $name<T> {
            type Output = $name<T>;
            fn div(self, rhs: T) -> $name<T> {
                $name {
                    $($f: self.$f / rhs,)*
                }
            }
        }
        
        impl<T: Copy + DivAssign> DivAssign<T> for $name<T> {
            fn div_assign(&mut self, rhs: T) {
                $(self.$f /= rhs;)*
            }
        }
    };
}

vec_impl_ops!(Vec2: x, y);
vec_impl_ops!(Vec3: x, y, z);
vec_impl_ops!(Vec4: x, y, z, w);