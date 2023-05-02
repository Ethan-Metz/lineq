#[derive(Copy, Clone, PartialEq)]
pub struct Vec3 {
        pub x : f32,
        pub y : f32,
        pub z : f32,
}

macro_rules! value_impl {
        ($imp:ident;$func:ident;$op:tt; 3 $rhs:ty; for $lhs:ty) => {
                impl $imp<$rhs> for $lhs {
                        type Output = Vec3;
                        #[inline]
                        fn $func(self, rhs: $rhs) -> Vec3 {
				Vec3 { x: self.x $op rhs.x, y: self.y $op rhs.y, z: self.z $op rhs.z }
                        }
                }
        };
        ($imp:ident;$func:ident;$op:tt; 2 $rhs:ty; for $lhs:ty) => {
                impl $imp<$rhs> for $lhs {
                        type Output = Vec3;
                        #[inline]
                        fn $func(self, rhs: $rhs) -> Vec3 {
                                Vec3 { x: self $op rhs.x, y: self $op rhs.y, z: self $op rhs.z }
                        }
                }
        };
        ($imp:ident;$func:ident;$op:tt; 1 $rhs:ty; for $lhs:ty) => {
                impl $imp<$rhs> for $lhs {
                        type Output = Vec3;
                        #[inline]
                        fn $func(self, rhs: $rhs) -> Vec3 {
                                Vec3 { x: self.x $op rhs, y: self.y $op rhs, z: self.z $op rhs }
                        }
                }
        };
}

macro_rules! inplace_impl {
        ($imp:ident;$func:ident;$op:tt; 0 $rhs:ty; for $lhs:ty) => {
                impl $imp<$rhs> for $lhs {
                        #[inline]
                        fn $func(&mut self, rhs: $rhs) {
                                self.x $op rhs;
                                self.y $op rhs;
				self.z $op rhs;
                        }
                }
        };
        ($imp:ident;$func:ident;$op:tt; 1 $rhs:ty; for $lhs:ty) => {
                impl $imp<$rhs> for $lhs {
                        #[inline]
                        fn $func(&mut self, rhs: $rhs) {
                                self.x $op rhs.x;
                                self.y $op rhs.y;
                        	self.z $op rhs.z;
                        }
                }
        };
}

macro_rules! dot_impl {
        (Dot $rhs:ty; for $lhs:ty) => {
                impl Mul<$rhs> for $lhs {
                        type Output = f32;
                        #[inline]
                        fn mul(self, rhs: $rhs) -> f32 {
				self.x*rhs.x+self.y*rhs.y+self.z*rhs.z
                        }
                }
        };
}

//Add
use std::ops::Add;

value_impl! {Add;add;+; 1 f32; for Vec3}
value_impl! {Add;add;+; 2 Vec3; for f32}
value_impl! {Add;add;+; 3 Vec3; for Vec3}
value_impl! {Add;add;+; 1 &f32; for Vec3}
value_impl! {Add;add;+; 2 &Vec3; for f32}
value_impl! {Add;add;+; 3 &Vec3; for Vec3}
value_impl! {Add;add;+; 1 f32; for &Vec3}
value_impl! {Add;add;+; 2 Vec3; for &f32}
value_impl! {Add;add;+; 3 Vec3; for &Vec3}
value_impl! {Add;add;+; 1 &f32; for &Vec3}
value_impl! {Add;add;+; 2 &Vec3; for &f32}
value_impl! {Add;add;+; 3 &Vec3; for &Vec3}

//AddAssign
use std::ops::AddAssign;

inplace_impl! {AddAssign;add_assign;+=; 0 f32; for Vec3}
inplace_impl! {AddAssign;add_assign;+=; 1 Vec3; for Vec3}
inplace_impl! {AddAssign;add_assign;+=; 0 &f32; for Vec3}
inplace_impl! {AddAssign;add_assign;+=; 1 &Vec3; for Vec3}

//Display
use std::fmt;

impl fmt::Display for Vec3 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
        }
}

impl fmt::Debug for Vec3 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct("Vec3")
                .field("x", &self.x)
                .field("y", &self.y)
                .field("z", &self.z)
                .finish()
        }
}

//Div
use std::ops::Div;

value_impl! {Div;div;/; 1 f32; for Vec3}
value_impl! {Div;div;/; 1 &f32; for Vec3}
value_impl! {Div;div;/; 1 f32; for &Vec3}
value_impl! {Div;div;/; 1 &f32; for &Vec3}

//DivAssign
use std::ops::DivAssign;

inplace_impl! {DivAssign;div_assign;/=; 0 f32; for Vec3}
inplace_impl! {DivAssign;div_assign;/=; 0 &f32; for Vec3}

//Mult
use std::ops::Mul;

value_impl! {Mul;mul;*; 1 f32; for Vec3}
value_impl! {Mul;mul;*; 2 Vec3; for f32}
dot_impl! {Dot Vec3; for Vec3}
value_impl! {Mul;mul;*; 1 &f32; for Vec3}
value_impl! {Mul;mul;*; 2 &Vec3; for f32}
dot_impl! {Dot &Vec3; for Vec3}
value_impl! {Mul;mul;*; 1 f32; for &Vec3}
value_impl! {Mul;mul;*; 2 Vec3; for &f32}
dot_impl! {Dot Vec3; for &Vec3}
value_impl! {Mul;mul;*; 1 &f32; for &Vec3}
value_impl! {Mul;mul;*; 2 &Vec3; for &f32}
dot_impl! {Dot &Vec3; for &Vec3}

//MultAssign
use std::ops::MulAssign;

inplace_impl! {MulAssign;mul_assign;*=; 0 f32; for Vec3}
inplace_impl! {MulAssign;mul_assign;*=; 0 &f32; for Vec3}

//Neg
use std::ops::Neg;

impl Neg for Vec3 {
        type Output = Vec3;
        fn neg(self) -> Vec3 {
                Vec3 { x: -self.x, y: -self.y, z: -self.z }
        }
}

impl Neg for &Vec3 {
        type Output = Vec3;
        fn neg(self) -> Vec3 {
                Vec3 { x: -self.x, y: -self.y, z: -self.z }
        }
}

//Sub
use std::ops::Sub;

value_impl! {Sub;sub;-; 1 f32; for Vec3}
value_impl! {Sub;sub;-; 3 Vec3; for Vec3}
value_impl! {Sub;sub;-; 1 &f32; for Vec3}
value_impl! {Sub;sub;-; 3 &Vec3; for Vec3}
value_impl! {Sub;sub;-; 1 f32; for &Vec3}
value_impl! {Sub;sub;-; 3 Vec3; for &Vec3}
value_impl! {Sub;sub;-; 1 &f32; for &Vec3}
value_impl! {Sub;sub;-; 3 &Vec3; for &Vec3}

//SubAssign
use std::ops::SubAssign;

inplace_impl! {SubAssign;sub_assign;-=; 0 f32; for Vec3}
inplace_impl! {SubAssign;sub_assign;-=; 1 Vec3; for Vec3}
inplace_impl! {SubAssign;sub_assign;-=; 0 &f32; for Vec3}
inplace_impl! {SubAssign;sub_assign;-=; 1 &Vec3; for Vec3}

//Vec3 Methods

impl Vec3 {
	pub const ZERO : Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
        pub const ONE : Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
        pub const LEFT : Vec3 = Vec3 { x: -1.0, y: 0.0, z: 0.0 };
        pub const RIGHT : Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
        pub const UP : Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
        pub const DOWN : Vec3 = Vec3 { x: 0.0, y: -1.0, z: 0.0 };
	pub const IN : Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };
	pub const OUT : Vec3 = Vec3 { x: 0.0, y: 0.0, z:-1.0 };

	pub fn cross(&self, rhs : Vec3) -> Vec3 {
                Vec3 { 
                        x: self.y * rhs.z - self.z * rhs.y,
                        y: self.z * rhs.x - self.x * rhs.z,
                        z: self.x * rhs.y - self.y * rhs.x
		}
	}

        pub fn mag(&self) -> f32 {
                (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
        }

        pub fn mag2(&self) -> f32 {
                self.x*self.x + self.y*self.y + self.z*self.z
        }

        pub fn norm(&self) -> Vec3 {
                let mag : f32 = self.mag();
                Vec3 { x: self.x/mag, y: self.y/mag, z: self.z/mag }
        }
}
