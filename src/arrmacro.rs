/// creates an implmentation of [Deref](https://doc.rust-lang.org/std/ops/trait.Deref.html) for a given one element tuple struct.
///
/// This macro expects either an internal item that is a value ("val") or a pointer 
/// ("ptr").
///
/// Descriptions of input variables:
/// - tin: the input type, this will be your outside struct type.
/// - tout: the output type, this will be your internal type.
/// - gen: if your struct includes one, and only one, const generic, then this will
///   be the name of it.
/// - gent: if your struct includes one, and only one, const generic, then this will
///   be the type of that const generic.
///
/// # Examples
///
/// ```rust
/// # extern crate lineq
/// use std::ops::Deref;
/// use lineq::vec3arr::Vec3arr;
///
/// deref_impl! {Deref val Vec3arr<N>; to [Vec3; N]; const N: usize}
/// ```
#[macro_export]
macro_rules! deref_impl {
        (Deref val $tin:ty; to $tout:ty$(; const $gen:ident: $gent:ty)?) => {
                impl$(<const $gen: $gent>)? Deref for $tin {
                        type Target = $tout;
                        fn deref(&self) -> &Self::Target {
                                &self.0
                        }
                }
        };
        (Deref ptr $tin:ty; to $tout:ty) => {
                impl Deref for $tin {
                        type Target = $tout;
                        fn deref(&self) -> &Self::Target {
                                &*self.0
                        }
                }
        };
}

#[macro_export]
macro_rules! deref_mut_impl {
        (DerefMut val $tin:ty; to $tout:ty$(; const $gen:ident: $gent:ty)?) => {
                impl$(<const $gen: $gent>)? DerefMut for $tin {
                        fn deref_mut(&mut self) -> &mut $tout {
                                &mut self.0
                        }
                }
        };
        (DerefMut ptr $tin:ty; to $tout:ty) => {
                impl DerefMut for $tin {
                        fn deref_mut(&mut self) -> &mut $tout {
                                &mut*self.0
                        }
                }
        };
}

#[macro_export]
macro_rules! pv_value_impl {
	($imp:ident;$func:ident;$op:tt; 3 $rhs:ty; for $lhs:ty; out: $out:tt$(; <$lt:lifetime>)?) => {
		value_impl!{$imp;$func;$op; 3 $rhs; for $lhs; out: $out$(; <$lt>)?}
		value_impl!{$imp;$func;$op; 3 &$rhs; for $lhs; out: $out$(; <$lt>)?}
		value_impl!{$imp;$func;$op; 3 $rhs; for &$lhs; out: $out$(; <$lt>)?}
		value_impl!{$imp;$func;$op; 3 &$rhs; for &$lhs; out: $out$(; <$lt>)?}
	};
	($imp:ident;$func:ident;$op:tt; 2 $rhs:ty; for $lhs:ty; out: $out:tt) => {
		value_impl!{$imp;$func;$op; 2 $rhs; for $lhs; out: $out}
		value_impl!{$imp;$func;$op; 2 &$rhs; for $lhs; out: $out}
		value_impl!{$imp;$func;$op; 2 $rhs; for &$lhs; out: $out}
		value_impl!{$imp;$func;$op; 2 &$rhs; for &$lhs; out: $out}
	};
	($imp:ident;$func:ident;$op:tt; 1 $rhs:ty; for $lhs:ty; out: $out:tt) => {
		value_impl!{$imp;$func;$op; 1 $rhs; for $lhs; out: $out}
		value_impl!{$imp;$func;$op; 1 &$rhs; for $lhs; out: $out}
		value_impl!{$imp;$func;$op; 1 $rhs; for &$lhs; out: $out}
		value_impl!{$imp;$func;$op; 1 &$rhs; for &$lhs; out: $out}
	};
	($imp:ident;$func:ident;$op:tt; 3 $rhs:ty; for $lhs:ty; out: $out:ty; const $gen:ident: $gent:ty$(; <$lt:lifetime>)?) => {
		value_impl!{$imp;$func;$op; 3 $rhs; for $lhs; out: $out; const $gen: $gent$(; <$lt>)?}
		value_impl!{$imp;$func;$op; 3 &$rhs; for $lhs; out: $out; const $gen: $gent$(; <$lt>)?}
		value_impl!{$imp;$func;$op; 3 $rhs; for &$lhs; out: $out; const $gen: $gent$(; <$lt>)?}
		value_impl!{$imp;$func;$op; 3 &$rhs; for &$lhs; out: $out; const $gen: $gent$(; <$lt>)?}
	};
	($imp:ident;$func:ident;$op:tt; 1 $rhs:ty; for $lhs:ty; out: $out:ty; const $gen:ident: $gent:ty$(; <$lt:lifetime>)?) => {
		value_impl!{$imp;$func;$op; 1 $rhs; for $lhs; out: $out; const $gen: $gent$(; <$lt>)?}
		value_impl!{$imp;$func;$op; 1 &$rhs; for $lhs; out: $out; const $gen: $gent$(; <$lt>)?}
		value_impl!{$imp;$func;$op; 1 $rhs; for &$lhs; out: $out; const $gen: $gent$(; <$lt>)?}
		value_impl!{$imp;$func;$op; 1 &$rhs; for &$lhs; out: $out; const $gen: $gent$(; <$lt>)?}
	};
	($imp:ident;$func:ident;$op:tt; 2 $rhs:ty; for $lhs:ty; out: $out:ty; const $gen:ident: $gent:ty$(; <$lt:lifetime>)?) => {
		value_impl!{$imp;$func;$op; 2 $rhs; for $lhs; out: $out; const $gen: $gent$(; <$lt>)?}
                value_impl!{$imp;$func;$op; 2 &$rhs; for $lhs; out: $out; const $gen: $gent$(; <$lt>)?}
                value_impl!{$imp;$func;$op; 2 $rhs; for &$lhs; out: $out; const $gen: $gent$(; <$lt>)?}
                value_impl!{$imp;$func;$op; 2 &$rhs; for &$lhs; out: $out; const $gen: $gent$(; <$lt>)?}
	};
}

#[macro_export]
macro_rules! pv_inplace_impl {
	($imp:ident;$func:ident;$op:tt; 0 $rhs:ty; for $lhs:ty; const $gen:ident: $gent:ty$(; <$lt:lifetime>)?) => {
		inplace_impl!{$imp;$func;$op; 0 $rhs; for $lhs; const $gen: $gent$(; <$lt>)?}
		inplace_impl!{$imp;$func;$op; 0 &$rhs; for $lhs; const $gen: $gent$(; <$lt>)?}
	};
	($imp:ident;$func:ident;$op:tt; 1 $rhs:ty; for $lhs:ty; const $gen:ident: $gent:ty$(; <$lt:lifetime>)?) => {
		inplace_impl!{$imp;$func;$op; 1 $rhs; for $lhs; const $gen: $gent$(; <$lt>)?}
                inplace_impl!{$imp;$func;$op; 1 &$rhs; for $lhs; const $gen: $gent$(; <$lt>)?}
	};
	($imp:ident;$func:ident;$op:tt; 0 $rhs:ty; for $lhs:ty$(; <$lt:lifetime>)?) => {
		inplace_impl!{$imp;$func;$op; 0 $rhs; for $lhs$(; <$lt>)?}
		inplace_impl!{$imp;$func;$op; 0 &$rhs; for $lhs$(; <$lt>)?}
	};
	($imp:ident;$func:ident;$op:tt; 1 $rhs:ty; for $lhs:ty$(; <$lt:lifetime>)?) => {
                inplace_impl!{$imp;$func;$op; 1 $rhs; for $lhs$(; <$lt>)?}
                inplace_impl!{$imp;$func;$op; 1 &$rhs; for $lhs$(; <$lt>)?}
        };
}

#[macro_export]
macro_rules! pv_dot_impl {
	(Dot $rhs:ty; for $lhs:ty$(; <$lt:lifetime>)?) => {
		dot_impl!{Dot $rhs; for $lhs$(; <$lt>)?}
		dot_impl!{Dot &$rhs; for $lhs$(; <$lt>)?}
		dot_impl!{Dot $rhs; for &$lhs$(; <$lt>)?}
		dot_impl!{Dot &$rhs; for &$lhs$(; <$lt>)?}
	};
	(Dot $rhs:ty; for $lhs:ty; const $gen:ident: $gent:ty$(; <$lt:lifetime>)?) => {
		dot_impl!{Dot $rhs; for $lhs; const $gen: $gent$(; <$lt>)?}
		dot_impl!{Dot &$rhs; for $lhs; const $gen: $gent$(; <$lt>)?}
		dot_impl!{Dot $rhs; for &$lhs; const $gen: $gent$(; <$lt>)?}
		dot_impl!{Dot &$rhs; for &$lhs; const $gen: $gent$(; <$lt>)?}
	};
}

#[macro_export]
macro_rules! value_impl {
	($imp:ident;$func:ident;$op:tt; 3 $rhs:ty; for $lhs:ty; out: $out:tt$(; <$lt:lifetime>)?) => {
                impl$(<$lt>)? $imp<$rhs> for $lhs {
                        type Output = $out;
                        #[inline]
                        fn $func(self, rhs: $rhs) -> $out {
                                if self.len() != rhs.len() { panic!("slices inequal length"); }
                                let mut tmp = $out::new_uninit_box(self.len());
                                let tmp = unsafe {
                                        for i in 0..self.len() {
                                                tmp[i].as_mut_ptr().write( self[i] $op rhs[i] );
                                        }
                                        tmp.assume_init()
                                };
                                $out(tmp)
                        }
                }
        };
	($imp:ident;$func:ident;$op:tt; 2 $rhs:ty; for $lhs:ty; out: $out:tt) => {
                impl $imp<$rhs> for $lhs {
                        type Output = $out;
                        #[inline]
                        fn $func(self, rhs: $rhs) -> $out {
                                let mut tmp = $out::new_uninit_box(rhs.len());
                                let tmp = unsafe {
                                        for i in 0..rhs.len() {
                                                tmp[i].as_mut_ptr().write( self $op rhs[i] );
                                        }
                                        tmp.assume_init()
                                };
                                $out(tmp)
                        }
                }
        };
	($imp:ident;$func:ident;$op:tt; 1 $rhs:ty; for $lhs:ty; out: $out:tt) => {
                impl $imp<$rhs> for $lhs {
                        type Output = $out;
                        #[inline]
                        fn $func(self, rhs: $rhs) -> $out {
                                let mut tmp = $out::new_uninit_box(self.len());
                                let tmp = unsafe {
                                        for i in 0..self.len() {
                                                tmp[i].as_mut_ptr().write( self[i] $op rhs );
                                        }
                                        tmp.assume_init()
                                };
                                $out(tmp)
                        }
                }
        };
	($imp:ident;$func:ident;$op:tt; 3 $rhs:ty; for $lhs:ty; out: $out:ty; const $gen:ident: $gent:ty$(; <$lt:lifetime>)?) => {
                impl<$($lt,)?const $gen: $gent> $imp<$rhs> for $lhs {
                        type Output = $out;
			#[inline]
                        fn $func(self, rhs: $rhs) -> $out {
				if self.len() != rhs.len() { panic!("slice and array inequal length"); }
                                let mut tmp: $out = unsafe { MaybeUninit::uninit().assume_init() };
                                for i in 0..$gen {
					tmp[i] = self[i] $op rhs[i];
                                }
                                unsafe { std::mem::transmute::<_, $out>(tmp) }
                        }
                }
        };
	($imp:ident;$func:ident;$op:tt; 1 $rhs:ty; for $lhs:ty; out: $out:ty; const $gen:ident: $gent:ty$(; <$lt:lifetime>)?) => {
                impl<$($lt,)?const $gen: $gent> $imp<$rhs> for $lhs {
                        type Output = $out;
                        #[inline]
                        fn $func(self, rhs: $rhs) -> $out {
                                let mut tmp: $out = unsafe { MaybeUninit::uninit().assume_init() };
                                for i in 0..$gen {
                                        tmp[i] = self[i] $op rhs;
                                }
                                unsafe { std::mem::transmute::<_, $out>(tmp) }
                        }
                }
        };
	($imp:ident;$func:ident;$op:tt; 2 $rhs:ty; for $lhs:ty; out: $out:ty; const $gen:ident: $gent:ty$(; <$lt:lifetime>)?) => {
                impl<$($lt,)?const $gen: $gent> $imp<$rhs> for $lhs {
                        type Output = $out;
                        #[inline]
                        fn $func(self, rhs: $rhs) -> $out {
                                let mut tmp: $out = unsafe { MaybeUninit::uninit().assume_init() };
                                for i in 0..$gen {
                                        tmp[i] = self $op rhs[i];
                                }
                                unsafe { std::mem::transmute::<_, $out>(tmp) }
                        }
                }
        };
}

#[macro_export]
macro_rules! inplace_impl {
	($imp:ident;$func:ident;$op:tt; 0 $rhs:ty; for $lhs:ty; const $gen:ident: $gent:ty$(; <$lt:lifetime>)?) => {
		impl<$($lt,)?const $gen: $gent> $imp<$rhs> for $lhs {
			#[inline]
        		fn $func(&mut self, rhs: $rhs) {
                		for i in 0..self.len() {
                        		self[i] $op rhs;
                		}
        		}
		}
	};
	($imp:ident;$func:ident;$op:tt; 1 $rhs:ty; for $lhs:ty; const $gen:ident: $gent:ty$(; <$lt:lifetime>)?) => {
                impl<$($lt,)?const $gen: $gent> $imp<$rhs> for $lhs {
			#[inline]
                        fn $func(&mut self, rhs: $rhs) {
                                if self.len() != rhs.len() { panic!("slice and array inequal length"); }
                                for i in 0..self.len() {
                                        self[i] $op rhs[i];
                                }
                        }
                }
        };
	($imp:ident;$func:ident;$op:tt; 0 $rhs:ty; for $lhs:ty$(; <$lt:lifetime>)?) => {
                impl$(<$lt>)? $imp<$rhs> for $lhs {
			#[inline]
                        fn $func(&mut self, rhs: $rhs) {
                                for i in 0..self.len() {
                                        self[i] $op rhs;
                                }
                        }
                }
        };
        ($imp:ident;$func:ident;$op:tt; 1 $rhs:ty; for $lhs:ty$(; <$lt:lifetime>)?) => {
                impl$(<$lt>)? $imp<$rhs> for $lhs {
			#[inline]
                        fn $func(&mut self, rhs: $rhs) {
				if self.len() != rhs.len() { panic!("slices inequal length"); }
                                for i in 0..self.len() {
                                        self[i] $op rhs[i];
                                }
                        }
                }
        };
}

#[macro_export]
macro_rules! dot_impl {
	(Dot $rhs:ty; for $lhs:ty$(; <$lt:lifetime>)?) => {
                impl$(<$lt>)? Mul<$rhs> for $lhs {
                        type Output = Box<[f32]>;
                        #[inline]
                        fn mul(self, rhs: $rhs) -> Box<[f32]> {
                                if self.len() != rhs.len() { panic!("slices inequal length"); }
                                let mut tmp = Box::<[f32]>::new_uninit_slice(self.len());
                                let tmp = unsafe {
                                        for i in 0..self.len() {
                                                tmp[i].as_mut_ptr().write(self[i]*rhs[i]);
                                        }
                                        tmp.assume_init()
                                };
                                tmp
                        }
                }
        };
	(Dot $rhs:ty; for $lhs:ty; const $gen:ident: $gent:ty$(; <$lt:lifetime>)?) => {
                impl<$($lt,)?const $gen: $gent> Mul<$rhs> for $lhs {
                        type Output = [f32; N];
                        #[inline]
                        fn mul(self, rhs: $rhs) -> [f32; N] {
                                if self.len() != rhs.len() { panic!("slice and array inequal length"); }
                                let mut tmp: [f32; N] = unsafe { MaybeUninit::uninit().assume_init() };
                                for i in 0..$gen {
                                        tmp[i] = self[i]*rhs[i];
                                }
                                unsafe { std::mem::transmute::<_, [f32; N]>(tmp) }
                        }
                }
        };
}
