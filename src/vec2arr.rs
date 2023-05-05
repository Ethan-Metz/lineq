use crate::vec2::Vec2;
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2arr<const N: usize>(pub [Vec2; N]);

#[derive(Clone, Debug, PartialEq)]
pub struct Vec2box(pub Box<[Vec2]>);

#[derive(Debug, PartialEq)]
pub struct Vec2win<'a>(pub &'a mut [Vec2]);

#[derive(Debug, PartialEq)]
pub struct Vec2raw(pub *mut [Vec2]);

macro_rules! disp_impl {
        (Disp $t:ty$(; const $gen:ident: $gent:ty)?$(; <$lt:lifetime>)?) => {
                impl$(<$lt>)?$(<const $gen: $gent>)? fmt::Display for $t {
			#[inline]
                        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                                write!(f, "[")?;
                                for i in 0..self.len()-1 {
                                        write!(f, "<{}, {}>, ", self[i].x, self[i].y)?
                                }
                                write!(f, "<{}, {}>]", self[self.len()-1].x, self[self.len()-1].y)
                        }
                }
        };
}

//Deref
use std::ops::Deref;

deref_impl! {Deref val Vec2arr<N>; to [Vec2; N]; const N: usize}
deref_impl! {Deref val Vec2box; to [Vec2]}
deref_impl! {Deref ptr Vec2win<'_>; to [Vec2]}
impl Deref for Vec2raw {
	type Target = [Vec2];
	fn deref(&self) -> &Self::Target {
		unsafe { &*self.0 }
	}
}

//DerefMut
use std::ops::DerefMut;

deref_mut_impl! {DerefMut val Vec2arr<N>; to [Vec2; N]; const N: usize}
deref_mut_impl! {DerefMut val Vec2box; to [Vec2]}
deref_mut_impl! {DerefMut ptr Vec2win<'_>; to [Vec2]}
impl DerefMut for Vec2raw {
	fn deref_mut(&mut self) -> &mut [Vec2] {
		unsafe { &mut *self.0 }
	}
}

//Add
use std::ops::Add;
use std::ops::Index;
	
impl<T, const N: usize> Add<T> for Vec2arr<N>
where 
    Vec2: Add<T>,
    <Vec2 as Add<T>>::Output: Vec2,
{
    type Output = Vec2arr<N>;
    #[inline]
    fn add(self, rhs: T) -> Vec2arr<N> {
        let mut tmp: Vec2arr<N> = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            tmp[i] = self[i] + rhs;
        }
        unsafe { std::mem::transmute::<_, Vec2arr<N>>(tmp) }
    }
}

/*impl<const N: usize> Add<&f32> for Vec2arr<N> {
    type Output = Vec2arr<N>;
    #[inline]
    fn add(self, rhs: &f32) -> Vec2arr<N> {
        let mut tmp: Vec2arr<N> = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            tmp[i] = self[i] + rhs;
        }
        unsafe { std::mem::transmute::<_, Vec2arr<N>>(tmp) }
    }
}*/

impl<const N: usize> Add<f32> for &Vec2arr<N> {
    type Output = Vec2arr<N>;
    #[inline]
    fn add(self, rhs: f32) -> Vec2arr<N> {
        let mut tmp: Vec2arr<N> = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            tmp[i] = self[i] + rhs;
        }
        unsafe { std::mem::transmute::<_, Vec2arr<N>>(tmp) }
    }
}

impl<const N: usize> Add<&f32> for &Vec2arr<N> {
    type Output = Vec2arr<N>;
    #[inline]
    fn add(self, rhs: &f32) -> Vec2arr<N> {
        let mut tmp: Vec2arr<N> = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            tmp[i] = self[i] + rhs;
        }
        unsafe { std::mem::transmute::<_, Vec2arr<N>>(tmp) }
    }
}

/*pv_value_impl! {Add;add;+; 1 f32; for Vec2box; out: Vec2box}
pv_value_impl! {Add;add;+; 2 Vec2arr<N>; for f32; out: Vec2arr<N>; const N: usize}
pv_value_impl! {Add;add;+; 2 Vec2box; for f32; out: Vec2box}
pv_value_impl! {Add;add;+; 3 Vec2arr<N>; for Vec2arr<N>; out: Vec2arr<N>; const N: usize}
pv_value_impl! {Add;add;+; 3 Vec2box; for Vec2arr<N>; out: Vec2arr<N>; const N: usize}
pv_value_impl! {Add;add;+; 3 Vec2win<'a>; for Vec2arr<N>; out: Vec2arr<N>; const N: usize; <'a>}
pv_value_impl! {Add;add;+; 3 Vec2raw; for Vec2arr<N>; out: Vec2arr<N>; const N: usize}
pv_value_impl! {Add;add;+; 3 Vec2arr<N>; for Vec2box; out: Vec2arr<N>; const N: usize}
pv_value_impl! {Add;add;+; 3 Vec2box; for Vec2box; out: Vec2box}
pv_value_impl! {Add;add;+; 3 Vec2win<'a>; for Vec2box; out: Vec2box; <'a>}
pv_value_impl! {Add;add;+; 3 Vec2raw; for Vec2box; out: Vec2box}
pv_value_impl! {Add;add;+; 3 Vec2arr<N>; for Vec2win<'a>; out: Vec2arr<N>; const N: usize; <'a>}
pv_value_impl! {Add;add;+; 3 Vec2box; for Vec2win<'a>; out: Vec2box; <'a>}
pv_value_impl! {Add;add;+; 3 Vec2arr<N>; for Vec2raw; out: Vec2arr<N>; const N: usize}
pv_value_impl! {Add;add;+; 3 Vec2box; for Vec2raw; out: Vec2box}*/

//AddAssign
use std::ops::AddAssign;

pv_inplace_impl! {AddAssign;add_assign;+=; 0 f32; for Vec2arr<N>; const N: usize}
pv_inplace_impl! {AddAssign;add_assign;+=; 0 f32; for Vec2box}
pv_inplace_impl! {AddAssign;add_assign;+=; 0 f32; for Vec2win<'a>; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 0 f32; for Vec2raw}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec2arr<N>; for Vec2arr<N>; const N: usize}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec2box; for Vec2arr<N>; const N: usize}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec2win<'a>; for Vec2arr<N>; const N: usize; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec2raw; for Vec2arr<N>; const N: usize}
inplace_impl! {AddAssign;add_assign;+=; 1 &'a mut[Vec2]; for Vec2arr<N>; const N: usize; <'a>}
inplace_impl! {AddAssign;add_assign;+=; 1 &'a [Vec2]; for Vec2arr<N>; const N: usize; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec2arr<N>; for Vec2box; const N: usize}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec2box; for Vec2box}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec2win<'a>; for Vec2box; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec2raw; for Vec2box}
inplace_impl! {AddAssign;add_assign;+=; 1 &'a mut[Vec2]; for Vec2box; <'a>}
inplace_impl! {AddAssign;add_assign;+=; 1 &'a [Vec2]; for Vec2box; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec2arr<N>; for Vec2win<'a>; const N: usize; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec2box; for Vec2win<'a>; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec2win<'a>; for Vec2win<'a>; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec2raw; for Vec2win<'a>; <'a>}
inplace_impl! {AddAssign;add_assign;+=; 1 &'a mut[Vec2]; for Vec2win<'a>; <'a>}
inplace_impl! {AddAssign;add_assign;+=; 1 &'a [Vec2]; for Vec2win<'a>; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec2arr<N>; for Vec2raw; const N: usize}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec2box; for Vec2raw}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec2win<'a>; for Vec2raw; <'a>}
pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec2raw; for Vec2raw}
inplace_impl! {AddAssign;add_assign;+=; 1 &'a mut[Vec2]; for Vec2raw; <'a>}
inplace_impl! {AddAssign;add_assign;+=; 1 &'a [Vec2]; for Vec2raw; <'a>}

//Display
use std::fmt;

disp_impl! {Disp Vec2arr<N>; const N: usize}
disp_impl! {Disp Vec2box}
disp_impl! {Disp Vec2win<'a>; <'a>}
disp_impl! {Disp Vec2raw}

//Div
use std::ops::Div;

pv_value_impl! {Div;div;/; 1 f32; for Vec2arr<N>; out: Vec2arr<N>; const N: usize}
pv_value_impl! {Div;div;/; 1 f32; for Vec2box; out: Vec2box}

//DivAssign
use std::ops::DivAssign;

pv_inplace_impl! {DivAssign;div_assign;/=; 0 f32; for Vec2arr<N>; const N: usize}
pv_inplace_impl! {DivAssign;div_assign;/=; 0 f32; for Vec2box}
pv_inplace_impl! {DivAssign;div_assign;/=; 0 f32; for Vec2win<'a>; <'a>}
pv_inplace_impl! {DivAssign;div_assign;/=; 0 f32; for Vec2raw}

//Mult
use std::ops::Mul;

pv_value_impl! {Mul;mul;*; 1 f32; for Vec2arr<N>; out: Vec2arr<N>; const N: usize}
pv_value_impl! {Mul;mul;*; 1 f32; for Vec2box; out: Vec2box}
pv_value_impl! {Mul;mul;*; 2 Vec2arr<N>; for f32; out: Vec2arr<N>; const N: usize}
pv_value_impl! {Mul;mul;*; 2 Vec2box; for f32; out: Vec2box}
pv_dot_impl! {Dot Vec2arr<N>; for Vec2arr<N>; const N: usize}
pv_dot_impl! {Dot Vec2box; for Vec2arr<N>; const N: usize}
pv_dot_impl! {Dot Vec2win<'a>; for Vec2arr<N>; const N: usize; <'a>}
pv_dot_impl! {Dot Vec2raw; for Vec2arr<N>; const N: usize}
pv_dot_impl! {Dot Vec2arr<N>; for Vec2box; const N: usize}
pv_dot_impl! {Dot Vec2box; for Vec2box}
pv_dot_impl! {Dot Vec2win<'a>; for Vec2box; <'a>}
pv_dot_impl! {Dot Vec2raw; for Vec2box}
pv_dot_impl! {Dot Vec2arr<N>; for Vec2win<'a>; const N: usize; <'a>}
pv_dot_impl! {Dot Vec2box; for Vec2win<'a>; <'a>}
pv_dot_impl! {Dot Vec2arr<N>; for Vec2raw; const N: usize}
pv_dot_impl! {Dot Vec2box; for Vec2raw}

//MultAssign
use std::ops::MulAssign;

pv_inplace_impl! {MulAssign;mul_assign;*=; 0 f32; for Vec2arr<N>; const N: usize}
pv_inplace_impl! {MulAssign;mul_assign;*=; 0 f32; for Vec2box}
pv_inplace_impl! {MulAssign;mul_assign;*=; 0 f32; for Vec2win<'a>; <'a>}
pv_inplace_impl! {MulAssign;mul_assign;*=; 0 f32; for Vec2raw}

//Neg
use std::ops::Neg;

impl<const N: usize> Neg for Vec2arr<N> {
        type Output = Vec2arr<N>;
        fn neg(self) -> Vec2arr<N> {
		let mut tmp: Vec2arr<N> = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..N {
                        tmp[i] = Vec2 { x: -self[i].x, y: -self[i].y };
                }
		unsafe { std::mem::transmute::<_, Vec2arr<N>>(tmp) }
        }
}

impl Neg for Vec2box {
        type Output = Vec2box;
        fn neg(self) -> Vec2box {
                let mut tmp = Box::<[Vec2]>::new_uninit_slice(self.len());
                let tmp = unsafe {
                        for i in 0..self.len() {
                                tmp[i].as_mut_ptr().write( Vec2 { x: -self[i].x, y: -self[i].y } );
                        }
                        tmp.assume_init()
                };
                Vec2box(tmp)
        }
}

//Sub
use std::ops::Sub;

pv_value_impl! {Sub;sub;-; 1 f32; for Vec2arr<N>; out: Vec2arr<N>; const N: usize}
pv_value_impl! {Sub;sub;-; 1 f32; for Vec2box; out: Vec2box}
pv_value_impl! {Sub;sub;-; 3 Vec2arr<N>; for Vec2arr<N>; out: Vec2arr<N>; const N: usize}
pv_value_impl! {Sub;sub;-; 3 Vec2box; for Vec2arr<N>; out: Vec2arr<N>; const N: usize}
pv_value_impl! {Sub;sub;-; 3 Vec2win<'a>; for Vec2arr<N>; out: Vec2arr<N>; const N: usize; <'a>}
pv_value_impl! {Sub;sub;-; 3 Vec2raw; for Vec2arr<N>; out: Vec2arr<N>; const N: usize}
pv_value_impl! {Sub;sub;-; 3 Vec2arr<N>; for Vec2box; out: Vec2arr<N>; const N: usize}
pv_value_impl! {Sub;sub;-; 3 Vec2box; for Vec2box; out: Vec2box}
pv_value_impl! {Sub;sub;-; 3 Vec2win<'a>; for Vec2box; out: Vec2box; <'a>}
pv_value_impl! {Sub;sub;-; 3 Vec2raw; for Vec2box; out: Vec2box}
pv_value_impl! {Sub;sub;-; 3 Vec2arr<N>; for Vec2win<'a>; out: Vec2arr<N>; const N: usize; <'a>}
pv_value_impl! {Sub;sub;-; 3 Vec2box; for Vec2win<'a>; out: Vec2box; <'a>}
pv_value_impl! {Sub;sub;-; 3 Vec2arr<N>; for Vec2raw; out: Vec2arr<N>; const N: usize}
pv_value_impl! {Sub;sub;-; 3 Vec2box; for Vec2raw; out: Vec2box}

//SubAssign
use std::ops::SubAssign;

pv_inplace_impl! {SubAssign;sub_assign;-=; 0 f32; for Vec2arr<N>; const N: usize}
pv_inplace_impl! {SubAssign;sub_assign;-=; 0 f32; for Vec2box}
pv_inplace_impl! {SubAssign;sub_assign;-=; 0 f32; for Vec2win<'a>; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 0 f32; for Vec2raw}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec2arr<N>; for Vec2arr<N>; const N: usize}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec2box; for Vec2arr<N>; const N: usize}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec2win<'a>; for Vec2arr<N>; const N: usize; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec2raw; for Vec2arr<N>; const N: usize}
inplace_impl! {SubAssign;sub_assign;-=; 1 &'a mut[Vec2]; for Vec2arr<N>; const N: usize; <'a>}
inplace_impl! {SubAssign;sub_assign;-=; 1 &'a [Vec2]; for Vec2arr<N>; const N: usize; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec2arr<N>; for Vec2box; const N: usize}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec2box; for Vec2box}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec2win<'a>; for Vec2box; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec2raw; for Vec2box}
inplace_impl! {SubAssign;sub_assign;-=; 1 &'a mut[Vec2]; for Vec2box; <'a>}
inplace_impl! {SubAssign;sub_assign;-=; 1 &'a [Vec2]; for Vec2box; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec2arr<N>; for Vec2win<'a>; const N: usize; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec2box; for Vec2win<'a>; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec2win<'a>; for Vec2win<'a>; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec2raw; for Vec2win<'a>; <'a>}
inplace_impl! {SubAssign;sub_assign;-=; 1 &'a mut[Vec2]; for Vec2win<'a>; <'a>}
inplace_impl! {SubAssign;sub_assign;-=; 1 &'a [Vec2]; for Vec2win<'a>; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec2arr<N>; for Vec2raw; const N: usize}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec2box; for Vec2raw}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec2win<'a>; for Vec2raw; <'a>}
pv_inplace_impl! {SubAssign;sub_assign;-=; 1 Vec2raw; for Vec2raw}
inplace_impl! {SubAssign;sub_assign;-=; 1 &'a mut[Vec2]; for Vec2raw; <'a>}
inplace_impl! {SubAssign;sub_assign;-=; 1 &'a [Vec2]; for Vec2raw; <'a>}

//Vec2 Methods

impl<const N: usize> Vec2arr<N> {

        pub fn mag(&self) -> [f32; N] {
		let mut tmp: [f32; N] = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..N {
                        tmp[i] = (self[i].x*self[i].x + self[i].y*self[i].y).sqrt();
                }
                unsafe { std::mem::transmute::<_, [f32; N]>(tmp) }
        }

        pub fn mag2(&self) -> [f32; N] {
                let mut tmp: [f32; N] = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..N {
                        tmp[i] = self[i].x*self[i].x + self[i].y*self[i].y;
                }
                unsafe { std::mem::transmute::<_, [f32; N]>(tmp) }
        }

        pub fn norm(&self) -> Vec2arr<N> {
		let mut tmp: Vec2arr<N> = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..N {
			let mag : f32 = (self[i].x*self[i].x + self[i].y*self[i].y).sqrt();
                        tmp[i] = Vec2 { x: self[i].x/mag, y: self[i].y/mag };
                }
                unsafe { std::mem::transmute::<_, Vec2arr<N>>(tmp) }
        }

	pub fn transform<F : Fn(Vec2) -> Vec2>(&mut self, f: F) {
		for i in 0..N {
                        self[i] = f(self[i]);
                }
	}

	pub fn ftoarr<F : Fn(usize) -> Vec2>(f: F) -> Vec2arr<N> {
		let mut tmp: Vec2arr<N> = unsafe { MaybeUninit::uninit().assume_init() };
		for i in 0..N {
                        tmp[i] = f(i);
                }
		unsafe { std::mem::transmute::<_, Vec2arr<N>>(tmp) }
	}
}

impl Vec2box {

	pub fn new_uninit_box(len: usize) -> Box<[MaybeUninit<Vec2>]> {
		Box::<[Vec2]>::new_uninit_slice(len)
	}

	pub fn new_zeroed(len: usize) -> Vec2box {
                unsafe { Vec2box(Box::<[Vec2]>::new_zeroed_slice(len).assume_init()) }
        }

	pub fn from_raw_parts(ptr: *mut Vec2, len: usize) -> Vec2box {
                unsafe { Vec2box(Box::from_raw(slice_from_raw_parts_mut(ptr, len))) }
        }

        pub fn new_from_arr<const N: usize>(arr: Vec2arr<N>) -> Vec2box {
                let mut new = Vec2box::new_uninit_box(N);
                let new = unsafe {
			for i in 0..N {
                        	new[i].as_mut_ptr().write(arr[i]);
                	}
			new.assume_init()
		};
                Vec2box(new)
        }

        pub fn new_from_box(arr: Vec2box) -> Vec2box {
                let mut new = Vec2box::new_uninit_box(arr.len());
                let new = unsafe {
                        for i in 0..arr.len() {
                                new[i].as_mut_ptr().write(arr[i]);
                        }
                        new.assume_init()
                };
                Vec2box(new)
        }

        pub fn new_from_win<'a>(arr : Vec2win<'a>) -> Vec2box {
                let mut new = Vec2box::new_uninit_box(arr.len());
                let new = unsafe {
                        for i in 0..arr.len() {
                                new[i].as_mut_ptr().write(arr[i]);
                        }
                        new.assume_init()
                };
                Vec2box(new)
        }

	pub fn new_from_raw<'a>(arr : Vec2raw) -> Vec2box {
                let mut new = Vec2box::new_uninit_box(arr.len());
                let new = unsafe {
                        for i in 0..arr.len() {
                                new[i].as_mut_ptr().write(arr[i]);
                        }
                        new.assume_init()
                };
                Vec2box(new)
        }

	pub fn mag(&self) -> Box<[f32]> {
		let mut tmp = Box::<[f32]>::new_uninit_slice(self.len());
                let tmp = unsafe {
                        for i in 0..self.len() {
                        	tmp[i].as_mut_ptr().write((self[i].x*self[i].x + self[i].y*self[i].y).sqrt());
			}
			tmp.assume_init()
                };
                tmp
        }

        pub fn mag2(&self) -> Box<[f32]> {
		let mut tmp = Box::<[f32]>::new_uninit_slice(self.len());
                let tmp = unsafe {
                        for i in 0..self.len() {
                        	tmp[i].as_mut_ptr().write(self[i].x*self[i].x + self[i].y*self[i].y);
			}
			tmp.assume_init()
                };
                tmp
        }

        pub fn norm(&self) -> Vec2box {
		let mut tmp = Box::<[Vec2]>::new_uninit_slice(self.len());
                let tmp = unsafe {
                        for i in 0..self.len() {
                        	let mag : f32 = (self[i].x*self[i].x + self[i].y*self[i].y).sqrt();
                        	tmp[i].as_mut_ptr().write( Vec2 { x: self[i].x/mag, y: self[i].y/mag } );
			}
			tmp.assume_init()
                };
                Vec2box(tmp)
        }

	pub fn transform<F: Fn(Vec2) -> Vec2>(&mut self, f: F) {
                for i in 0..self.len() {
                        self[i] = f(self[i]);
                }
        }

	pub fn ftoarr<F: Fn(usize) -> Vec2>(f: F, n: usize) -> Vec2box {
		let mut tmp = Box::<[Vec2]>::new_uninit_slice(n);
                let tmp = unsafe {
                        for i in 0..n {
                        	tmp[i].as_mut_ptr().write(f(i));
			}
			tmp.assume_init()
                };
                Vec2box(tmp)
        }
}

// Vec2win Methods

impl<'a> Vec2win<'a> {

	pub fn transform<F: Fn(Vec2) -> Vec2>(&mut self, f: F) {
                for i in 0..self.len() {
                        self[i] = f(self[i]);
                }
        }

}

//Vec2raw Methods

impl Vec2raw {

	fn layout(len: usize) -> Layout {
		Layout::from_size_align(Layout::new::<Vec2>().size()*len,Layout::new::<Vec2>().align()).unwrap()
	}

	pub unsafe fn new_uninit_ptr(len: usize) -> *mut [MaybeUninit<Vec2>] {
		let ptr = alloc(Vec2raw::layout(len)) as *mut MaybeUninit<Vec2>;
		slice_from_raw_parts_mut(ptr, len)
	}

	pub fn from_raw_parts(ptr: *mut Vec2, len: usize) -> Vec2raw {
		Vec2raw(slice_from_raw_parts_mut(ptr, len))
	}

	pub fn new_zeroed(len: usize) -> Vec2raw {
		let ptr = unsafe { Vec2raw::new_uninit_ptr(len) };
		for i in 0..len {
			unsafe { (*ptr.as_mut_ptr().add(i)).write(Vec2::ZERO) };
		}
		Vec2raw(ptr as *mut [Vec2])
	}

	pub fn new_from_arr<const N:usize>(arr: Vec2arr<N>) -> Vec2raw {
		let ptr = unsafe { Vec2raw::new_uninit_ptr(N) };
                for i in 0..N {
			unsafe { (*ptr.as_mut_ptr().add(i)).write(arr[i]) };
                }
                Vec2raw(ptr as *mut [Vec2])
	}

	pub fn new_from_box(arr: Vec2box) -> Vec2raw {
		let ptr = unsafe { Vec2raw::new_uninit_ptr(arr.len()) };
                for i in 0..arr.len() {
                        unsafe { (*ptr.as_mut_ptr().add(i)).write(arr[i]) };
                }
                Vec2raw(ptr as *mut [Vec2])
	}

	pub fn new_from_win<'a>(arr: Vec2win<'a>) -> Vec2raw {
		let ptr = unsafe { Vec2raw::new_uninit_ptr(arr.len()) };
                for i in 0..arr.len() {
                        unsafe { (*ptr.as_mut_ptr().add(i)).write(arr[i]) };
                }
                Vec2raw(ptr as *mut [Vec2])
	}

	pub fn new_from_raw<'a>(arr : Vec2raw) -> Vec2raw {
		let ptr = unsafe { Vec2raw::new_uninit_ptr(arr.len()) };
                for i in 0..arr.len() {
                        unsafe { (*ptr.as_mut_ptr().add(i)).write(arr[i]) };
                }
                Vec2raw(ptr as *mut [Vec2])
	}

	pub fn from_arr<const N: usize>(&mut self, arr: Vec2arr<N>) {
		if self.len() != arr.len() { panic!("slice and array inequal length"); }
		for i in 0..self.len() {
			self[i] = arr[i];
		}
	}

	pub fn from_box(&mut self, arr: Vec2box) {
		if self.len() != arr.len() { panic!("slices inequal length"); }
		for i in 0..self.len() {
                        self[i] = arr[i];
                }
	}

	pub fn from_win<'a>(&mut self, arr : Vec2win<'a>) {
		if self.len() != arr.len() { panic!("slices inequal length"); }
		for i in 0..self.len() {
                        self[i] = arr[i];
                }
	}

	pub fn to_arr<const N: usize>(&self) -> Vec2arr<N> {
		if self.len() != N { panic!("slice and array inequal length"); }
		let mut tmp: Vec2arr<N> = unsafe { MaybeUninit::uninit().assume_init() };
		for i in 0..N {
                        tmp[i] = self[i];
                }
                unsafe { std::mem::transmute::<_, Vec2arr<N>>(tmp) }
	}

	pub fn to_box(&self) -> Vec2box {
		let mut tmp = Box::<[Vec2]>::new_uninit_slice(self.len());
		let tmp = unsafe {
                        for i in 0..self.len() {
                                tmp[i].as_mut_ptr().write(self[i]);
                        }
                        tmp.assume_init()
                };
		Vec2box(tmp)
	}

	pub fn transform<F: Fn(Vec2) -> Vec2>(&mut self, f: F) {
                for i in 0..self.len() {
                        self[i] = f(self[i]);
                }
        }

}
