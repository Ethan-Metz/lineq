//! Module containing matricies.

use crate::vec2::Vec2;
use crate::vec3::Vec3;

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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat33 {
        pub x1 : f32,
        pub y1 : f32,
	pub z1 : f32,
	pub x2 : f32,
	pub y2 : f32,
	pub z2 : f32,
	pub x3 : f32,
	pub y3 : f32,
	pub z3 : f32,
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

impl Add<f32> for Mat33 {
        type Output = Mat33;
        fn add(self, rhs: f32) -> Mat33 {
                Mat33 { x1: self.x1 + rhs, y1: self.y1 + rhs, z1: self.z1 + rhs, 
			x2: self.x2 + rhs, y2: self.y2 + rhs, z2: self.z2 + rhs,
			x3: self.x3 + rhs, y3: self.y3 + rhs, z3: self.z3 + rhs }
        }
}

impl Add<Mat33> for f32 {
        type Output = Mat33;
        fn add(self, rhs: Mat33) -> Mat33 {
		Mat33 { x1: self + rhs.x1, y1: self + rhs.y1, z1: self + rhs.z1,
                        x2: self + rhs.x2, y2: self + rhs.y2, z2: self + rhs.z2,
                        x3: self + rhs.x3, y3: self + rhs.y3, z3: self + rhs.z3 }
        }
}

impl Add<Mat33> for Mat33 {
        type Output = Mat33;
        fn add(self, rhs: Mat33) -> Mat33 {
		Mat33 { x1: self.x1 + rhs.x1, y1: self.y1 + rhs.y1, z1: self.z1 + rhs.z1,
                        x2: self.x2 + rhs.x2, y2: self.y2 + rhs.y2, z2: self.z2 + rhs.z2,
                        x3: self.x3 + rhs.x3, y3: self.y3 + rhs.y3, z3: self.z3 + rhs.z3 }
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

impl AddAssign<f32> for Mat33 {
        fn add_assign(&mut self, rhs: f32) {
                self.x1 += rhs; self.y1 += rhs; self.z1 += rhs;
		self.x2 += rhs; self.y2 += rhs; self.z2 += rhs;
		self.x3 += rhs; self.y3 += rhs; self.z3 += rhs;
        }
}

impl AddAssign<Mat33> for Mat33 {
        fn add_assign(&mut self, rhs: Mat33) {
		self.x1 += rhs.x1; self.y1 += rhs.y1; self.z1 += rhs.z1;
                self.x2 += rhs.x2; self.y2 += rhs.y2; self.z2 += rhs.z2;
                self.x3 += rhs.x3; self.y3 += rhs.y3; self.z3 += rhs.z3;
        }
}

//Display
use std::fmt;

impl fmt::Display for Mat22 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "[[{}, {}][{}, {}]]", self.x1, self.x2, self.y1, self.y2)
        }
}

impl fmt::Display for Mat33 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "[[{}, {}, {}][{}, {}, {}][{}, {}, {}]]", self.x1, self.x2, self.x3, self.y1, self.y2, self.y3, self.z1, self.z2, self.z3)
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

impl Div<f32> for Mat33 {
        type Output = Mat33;
        fn div(self, rhs: f32) -> Mat33 {
                Mat33 { x1: self.x1/rhs, y1: self.y1/rhs, z1: self.z1/rhs,
                        x2: self.x2/rhs, y2: self.y2/rhs, z2: self.z2/rhs,
                        x3: self.x3/rhs, y3: self.y3/rhs, z3: self.z3/rhs }
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

impl DivAssign<f32> for Mat33 {
        fn div_assign(&mut self, rhs: f32) {
                self.x1 /= rhs; self.y1 /= rhs; self.z1 /= rhs;
                self.x2 /= rhs; self.y2 /= rhs; self.z2 /= rhs;
                self.x3 /= rhs; self.y3 /= rhs; self.z3 /= rhs;
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

impl Mul<f32> for Mat33 {
        type Output = Mat33;
        fn mul(self, rhs: f32) -> Mat33 {
                Mat33 { x1: self.x1 * rhs, y1: self.y1 * rhs, z1: self.z1 * rhs,
                        x2: self.x2 * rhs, y2: self.y2 * rhs, z2: self.z2 * rhs,
                        x3: self.x3 * rhs, y3: self.y3 * rhs, z3: self.z3 * rhs }
        }
}

impl Mul<Mat33> for f32 {
        type Output = Mat33;
        fn mul(self, rhs: Mat33) -> Mat33 {
                Mat33 { x1: self * rhs.x1, y1: self * rhs.y1, z1: self * rhs.z1,
                        x2: self * rhs.x2, y2: self * rhs.y2, z2: self * rhs.z2,
                        x3: self * rhs.x3, y3: self * rhs.y3, z3: self * rhs.z3 }
        }
}

impl Mul<Mat33> for Mat33 {
        type Output = Mat33;
        fn mul(self, rhs: Mat33) -> Mat33 {
                Mat33 { x1: self.x1*rhs.x1 + self.x2*rhs.y1 + self.x3*rhs.z1, y1: self.y1*rhs.x1 + self.y2*rhs.y1 + self.y3*rhs.z1, z1: self.z1*rhs.x1 + self.z2*rhs.y1 + self.z3*rhs.z1,
                        x2: self.x1*rhs.x2 + self.x2*rhs.y2 + self.x3*rhs.z2, y2: self.y1*rhs.x2 + self.y2*rhs.y2 + self.y3*rhs.z2, z2: self.z1*rhs.x2 + self.z2*rhs.y2 + self.z3*rhs.z2,
                        x3: self.x1*rhs.x3 + self.x2*rhs.y3 + self.x3*rhs.z3, y3: self.y1*rhs.x3 + self.y2*rhs.y3 + self.y3*rhs.z3, z3: self.x1*rhs.x3 + self.z2*rhs.y3 + self.z3*rhs.z3, }
        }
}

impl Mul<Vec3> for Mat33 {
	type Output = Vec3;
	fn mul(self, rhs: Vec3) -> Vec3 {
		Vec3 { x: self.x1*rhs.x + self.x2*rhs.y + self.x3*rhs.z, y: self.y1*rhs.x + self.y2*rhs.y + self.y3*rhs.z, z: self.z1*rhs.x + self.z2*rhs.y + self.z3*rhs.z }
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

impl MulAssign<f32> for Mat33 {
        fn mul_assign(&mut self, rhs: f32) {
                self.x1 *= rhs; self.y1 *= rhs; self.z1 *= rhs;
                self.x2 *= rhs; self.y2 *= rhs; self.z2 *= rhs;
                self.x3 *= rhs; self.y3 *= rhs; self.z3 *= rhs;
        }
}

impl MulAssign<Mat33> for Mat33 {
	fn mul_assign(&mut self, rhs : Mat33) {
		self.x1 = self.x1*rhs.x1 + self.x2*rhs.y1 + self.x3*rhs.z1; self.y1 = self.y1*rhs.x1 + self.y2*rhs.y1 + self.y3*rhs.z1; self.z1 = self.z1*rhs.x1 + self.z2*rhs.y1 + self.z3*rhs.z1;
                self.x2 = self.x1*rhs.x2 + self.x2*rhs.y2 + self.x3*rhs.z2; self.y2 = self.y1*rhs.x2 + self.y2*rhs.y2 + self.y3*rhs.z2; self.z2 = self.z1*rhs.x2 + self.z2*rhs.y2 + self.z3*rhs.z2;
                self.x3 = self.x1*rhs.x3 + self.x2*rhs.y3 + self.x3*rhs.z3; self.y3 = self.y1*rhs.x3 + self.y2*rhs.y3 + self.y3*rhs.z3; self.z3 = self.x1*rhs.x3 + self.z2*rhs.y3 + self.z3*rhs.z3;
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

impl Neg for Mat33 {
        type Output = Mat33;
        fn neg(self) -> Mat33 {
                Mat33 { x1: -self.x1, y1: -self.y1, z1: -self.z1,
                        x2: -self.x2, y2: -self.y2, z2: -self.z2,
                        x3: -self.x3, y3: -self.y3, z3: -self.z3 }
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

impl Sub<f32> for Mat33 {
        type Output = Mat33;
        fn sub(self, rhs: f32) -> Mat33 {
                Mat33 { x1: self.x1 - rhs, y1: self.y1 - rhs, z1: self.z1 - rhs,
                        x2: self.x2 - rhs, y2: self.y2 - rhs, z2: self.z2 - rhs,
                        x3: self.x3 - rhs, y3: self.y3 - rhs, z3: self.z3 - rhs }
        }
}

impl Sub<Mat33> for Mat33 {
        type Output = Mat33;
        fn sub(self, rhs: Mat33) -> Mat33 {
		Mat33 { x1: self.x1 - rhs.x1, y1: self.y1 - rhs.y1, z1: self.z1 - rhs.z1,
                        x2: self.x2 - rhs.x2, y2: self.y2 - rhs.y2, z2: self.z2 - rhs.z2,
                        x3: self.x3 - rhs.x3, y3: self.y3 - rhs.y3, z3: self.z3 - rhs.z3 }
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

impl SubAssign<f32> for Mat33 {
        fn sub_assign(&mut self, rhs: f32) {
                self.x1 -= rhs; self.y1 -= rhs; self.z1 -= rhs;
                self.x2 -= rhs; self.y2 -= rhs; self.z2 -= rhs;
                self.x3 -= rhs; self.y3 -= rhs; self.z3 -= rhs;
        }
}

impl SubAssign<Mat33> for Mat33 {
        fn sub_assign(&mut self, rhs: Mat33) {
                self.x1 -= rhs.x1; self.y1 -= rhs.y1; self.z1 -= rhs.z1;
                self.x2 -= rhs.x2; self.y2 -= rhs.y2; self.z2 -= rhs.z2;
                self.x3 -= rhs.x3; self.y3 -= rhs.y3; self.z3 -= rhs.z3;
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

//Mat33 Methods

impl Mat33 {
	pub const IDENTITY : Mat33 = 	Mat33 { x1: 1.0, y1: 0.0, z1: 0.0, 
						x2: 0.0, y2: 1.0, z2: 0.0,
						x3: 0.0, y3: 0.0, z3: 1.0 };
	pub const ZERO : Mat33 = 	Mat33 { x1: 0.0, y1: 0.0, z1: 0.0, 
						x2: 0.0, y2: 0.0, z2: 0.0,
						x3: 0.0, y3: 0.0, z3: 0.0 };
	
	pub fn adj(&self) -> Mat33 {
                Mat33 { x1: self.y2*self.z3-self.y3*self.z2, y1: self.y3*self.z1-self.y1*self.z3, z1: self.y1*self.z2-self.y2*self.z1,
                        x2: self.x3*self.z2-self.x2*self.z3, y2: self.x1*self.z3-self.x3*self.z1, z2: self.x2*self.z1-self.x1*self.z2,
                        x3: self.x2*self.y3-self.x3*self.y2, y3: self.x3*self.y1-self.x1*self.y3, z3: self.x1*self.y2-self.x2*self.y1 }
        }

	pub fn augment(v1 : Vec3, v2 : Vec3, v3 : Vec3) -> Mat33 {
		Mat33 { x1: v1.x, y1: v1.y, z1: v1.z,
                        x2: v2.x, y2: v2.y, z2: v2.z,
                        x3: v3.x, y3: v3.y, z3: v3.z }
	}
	
	pub fn cofactor(&self) -> Mat33 {
		Mat33 { x1: self.y2*self.z3-self.y3*self.z2, y1: self.x3*self.z2-self.x2*self.z3, z1: self.x2*self.y3-self.x3*self.y2,
			x2: self.y3*self.z1-self.y1*self.z3, y2: self.x1*self.z3-self.x3*self.z1, z2: self.x3*self.y1-self.x1*self.y3,
			x3: self.y1*self.z2-self.y2*self.z1, y3: self.x2*self.z1-self.x1*self.z2, z3: self.x1*self.y2-self.x2*self.y1 }
	}

	pub fn det(&self) -> f32 {
                 self.x1*(self.y2*self.z3-self.y3*self.z2)
		-self.x2*(self.y1*self.z3-self.y3*self.z1)
		+self.x3*(self.y1*self.z2-self.y2*self.z1)
	}

        pub fn inverse(&self) -> Mat33 {
		self.adj()/self.det()
	}

	pub fn t(&self) -> Mat33 {
		Mat33 { x1: self.x1, y1: self.x2, z1: self.x3,
                        x2: self.y1, y2: self.y2, z2: self.y3,
                        x3: self.x1, y3: self.z2, z3: self.z3 }
	}
}
