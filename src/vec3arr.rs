use crate::vec3::Vec3;
use std::mem::MaybeUninit;
use std::ptr::slice_from_raw_parts_mut;
use std::alloc::Layout;
use std::alloc::alloc;
use ::deref_impl;
use ::deref_mut_impl;
use ::value_impl;
use ::inplace_impl;
use ::dot_impl;
use ::pv_value_impl;
use ::pv_inplace_impl;
use ::pv_dot_impl;

#[derive(Copy, Clone, Debug)]
pub struct Vec3arr<const N: usize>(pub [Vec3; N]);

#[derive(Clone, Debug)]
pub struct Vec3box(pub Box<[Vec3]>);

#[derive(Debug)]
pub struct Vec3win<'a>(pub &'a mut [Vec3]);

#[derive(Debug)]
pub struct Vec3raw(pub *mut [Vec3]);

macro_rules! disp_impl {
        (Disp $t:ty$(; const $gen:ident: $gent:ty)?$(; <$lt:lifetime>)?) => {
                impl$(<$lt>)?$(<const $gen: $gent>)? fmt::Display for $t {
			#[inline]
                        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                                write!(f, "[")?;
                                for i in 0..self.len()-1 {
                                        write!(f, "<{}, {}, {}>, ", self[i].x, self[i].y, self[i].z)?
                                }
                                write!(f, "<{}, {}, {}>]", self[self.len()-1].x, self[self.len()-1].y, self[self.len()-1].z)
                        }
                }
        };
}

//Deref
use std::ops::Deref;

deref_impl! {Deref val Vec3arr<N>; to [Vec3; N]; const N: usize}
deref_impl! {Deref val Vec3box; to [Vec3]}
deref_impl! {Deref ptr Vec3win<'_>; to [Vec3]}
impl Deref for Vec3raw {
	type Target = [Vec3];
	fn deref(&self) -> &Self::Target {
		unsafe { &*self.0 }
	}
}

//DerefMut
use std::ops::DerefMut;

deref_mut_impl! {DerefMut val Vec3arr<N>; to [Vec3; N]; const N: usize}
deref_mut_impl! {DerefMut val Vec3box; to [Vec3]}
deref_mut_impl! {DerefMut ptr Vec3win<'_>; to [Vec3]}
impl DerefMut for Vec3raw {
	fn deref_mut(&mut self) -> &mut [Vec3] {
		unsafe { &mut *self.0 }
	}
}

//Add
use std::ops::Add;

pv_value_impl! {Add;add;+; 1 f32; for Vec3arr<N>; out: Vec3arr<N>; const N: usize}
pv_value_impl! {Add;add;+; 1 f32; for Vec3box; out: Vec3box}
pv_value_impl! {Add;add;+; 2 Vec3arr<N>; for f32; out: Vec3arr<N>; const N: usize}
pv_value_impl! {Add;add;+; 2 Vec3box; for f32; out: Vec3box}
pv_value_impl! {Add;add;+; 3 Vec3arr<N>; for Vec3arr<N>; out: Vec3arr<N>; const N: usize}
pv_value_impl! {Add;add;+; 3 Vec3box; for Vec3arr<N>; out: Vec3arr<N>; const N: usize}
pv_value_impl! {Add;add;+; 3 Vec3win<'a>; for Vec3arr<N>; out: Vec3arr<N>; const N: usize; <'a>}
pv_value_impl! {Add;add;+; 3 Vec3raw; for Vec3arr<N>; out: Vec3arr<N>; const N: usize}
pv_value_impl! {Add;add;+; 3 Vec3arr<N>; for Vec3box; out: Vec3arr<N>; const N: usize}
pv_value_impl! {Add;add;+; 3 Vec3box; for Vec3box; out: Vec3box}
pv_value_impl! {Add;add;+; 3 Vec3win<'a>; for Vec3box; out: Vec3box; <'a>}
pv_value_impl! {Add;add;+; 3 Vec3raw; for Vec3box; out: Vec3box}
pv_value_impl! {Add;add;+; 3 Vec3arr<N>; for Vec3win<'a>; out: Vec3arr<N>; const N: usize; <'a>}
pv_value_impl! {Add;add;+; 3 Vec3box; for Vec3win<'a>; out: Vec3box; <'a>}
pv_value_impl! {Add;add;+; 3 Vec3arr<N>; for Vec3raw; out: Vec3arr<N>; const N: usize}
pv_value_impl! {Add;add;+; 3 Vec3box; for Vec3raw; out: Vec3box}

//AddAssign
use std::ops::AddAssign;

pv_inplace_impl! {AddAssign;add_assign;+=; 0 f32; for Vec3arr<N>; const N: usize}
pv_inplace_impl! {AddAssign;add_assign;+=; 0 f32; for Vec3box}
pv_inplace_impl! {AddAssign;add_assign;+=; 0 f32; for Vec3win<'a>; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 0 f32; for Vec3raw}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec3arr<N>; for Vec3arr<N>; const N: usize}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec3box; for Vec3arr<N>; const N: usize}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec3win<'a>; for Vec3arr<N>; const N: usize; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec3raw; for Vec3arr<N>; const N: usize}
inplace_impl! {AddAssign;add_assign;+=; 1 &'a mut[Vec3]; for Vec3arr<N>; const N: usize; <'a>}
inplace_impl! {AddAssign;add_assign;+=; 1 &'a [Vec3]; for Vec3arr<N>; const N: usize; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec3arr<N>; for Vec3box; const N: usize}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec3box; for Vec3box}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec3win<'a>; for Vec3box; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec3raw; for Vec3box}
inplace_impl! {AddAssign;add_assign;+=; 1 &'a mut[Vec3]; for Vec3box; <'a>}
inplace_impl! {AddAssign;add_assign;+=; 1 &'a [Vec3]; for Vec3box; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec3arr<N>; for Vec3win<'a>; const N: usize; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec3box; for Vec3win<'a>; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec3win<'a>; for Vec3win<'a>; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec3raw; for Vec3win<'a>; <'a>}
inplace_impl! {AddAssign;add_assign;+=; 1 &'a mut[Vec3]; for Vec3win<'a>; <'a>}
inplace_impl! {AddAssign;add_assign;+=; 1 &'a [Vec3]; for Vec3win<'a>; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec3arr<N>; for Vec3raw; const N: usize}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec3box; for Vec3raw}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec3win<'a>; for Vec3raw; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec3raw; for Vec3raw}
inplace_impl! {AddAssign;add_assign;+=; 1 &'a mut[Vec3]; for Vec3raw; <'a>}
inplace_impl! {AddAssign;add_assign;+=; 1 &'a [Vec3]; for Vec3raw; <'a>}

//Display
use std::fmt;

disp_impl! {Disp Vec3arr<N>; const N: usize}
disp_impl! {Disp Vec3box}
disp_impl! {Disp Vec3win<'a>; <'a>}
disp_impl! {Disp Vec3raw}

//Div
use std::ops::Div;

pv_value_impl! {Div;div;/; 1 f32; for Vec3arr<N>; out: Vec3arr<N>; const N: usize}
pv_value_impl! {Div;div;/; 1 f32; for Vec3box; out: Vec3box}

//DivAssign
use std::ops::DivAssign;

pv_inplace_impl! {DivAssign;div_assign;/=; 0 f32; for Vec3arr<N>; const N: usize}
pv_inplace_impl! {DivAssign;div_assign;/=; 0 f32; for Vec3box}
pv_inplace_impl! {DivAssign;div_assign;/=; 0 f32; for Vec3win<'a>; <'a>}
pv_inplace_impl! {DivAssign;div_assign;/=; 0 f32; for Vec3raw}

//Mult
use std::ops::Mul;

pv_value_impl! {Mul;mul;*; 1 f32; for Vec3arr<N>; out: Vec3arr<N>; const N: usize}
pv_value_impl! {Mul;mul;*; 1 f32; for Vec3box; out: Vec3box}
pv_value_impl! {Mul;mul;*; 2 Vec3arr<N>; for f32; out: Vec3arr<N>; const N: usize}
pv_value_impl! {Mul;mul;*; 2 Vec3box; for f32; out: Vec3box}
pv_dot_impl! {Dot Vec3arr<N>; for Vec3arr<N>; const N: usize}
pv_dot_impl! {Dot Vec3box; for Vec3arr<N>; const N: usize}
pv_dot_impl! {Dot Vec3win<'a>; for Vec3arr<N>; const N: usize; <'a>}
pv_dot_impl! {Dot Vec3raw; for Vec3arr<N>; const N: usize}
pv_dot_impl! {Dot Vec3arr<N>; for Vec3box; const N: usize}
pv_dot_impl! {Dot Vec3box; for Vec3box}
pv_dot_impl! {Dot Vec3win<'a>; for Vec3box; <'a>}
pv_dot_impl! {Dot Vec3raw; for Vec3box}
pv_dot_impl! {Dot Vec3arr<N>; for Vec3win<'a>; const N: usize; <'a>}
pv_dot_impl! {Dot Vec3box; for Vec3win<'a>; <'a>}
pv_dot_impl! {Dot Vec3arr<N>; for Vec3raw; const N: usize}
pv_dot_impl! {Dot Vec3box; for Vec3raw}

//MultAssign
use std::ops::MulAssign;

pv_inplace_impl! {MulAssign;mul_assign;*=; 0 f32; for Vec3arr<N>; const N: usize}
pv_inplace_impl! {MulAssign;mul_assign;*=; 0 f32; for Vec3box}
pv_inplace_impl! {MulAssign;mul_assign;*=; 0 f32; for Vec3win<'a>; <'a>}
pv_inplace_impl! {MulAssign;mul_assign;*=; 0 f32; for Vec3raw}

//Neg
use std::ops::Neg;

impl<const N: usize> Neg for Vec3arr<N> {
        type Output = Vec3arr<N>;
        fn neg(self) -> Vec3arr<N> {
		let mut tmp: Vec3arr<N> = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..N {
                        tmp[i] = Vec3 { x: -self[i].x, y: -self[i].y, z: -self[i].z };
                }
		unsafe { std::mem::transmute::<_, Vec3arr<N>>(tmp) }
        }
}

impl Neg for Vec3box {
        type Output = Vec3box;
        fn neg(self) -> Vec3box {
                let mut tmp = Box::<[Vec3]>::new_uninit_slice(self.len());
                let tmp = unsafe {
                        for i in 0..self.len() {
                                tmp[i].as_mut_ptr().write( Vec3 { x: -self[i].x, y: -self[i].y, z: -self[i].z } );
                        }
                        tmp.assume_init()
                };
                Vec3box(tmp)
        }
}

//Sub
use std::ops::Sub;

pv_value_impl! {Sub;sub;-; 1 f32; for Vec3arr<N>; out: Vec3arr<N>; const N: usize}
pv_value_impl! {Sub;sub;-; 1 f32; for Vec3box; out: Vec3box}
pv_value_impl! {Sub;sub;-; 3 Vec3arr<N>; for Vec3arr<N>; out: Vec3arr<N>; const N: usize}
pv_value_impl! {Sub;sub;-; 3 Vec3box; for Vec3arr<N>; out: Vec3arr<N>; const N: usize}
pv_value_impl! {Sub;sub;-; 3 Vec3win<'a>; for Vec3arr<N>; out: Vec3arr<N>; const N: usize; <'a>}
pv_value_impl! {Sub;sub;-; 3 Vec3raw; for Vec3arr<N>; out: Vec3arr<N>; const N: usize}
pv_value_impl! {Sub;sub;-; 3 Vec3arr<N>; for Vec3box; out: Vec3arr<N>; const N: usize}
pv_value_impl! {Sub;sub;-; 3 Vec3box; for Vec3box; out: Vec3box}
pv_value_impl! {Sub;sub;-; 3 Vec3win<'a>; for Vec3box; out: Vec3box; <'a>}
pv_value_impl! {Sub;sub;-; 3 Vec3raw; for Vec3box; out: Vec3box}
pv_value_impl! {Sub;sub;-; 3 Vec3arr<N>; for Vec3win<'a>; out: Vec3arr<N>; const N: usize; <'a>}
pv_value_impl! {Sub;sub;-; 3 Vec3box; for Vec3win<'a>; out: Vec3box; <'a>}
pv_value_impl! {Sub;sub;-; 3 Vec3arr<N>; for Vec3raw; out: Vec3arr<N>; const N: usize}
pv_value_impl! {Sub;sub;-; 3 Vec3box; for Vec3raw; out: Vec3box}

//SubAssign
use std::ops::SubAssign;

pv_inplace_impl! {SubAssign;sub_assign;-=; 0 f32; for Vec3arr<N>; const N: usize}
pv_inplace_impl! {SubAssign;sub_assign;-=; 0 f32; for Vec3box}
pv_inplace_impl! {SubAssign;sub_assign;-=; 0 f32; for Vec3win<'a>; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 0 f32; for Vec3raw}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec3arr<N>; for Vec3arr<N>; const N: usize}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec3box; for Vec3arr<N>; const N: usize}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec3win<'a>; for Vec3arr<N>; const N: usize; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec3raw; for Vec3arr<N>; const N: usize}
inplace_impl! {SubAssign;sub_assign;-=; 1 &'a mut[Vec3]; for Vec3arr<N>; const N: usize; <'a>}
inplace_impl! {SubAssign;sub_assign;-=; 1 &'a [Vec3]; for Vec3arr<N>; const N: usize; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec3arr<N>; for Vec3box; const N: usize}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec3box; for Vec3box}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec3win<'a>; for Vec3box; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec3raw; for Vec3box}
inplace_impl! {SubAssign;sub_assign;-=; 1 &'a mut[Vec3]; for Vec3box; <'a>}
inplace_impl! {SubAssign;sub_assign;-=; 1 &'a [Vec3]; for Vec3box; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec3arr<N>; for Vec3win<'a>; const N: usize; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec3box; for Vec3win<'a>; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec3win<'a>; for Vec3win<'a>; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec3raw; for Vec3win<'a>; <'a>}
inplace_impl! {SubAssign;sub_assign;-=; 1 &'a mut[Vec3]; for Vec3win<'a>; <'a>}
inplace_impl! {SubAssign;sub_assign;-=; 1 &'a [Vec3]; for Vec3win<'a>; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec3arr<N>; for Vec3raw; const N: usize}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec3box; for Vec3raw}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec3win<'a>; for Vec3raw; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec3raw; for Vec3raw}
inplace_impl! {SubAssign;sub_assign;-=; 1 &'a mut[Vec3]; for Vec3raw; <'a>}
inplace_impl! {SubAssign;sub_assign;-=; 1 &'a [Vec3]; for Vec3raw; <'a>}

//Vec3arr Methods

impl<const N: usize> Vec3arr<N> {

	pub fn crossarr(&self, rhs : Vec3arr<N>) -> Vec3arr<N> {
		let mut tmp: Vec3arr<N> = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..N {
                        tmp[i] = Vec3 {
                		x: self[i].y * rhs[i].z - self[i].z * rhs[i].y,
                                y: self[i].z * rhs[i].x - self[i].x * rhs[i].z,
                                z: self[i].x * rhs[i].y - self[i].y * rhs[i].x
                        };
                }
                unsafe { std::mem::transmute::<_, Vec3arr<N>>(tmp) }
	}

	pub fn cross(&self, rhs : Vec3) -> Vec3arr<N> {
		let mut tmp: Vec3arr<N> = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..N {
                        tmp[i] = Vec3 {
                                x: self[i].y * rhs.z - self[i].z * rhs.y,
                                y: self[i].z * rhs.x - self[i].x * rhs.z,
                                z: self[i].x * rhs.y - self[i].y * rhs.x
                        };
                }
                unsafe { std::mem::transmute::<_, Vec3arr<N>>(tmp) }
        }

        pub fn mag(&self) -> [f32; N] {
		let mut tmp: [f32; N] = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..N {
                        tmp[i] = (self[i].x*self[i].x + self[i].y*self[i].y + self[i].z*self[i].z).sqrt();
                }
                unsafe { std::mem::transmute::<_, [f32; N]>(tmp) }
        }

        pub fn mag2(&self) -> [f32; N] {
                let mut tmp: [f32; N] = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..N {
                        tmp[i] = self[i].x*self[i].x + self[i].y*self[i].y + self[i].z*self[i].z;
                }
                unsafe { std::mem::transmute::<_, [f32; N]>(tmp) }
        }

        pub fn norm(&self) -> Vec3arr<N> {
		let mut tmp: Vec3arr<N> = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..N {
			let mag : f32 = (self[i].x*self[i].x + self[i].y*self[i].y + self[i].z*self[i].z).sqrt();
                        tmp[i] = Vec3 { x: self[i].x/mag, y: self[i].y/mag, z: self[i].z/mag };
                }
                unsafe { std::mem::transmute::<_, Vec3arr<N>>(tmp) }
        }

	pub fn transform<F : Fn(Vec3) -> Vec3>(&mut self, f: F) {
		for i in 0..N {
                        self[i] = f(self[i]);
                }
	}

	pub fn ftoarr<F : Fn(usize) -> Vec3>(f: F) -> Vec3arr<N> {
		let mut tmp: Vec3arr<N> = unsafe { MaybeUninit::uninit().assume_init() };
		for i in 0..N {
                        tmp[i] = f(i);
                }
		unsafe { std::mem::transmute::<_, Vec3arr<N>>(tmp) }
	}
}

// Vec3box Methods

impl Vec3box {

        pub fn new_zeroed(len: usize) -> Vec3box {
                unsafe { Vec3box(Box::<[Vec3]>::new_zeroed_slice(len).assume_init()) }
        }

	pub fn from_raw_parts(ptr: *mut Vec3, len: usize) -> Vec3box {
                unsafe { Vec3box(Box::from_raw(slice_from_raw_parts_mut(ptr, len))) }
        }

        pub fn new_from_arr<const N: usize>(arr: Vec3arr<N>) -> Vec3box {
                let mut new = Vec3box::new_uninit_box(N);
                let new = unsafe {
			for i in 0..N {
                        	new[i].as_mut_ptr().write(arr[i]);
                	}
			new.assume_init()
		};
                Vec3box(new)
        }

        pub fn new_from_box(arr: Vec3box) -> Vec3box {
                let mut new = Vec3box::new_uninit_box(arr.len());
                let new = unsafe {
                        for i in 0..arr.len() {
                                new[i].as_mut_ptr().write(arr[i]);
                        }
                        new.assume_init()
                };
                Vec3box(new)
        }

        pub fn new_from_win<'a>(arr : Vec3win<'a>) -> Vec3box {
                let mut new = Vec3box::new_uninit_box(arr.len());
                let new = unsafe {
                        for i in 0..arr.len() {
                                new[i].as_mut_ptr().write(arr[i]);
                        }
                        new.assume_init()
                };
                Vec3box(new)
        }

	pub fn new_from_raw<'a>(arr : Vec3raw) -> Vec3box {
                let mut new = Vec3box::new_uninit_box(arr.len());
                let new = unsafe {
                        for i in 0..arr.len() {
                                new[i].as_mut_ptr().write(arr[i]);
                        }
                        new.assume_init()
                };
                Vec3box(new)
        }

	pub fn crossarr(&self, rhs : Vec3box) -> Vec3box {
		let mut tmp = Box::<[Vec3]>::new_uninit_slice(self.len());
                let tmp = unsafe {
                        for i in 0..self.len() {
                        	tmp[i].as_mut_ptr().write( Vec3 {
                                	x: self[i].y * rhs[i].z - self[i].z * rhs[i].y,
                                	y: self[i].z * rhs[i].x - self[i].x * rhs[i].z,
                                	z: self[i].x * rhs[i].y - self[i].y * rhs[i].x
                        	} );
			}
			tmp.assume_init()
                };
                Vec3box(tmp)
        }

        pub fn cross(&self, rhs : Vec3) -> Vec3box {
		let mut tmp = Box::<[Vec3]>::new_uninit_slice(self.len());
                let tmp = unsafe {
                        for i in 0..self.len() {
                        	tmp[i].as_mut_ptr().write( Vec3 {
                                	x: self[i].y * rhs.z - self[i].z * rhs.y,
                                	y: self[i].z * rhs.x - self[i].x * rhs.z,
                                	z: self[i].x * rhs.y - self[i].y * rhs.x
                        	} );
			}
			tmp.assume_init()
                };
                Vec3box(tmp)
        }

	pub fn mag(&self) -> Box<[f32]> {
		let mut tmp = Box::<[f32]>::new_uninit_slice(self.len());
                let tmp = unsafe {
                        for i in 0..self.len() {
                        	tmp[i].as_mut_ptr().write((self[i].x*self[i].x + self[i].y*self[i].y + self[i].z*self[i].z).sqrt());
			}
			tmp.assume_init()
                };
                tmp
        }

        pub fn mag2(&self) -> Box<[f32]> {
		let mut tmp = Box::<[f32]>::new_uninit_slice(self.len());
                let tmp = unsafe {
                        for i in 0..self.len() {
                        	tmp[i].as_mut_ptr().write(self[i].x*self[i].x + self[i].y*self[i].y + self[i].z*self[i].z);
			}
			tmp.assume_init()
                };
                tmp
        }

        pub fn norm(&self) -> Vec3box {
		let mut tmp = Box::<[Vec3]>::new_uninit_slice(self.len());
                let tmp = unsafe {
                        for i in 0..self.len() {
                        	let mag : f32 = (self[i].x*self[i].x + self[i].y*self[i].y + self[i].z*self[i].z).sqrt();
                        	tmp[i].as_mut_ptr().write( Vec3 { x: self[i].x/mag, y: self[i].y/mag, z: self[i].z/mag } );
			}
			tmp.assume_init()
                };
                Vec3box(tmp)
        }

	pub fn transform<F: Fn(Vec3) -> Vec3>(&mut self, f: F) {
                for i in 0..self.len() {
                        self[i] = f(self[i]);
                }
        }

	pub fn ftoarr<F: Fn(usize) -> Vec3>(f: F, n: usize) -> Vec3box {
		let mut tmp = Box::<[Vec3]>::new_uninit_slice(n);
                let tmp = unsafe {
                        for i in 0..n {
                        	tmp[i].as_mut_ptr().write(f(i));
			}
			tmp.assume_init()
                };
                Vec3box(tmp)
        }

	pub fn new_uninit_box(len: usize) -> Box<[MaybeUninit<Vec3>]> {
		Box::<[Vec3]>::new_uninit_slice(len)
	}
}

// Vec3win Methods

impl<'a> Vec3win<'a> {

	pub fn transform<F: Fn(Vec3) -> Vec3>(&mut self, f: F) {
                for i in 0..self.len() {
                        self[i] = f(self[i]);
                }
        }

}

// Vec3raw Methods

impl Vec3raw {

	fn layout(len: usize) -> Layout {
		Layout::from_size_align(Layout::new::<Vec3>().size()*len,Layout::new::<Vec3>().align()).unwrap()
	}

	pub unsafe fn new_uninit_ptr(len: usize) -> *mut [MaybeUninit<Vec3>] {
		let ptr = alloc(Vec3raw::layout(len)) as *mut MaybeUninit<Vec3>;
		slice_from_raw_parts_mut(ptr, len)
	}

	pub fn from_raw_parts(ptr: *mut Vec3, len: usize) -> Vec3raw {
		Vec3raw(slice_from_raw_parts_mut(ptr, len))
	}

	pub fn new_zeroed(len: usize) -> Vec3raw {
		let ptr = unsafe { Vec3raw::new_uninit_ptr(len) };
		for i in 0..len {
			unsafe { (*ptr.as_mut_ptr().add(i)).write(Vec3::ZERO) };
		}
		Vec3raw(ptr as *mut [Vec3])		
	}

	pub fn new_from_arr<const N: usize>(arr: Vec3arr<N>) -> Vec3raw {
		let ptr = unsafe { Vec3raw::new_uninit_ptr(N) };
                for i in 0..N {
			unsafe { (*ptr.as_mut_ptr().add(i)).write(arr[i]) };
                }
                Vec3raw(ptr as *mut [Vec3])
	}

	pub fn new_from_box(arr: Vec3box) -> Vec3raw {
		let ptr = unsafe { Vec3raw::new_uninit_ptr(arr.len()) };
                for i in 0..arr.len() {
                        unsafe { (*ptr.as_mut_ptr().add(i)).write(arr[i]) };
                }
                Vec3raw(ptr as *mut [Vec3])
	}

	pub fn new_from_win<'a>(arr : Vec3win<'a>) -> Vec3raw {
		let ptr = unsafe { Vec3raw::new_uninit_ptr(arr.len()) };
                for i in 0..arr.len() {
                        unsafe { (*ptr.as_mut_ptr().add(i)).write(arr[i]) };
                }
                Vec3raw(ptr as *mut [Vec3])
	}

	pub fn new_from_raw<'a>(arr : Vec3raw) -> Vec3raw {
                let ptr = unsafe { Vec3raw::new_uninit_ptr(arr.len()) };
                for i in 0..arr.len() {
                        unsafe { (*ptr.as_mut_ptr().add(i)).write(arr[i]) };
                }
                Vec3raw(ptr as *mut [Vec3])
        }

	pub fn from_arr<const N: usize>(&mut self, arr: Vec3arr<N>) {
                if self.len() != arr.len() { panic!("slice and array inequal length"); }
		for i in 0..self.len() {
			self[i] = arr[i];
		}
        }

        pub fn from_box(&mut self, arr: Vec3box) {
                if self.len() != arr.len() { panic!("slices inequal length"); }
		for i in 0..self.len() {
                        self[i] = arr[i];
                }
        }

        pub fn from_win<'a>(&mut self, arr : Vec3win<'a>) {
                if self.len() != arr.len() { panic!("slices inequal length"); }
		for i in 0..self.len() {
                        self[i] = arr[i];
                }
        }

	pub fn to_arr<const N: usize>(&self) -> Vec3arr<N> {
		if self.len() != N { panic!("slice and array inequal length"); }
		let mut tmp: Vec3arr<N> = unsafe { MaybeUninit::uninit().assume_init() };
		for i in 0..N {
                        tmp[i] = self[i];
                }
                unsafe { std::mem::transmute::<_, Vec3arr<N>>(tmp) }
        }

        pub fn to_box(&self) -> Vec3box {
		let mut tmp = Box::<[Vec3]>::new_uninit_slice(self.len());
		let tmp = unsafe {
                        for i in 0..self.len() {
                                tmp[i].as_mut_ptr().write(self[i]);
                        }
                        tmp.assume_init()
                };
		Vec3box(tmp)
        }

	pub fn transform<F: Fn(Vec3) -> Vec3>(&mut self, f: F) {
                for i in 0..self.len() {
                        self[i] = f(self[i]);
                }
        }
}
