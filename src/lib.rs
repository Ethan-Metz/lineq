//! Structs and methods for linear algebra.
//!
//! This crate provides 3 main types of structs that implement basic arithmetic operations 
//! like + and -, and also pervide useful methods:
//!
//! - Structs for handling vectors like [Vec2](crate::vec2::Vec2) and [Vec3](crate::vec3::Vec3).
//! - Structs for handling arrays of vectors with different types of memory orginization, like
//!   [Vec3arr](crate::vec3arr::Vec3arr) and [Vec3box](crate::vec3arr::Vec3box), for fixed length
//!   arrays and boxed arrays respectively.
//! - Structs for handling square matricies like [Mat22](crate::mat22::Mat22) and
//!   [Mat33](crate::mat33::Mat33).

#![feature(new_uninit)]
#![feature(slice_ptr_get)]
#![feature(test)]

pub mod vec2;
pub mod vec3;
pub mod mat22;
pub mod mat33;
pub mod vec3arr;
pub mod vec2arr;
mod arrmacro;
