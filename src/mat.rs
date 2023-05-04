//! Module containing matricies.

use crate::vec2::Vec2;

/// A two by two square matrix.
///
/// # Examples
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat22 {
        pub x1 : f32,
        pub y1 : f32,
	pub x2 : f32,
	pub y2 : f32,
}

//Add
use std::ops::Add;

impl Add<f32> for Mat22 {
        type Output = Mat22;
        fn add(self, rhs: f32) -> Mat22 {
                Mat22 { x1: self.x1 + rhs, y1: self.y1 + rhs, x2: self.x2 + rhs, y2: self.y2 + rhs }
        }
}

impl Add<Mat22> for f32 {
        type Output = Mat22;
        fn add(self, rhs: Mat22) -> Mat22 {
                Mat22 { x1: self + rhs.x1, y1: self + rhs.y1, x2: self + rhs.x2, y2: self + rhs.y2 }
        }
}

impl Add<Mat22> for Mat22 {
        type Output = Mat22;
        fn add(self, rhs: Mat22) -> Mat22 {
                Mat22 { x1: self.x1 + rhs.x1, y1: self.y1 + rhs.y1, x2: self.x2 + rhs.x2, y2: self.y2 + rhs.y2 }
        }
}

//AddAssign
use std::ops::AddAssign;

impl AddAssign<f32> for Mat22 {
        fn add_assign(&mut self, rhs: f32) {
                self.x1 += rhs;
                self.y1 += rhs;
		self.x2 += rhs;
		self.y2 += rhs;
        }
}

impl AddAssign<Mat22> for Mat22 {
        fn add_assign(&mut self, rhs: Mat22) {
                self.x1 += rhs.x1;
                self.y1 += rhs.y1;
		self.x2 += rhs.x2;
		self.y2 += rhs.y2;
        }
}

//Display
use std::fmt;

impl fmt::Display for Mat22 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "[[{}, {}][{}, {}]]", self.x1, self.x2, self.y1, self.y2)
        }
}

//Div
use std::ops::Div;

impl Div<f32> for Mat22 {
        type Output = Mat22;
        fn div(self, rhs: f32) -> Mat22 {
                Mat22 { x1: self.x1/rhs, y1: self.y1/rhs, x2: self.x2/rhs, y2: self.y2/rhs }
        }
}

//DivAssign
use std::ops::DivAssign;

impl DivAssign<f32> for Mat22 {
        fn div_assign(&mut self, rhs: f32) {
                self.x1 /= rhs;
                self.y1 /= rhs;
		self.x2 /= rhs;
		self.y2 /= rhs;
        }
}

//Mult
use std::ops::Mul;

impl Mul<f32> for Mat22 {
        type Output = Mat22;
        fn mul(self, rhs: f32) -> Mat22 {
                Mat22 { x1: self.x1 * rhs, y1: self.y1 * rhs, x2: self.x2 * rhs, y2: self.y2 * rhs }
        }
}

impl Mul<Mat22> for f32 {
        type Output = Mat22;
        fn mul(self, rhs: Mat22) -> Mat22 {
                Mat22 { x1: self*rhs.x1, y1: self*rhs.y1, x2: self*rhs.x2, y2: self*rhs.y2 }
        }
}

impl Mul<Mat22> for Mat22 {
        type Output = Mat22;
        fn mul(self, rhs: Mat22) -> Mat22 {
                Mat22 { x1: self.x1*rhs.x1 + self.x2*rhs.y1, y1: self.y1*rhs.x1 + self.y2*rhs.y1, x2: self.x1*rhs.x2 + self.x2*rhs.y2, y2: self.y1*rhs.x2 + self.y2*rhs.y2 }
        }
}

impl Mul<Vec2> for Mat22 {
	type Output = Vec2;
	fn mul(self, rhs: Vec2) -> Vec2 {
		Vec2 { x: self.x1*rhs.x + self.x2*rhs.y , y: self.y1*rhs.x + self.y2*rhs.y }
	}
}

//MultAssign
use std::ops::MulAssign;

impl MulAssign<f32> for Mat22 {
        fn mul_assign(&mut self, rhs: f32) {
                self.x1 *= rhs;
                self.y1 *= rhs;
		self.x2 *= rhs;
		self.y2 *= rhs;
        }
}

impl MulAssign<Mat22> for Mat22 {
	fn mul_assign(&mut self, rhs : Mat22) {
		self.x1 = self.x1*rhs.x1 + self.x2*rhs.y1;
		self.y1 = self.y1*rhs.x1 + self.y2*rhs.y1;
		self.x2 = self.x1*rhs.x2 + self.x2*rhs.y2;
		self.y2 = self.y1*rhs.x2 + self.y2*rhs.y2;
	}
}

//Neg
use std::ops::Neg;

impl Neg for Mat22 {
        type Output = Mat22;
        fn neg(self) -> Mat22 {
                Mat22 { x1: -self.x1, y1: -self.y1, x2: -self.x2, y2: -self.y2 }
        }
}

//Sub
use std::ops::Sub;

impl Sub<f32> for Mat22 {
        type Output = Mat22;
        fn sub(self, rhs: f32) -> Mat22 {
                Mat22 { x1: self.x1 - rhs, y1: self.y1 - rhs, x2: self.x2 - rhs, y2: self.y2 - rhs }
        }
}

impl Sub<Mat22> for Mat22 {
        type Output = Mat22;
        fn sub(self, rhs: Mat22) -> Mat22 {
                Mat22 { x1: self.x1 - rhs.x1, y1: self.y1 - rhs.y1, x2: self.x2 - rhs.x2, y2: self.y2 - rhs.y2 }
        }
}

//SubAssign
use std::ops::SubAssign;

impl SubAssign<f32> for Mat22 {
        fn sub_assign(&mut self, rhs: f32) {
                self.x1 -= rhs;
                self.y1 -= rhs;
		self.x2 -= rhs;
		self.y2 -= rhs;
        }
}

impl SubAssign<Mat22> for Mat22 {
        fn sub_assign(&mut self, rhs: Mat22) {
                self.x1 -= rhs.x1;
                self.y1 -= rhs.y1;
		self.x2 -= rhs.x2;
		self.y2 -= rhs.y2;
        }
}

//Mat22 Methods

impl Mat22 {
	pub const IDENTITY : Mat22 = Mat22 { x1: 1.0, y1: 0.0, x2: 0.0, y2: 1.0 };
	pub const ZERO : Mat22 = Mat22 { x1: 0.0, y1: 0.0, x2: 0.0, y2: 0.0 };
	
	pub fn augment(v1 : Vec2, v2 : Vec2) -> Mat22 {
		Mat22 { x1: v1.x, y1: v1.y, x2: v2.x, y2: v2.y }
	}

	pub fn det(&self) -> f32 {
                self.x1*self.y2-self.y1*self.x2
	}

        pub fn inverse(&self) -> Mat22 {
		let det : f32 = self.x1*self.y2-self.y1*self.x2;
		if det == 0.0 { panic!("non-invertible matrix"); }
		Mat22 { x1: self.y2/det, y1: -self.y1/det, x2: -self.x2/det, y2: self.x1/det }
	}

	pub fn t(&self) -> Mat22 {
		Mat22 { x1: self.x1, y1: self.x2, x2: self.y1, y2: self.y2 }
	}
}
