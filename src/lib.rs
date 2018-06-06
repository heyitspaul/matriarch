//! # Matriarch
//!
//! Matriarch is a Linear Algebra and Matrix library written in pure Rust.

#[doc(hidden)]
pub mod mat2;
#[doc(hidden)]
pub mod mat3;
#[doc(hidden)]
pub mod mat4;
#[doc(hidden)]
pub mod vec2;
#[doc(hidden)]
pub mod vec3;
#[doc(hidden)]
pub mod vec4;

#[doc(inline)]
pub use mat2::Mat2;
#[doc(inline)]
pub use mat3::Mat3;
#[doc(inline)]
pub use mat4::Mat4;
#[doc(inline)]
pub use vec2::Vec2;
#[doc(inline)]
pub use vec3::Vec3;
#[doc(inline)]
pub use vec4::Vec4;
