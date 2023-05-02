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
//!
//! Vectors can be easily initialized and used:
//! ```rust
//! # extern crate lineq;
//! use lineq::vec3::Vec3;
//! let a : Vec3 = Vec3::UP;
//! let b : Vec3 = Vec3 { x: -1.0, y: 0.0, z: 0.0 };
//! assert_eq!(a.cross(b),Vec3::IN);
//! ```
//!
//! Arrays of vectors can be used to parallelize arithmetic:
//! ```rust
//! # extern crate lineq;
//! use lineq::vec3::Vec3;
//! use lineq::vec3arr::Vec3arr;
//! //using a and b from last example:
//! # let a : Vec3 = Vec3::UP;
//! # let b : Vec3 = Vec3::LEFT;
//! let ab : Vec3arr<2>= Vec3arr([a,b]);
//! let cd : Vec3arr<2>= Vec3arr([Vec3::DOWN,Vec3::RIGHT]);
//! assert_eq!(ab + cd, Vec3arr([Vec3::ZERO,Vec3::ZERO]));
//! ```
//!
//! When adding arrays, the resulting type is deturmined by 
//! how structured the type is, so for example from most
//! structured to least structured we have Vec3arr, then
//! Vec3box, then Vec3win, then finally Vec3raw.
//!
//! Only Vec2arr/Vec3arr and Vec2box/Vec3box have allocators,
//! so when performing an operation that allocates (like +, -,
//! *, /) one of the types needs to be a Vec2arr/Vec3arr or 
//! Vec2box/Vec3box.
//!
//! Matricies are indexed like x1, y2, z3 ... where x, y, z
//! are the rows and 1, 2, 3 are the columns:
//! ```rust
//! # extern crate lineq;
//! use lineq::vec3::Vec3;
//! use lineq::mat33::Mat33;
//! //using a and b from first example:
//! # let a : Vec3 = Vec3::UP;
//! # let b : Vec3 = Vec3::LEFT;
//! let c : Vec3 = Vec3{ x: 1.0, y: -1.0, z: 2.0 };
//! let m1 : Mat33 = Mat33::augment(a,b,c);
//! let m2 : Mat33 = Mat33{
//! x1: 0.0, x2: -1.0, x3: 1.0,
//! y1: 1.0, y2: 0.0, y3: -1.0,
//! z1: 0.0, z2: 0.0, z3: 2.0 };
//! assert_eq!(m1,m2);
//! ```

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
