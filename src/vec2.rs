#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
        pub x : f32,
        pub y : f32,
}

macro_rules! value_impl {
        ($imp:ident;$func:ident;$op:tt; 3 $rhs:ty; for $lhs:ty) => {
                impl $imp<$rhs> for $lhs {
                        type Output = Vec2;
                        #[inline]
                        fn $func(self, rhs: $rhs) -> Vec2 {
				Vec2 { x: self.x $op rhs.x, y: self.y $op rhs.y }
                        }
                }
        };
        ($imp:ident;$func:ident;$op:tt; 2 $rhs:ty; for $lhs:ty) => {
                impl $imp<$rhs> for $lhs {
                        type Output = Vec2;
                        #[inline]
                        fn $func(self, rhs: $rhs) -> Vec2 {
                                Vec2 { x: self $op rhs.x, y: self $op rhs.y }
                        }
                }
        };
        ($imp:ident;$func:ident;$op:tt; 1 $rhs:ty; for $lhs:ty) => {
                impl $imp<$rhs> for $lhs {
                        type Output = Vec2;
                        #[inline]
                        fn $func(self, rhs: $rhs) -> Vec2 {
                                Vec2 { x: self.x $op rhs, y: self.y $op rhs }
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
                        }
                }
        };
        ($imp:ident;$func:ident;$op:tt; 1 $rhs:ty; for $lhs:ty) => {
                impl $imp<$rhs> for $lhs {
                        #[inline]
                        fn $func(&mut self, rhs: $rhs) {
                                self.x $op rhs.x;
                                self.y $op rhs.y;
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
				self.x*rhs.x+self.y*rhs.y
                        }
                }
        };
}

//Add
use std::ops::Add;

value_impl! {Add;add;+; 1 f32; for Vec2}
value_impl! {Add;add;+; 2 Vec2; for f32}
value_impl! {Add;add;+; 3 Vec2; for Vec2}
value_impl! {Add;add;+; 1 &f32; for Vec2}
value_impl! {Add;add;+; 2 &Vec2; for f32}
value_impl! {Add;add;+; 3 &Vec2; for Vec2}
value_impl! {Add;add;+; 1 f32; for &Vec2}
value_impl! {Add;add;+; 2 Vec2; for &f32}
value_impl! {Add;add;+; 3 Vec2; for &Vec2}
value_impl! {Add;add;+; 1 &f32; for &Vec2}
value_impl! {Add;add;+; 2 &Vec2; for &f32}
value_impl! {Add;add;+; 3 &Vec2; for &Vec2}

//AddAssign
use std::ops::AddAssign;

inplace_impl! {AddAssign;add_assign;+=; 0 f32; for Vec2}
inplace_impl! {AddAssign;add_assign;+=; 1 Vec2; for Vec2}
inplace_impl! {AddAssign;add_assign;+=; 0 &f32; for Vec2}
inplace_impl! {AddAssign;add_assign;+=; 1 &Vec2; for Vec2}

//Display
use std::fmt;

impl fmt::Display for Vec2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "<{}, {}>", self.x, self.y)
        }
}

//Div
use std::ops::Div;

value_impl! {Div;div;/; 1 f32; for Vec2}
value_impl! {Div;div;/; 1 &f32; for Vec2}
value_impl! {Div;div;/; 1 f32; for &Vec2}
value_impl! {Div;div;/; 1 &f32; for &Vec2}

//DivAssign
use std::ops::DivAssign;

inplace_impl! {DivAssign;div_assign;/=; 0 f32; for Vec2}
inplace_impl! {DivAssign;div_assign;/=; 0 &f32; for Vec2}

//Mult
use std::ops::Mul;

value_impl! {Mul;mul;*; 1 f32; for Vec2}
value_impl! {Mul;mul;*; 2 Vec2; for f32}
dot_impl! {Dot Vec2; for Vec2}
value_impl! {Mul;mul;*; 1 &f32; for Vec2}
value_impl! {Mul;mul;*; 2 &Vec2; for f32}
dot_impl! {Dot &Vec2; for Vec2}
value_impl! {Mul;mul;*; 1 f32; for &Vec2}
value_impl! {Mul;mul;*; 2 Vec2; for &f32}
dot_impl! {Dot Vec2; for &Vec2}
value_impl! {Mul;mul;*; 1 &f32; for &Vec2}
value_impl! {Mul;mul;*; 2 &Vec2; for &f32}
dot_impl! {Dot &Vec2; for &Vec2}

//MultAssign
use std::ops::MulAssign;

inplace_impl! {MulAssign;mul_assign;*=; 0 f32; for Vec2}
inplace_impl! {MulAssign;mul_assign;*=; 0 &f32; for Vec2}

//Neg
use std::ops::Neg;

impl Neg for Vec2 {
        type Output = Vec2;
        fn neg(self) -> Vec2 {
                Vec2 { x: -self.x, y: -self.y }
        }
}

impl Neg for &Vec2 {
        type Output = Vec2;
        fn neg(self) -> Vec2 {
                Vec2 { x: -self.x, y: -self.y }
        }
}

//Sub
use std::ops::Sub;

value_impl! {Sub;sub;-; 1 f32; for Vec2}
value_impl! {Sub;sub;-; 3 Vec2; for Vec2}
value_impl! {Sub;sub;-; 1 &f32; for Vec2}
value_impl! {Sub;sub;-; 3 &Vec2; for Vec2}
value_impl! {Sub;sub;-; 1 f32; for &Vec2}
value_impl! {Sub;sub;-; 3 Vec2; for &Vec2}
value_impl! {Sub;sub;-; 1 &f32; for &Vec2}
value_impl! {Sub;sub;-; 3 &Vec2; for &Vec2}

//SubAssign
use std::ops::SubAssign;

inplace_impl! {SubAssign;sub_assign;-=; 0 f32; for Vec2}
inplace_impl! {SubAssign;sub_assign;-=; 1 Vec2; for Vec2}
inplace_impl! {SubAssign;sub_assign;-=; 0 &f32; for Vec2}
inplace_impl! {SubAssign;sub_assign;-=; 1 &Vec2; for Vec2}

//Vec2 Methods

impl Vec2 {
	pub const ZERO : Vec2 = Vec2 { x: 0.0, y: 0.0 };
	pub const ONE : Vec2 = Vec2 { x: 1.0, y: 1.0 };
	pub const LEFT : Vec2 = Vec2 { x: -1.0, y: 0.0 };
	pub const RIGHT : Vec2 = Vec2 { x: 1.0, y: 0.0 };
	pub const UP : Vec2 = Vec2 { x: 0.0, y: 1.0 };
	pub const DOWN : Vec2 = Vec2 { x: 0.0, y: -1.0 };

	pub fn det(&self, rhs : Vec2) -> f32 {
                self.x*rhs.y-self.y*rhs.x
	}

        pub fn mag(&self) -> f32 {
                (self.x*self.x + self.y*self.y).sqrt()
        }

        pub fn mag2(&self) -> f32 {
                self.x*self.x + self.y*self.y
        }

        pub fn norm(&self) -> Vec2 {
                let mag : f32 = (self.x*self.x + self.y*self.y).sqrt();
                Vec2 { x: self.x/mag, y: self.y/mag }
        }
}
