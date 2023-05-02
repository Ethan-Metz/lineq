/// Creates an implementation of [Deref](https://doc.rust-lang.org/std/ops/trait.Deref.html) for a given one element tuple struct.
///
/// **This macro is mostly for internal use to reduce repeated lines**
///
/// This macro expects either an internal item that is a value ("val") or a pointer
/// ("ptr").
///
/// Descriptions of input variables:
/// - tin: The input type, this will be your outside struct type.
/// - tout: The output type, this will be your internal type.
/// - gen: If your struct includes one, and only one, const generic, then this will
///   be the name of it.
/// - gent: If your struct includes one, and only one, const generic, then this will
///   be the type of that const generic.
///
/// # Examples
///
/// ```rust
/// # extern crate lineq;
/// use std::ops::Deref;
/// use lineq::deref_impl;
///
/// struct f32arr<const N: usize>([f32; N]);
///
/// deref_impl! {Deref val f32arr<N>; to [f32; N]; const N: usize}
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

/// Creates an implementation of [DerefMut](https://doc.rust-lang.org/std/ops/trait.DerefMut.html) for a given one element tuple struct.
///
/// **This macro is mostly for internal use to reduce repeated lines**
///
/// This macro expects either an internal item that is a value ("val") or a pointer 
/// ("ptr").
///
/// Descriptions of input variables:
/// - tin: The input type, this will be your outside struct type.
/// - tout: The output type, this will be your internal type.
/// - gen: If your struct includes one, and only one, const generic, then this will
///   be the name of it.
/// - gent: If your struct includes one, and only one, const generic, then this will
///   be the type of that const generic.
///
/// # Examples
///
/// ```rust
/// # extern crate lineq;
/// use std::ops::DerefMut;
/// use lineq::deref_mut_impl;
///
/// struct f32arr<const N: usize>([f32; N]);
///
/// deref_mut_impl! {DerefMut val f32arr<N>; to [f32; N]; const N: usize}
/// ```
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

/// creates an implementation of some given allocating operation for a given one element tuple struct.
///
/// **This macro is mostly for internal use to reduce repeated lines**
///
/// This macro is a consolidation of four [value_impl](crate::value_impl) calls where they 
/// go through the four possibilities for pointer or value for the two given types.
///
/// This macro has three forms, marked with a 1, 2, and 3. These numbers mark in binary
/// about whether each type on each side is an array type or a single value (1 or 0 respectively).
/// For example for form 2, the first type mentioned, ie rhs, will be an array while the second type,
/// ie lhs, will be a value as the binary representation is 0b10.
///
/// Descriptions of input variables:
/// - imp: The name of the implementation for the operation that you are calling.
/// - func: The the name of the internal function for the implementation. It may be the
///   implementation name but lower case, or it may be more complex.
/// - op: The operation that is performed. If, for example, the implementation was Add, then
///   op would be +.
/// - rhs: This is the type on the right hand side of the operation.
/// - lhs: This is the type on the left hand side of the operation.
/// - out: This is the type that results after the operation, so if you add a f32 to a f32, your
///   output type is f32.
/// - gen: If your your types include one, and only one, const generic in total, then this
///    will be the name of it.
/// - gent: If your your types include one, and only one, const generic in total, then this
///    will be the type of that const generic.
/// - lt: If your your types include one, and only one, lifetime in total, then this will
///   be the name for it.
///
/// # Examples
///
/// ```rust
/// # extern crate lineq;
/// use std::ops::Add;
/// use std::mem::MaybeUninit;
/// use lineq::pv_value_impl;
/// use lineq::value_impl;
/// use lineq::vec3arr::Vec3arr;
/// use lineq::vec3arr::Vec3win;
///
/// pv_value_impl! {Add;add;+; 3 Vec3win<'a>; for Vec3arr<N>; out: Vec3arr<N>; const N: usize; <'a>}
/// // The above macro instance has the same effect as the four below.
/// value_impl! {Add;add;+; 3 Vec3win<'a>; for Vec3arr<N>; out: Vec3arr<N>; const N: usize; <'a>}
/// value_impl! {Add;add;+; 3 &Vec3win<'a>; for Vec3arr<N>; out: Vec3arr<N>; const N: usize; <'a>}
/// value_impl! {Add;add;+; 3 Vec3win<'a>; for &Vec3arr<N>; out: Vec3arr<N>; const N: usize; <'a>}
/// value_impl! {Add;add;+; 3 &Vec3win<'a>; for &Vec3arr<N>; out: Vec3arr<N>; const N: usize; <'a>}
/// ```
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

/// Creates an implementation of some given inplace operation for a given one element tuple struct.
///
/// **This macro is mostly for internal use to reduce repeated lines**
///
/// This macro is a consolidation of two [inplace_impl](crate::inplace_impl) calls where they 
/// go through the 2 possibilities for pointer or value for the given rhs.
///
/// This macro has two forms, marked with a 0 or 1. These numbers mark in binary
/// about whether rhs is an array type or a single value (1 or 0 respectively).
///
/// Descriptions of input variables:
/// - imp: The name of the implementation for the operation that you are calling.
/// - func: The the name of the internal function for the implementation. It may be the
///   implementation name but lower case, or it may be more complex.
/// - op: The operation that is performed. If, for example, the implementation was AddAssign, then
///   op would be +=.
/// - rhs: This is the type on the right hand side of the operation.
/// - lhs: This is the type on the left hand side of the operation.
/// - gen: If your your types include one, and only one, const generic in total, then this
///    will be the name of it.
/// - gent: If your your types include one, and only one, const generic in total, then this
///    will be the type of that const generic.
/// - lt: If your your types include one, and only one, lifetime in total, then this will
///   be the name for it.
///
/// # Examples
///
/// ```rust
/// # extern crate lineq;
/// use std::ops::AddAssign;
/// use lineq::pv_inplace_impl;
/// use lineq::inplace_impl;
/// use lineq::vec3arr::Vec3arr;
/// use lineq::vec3arr::Vec3win;
///
/// pv_inplace_impl! {AddAssign;add_assign;+=; 1 Vec3win<'a>; for Vec3arr<N>; const N: usize; <'a>}
/// // The above macro instance has the same effect as the two below.
/// inplace_impl! {AddAssign;add_assign;+=; 1 Vec3win<'a>; for Vec3arr<N>; const N: usize; <'a>}
/// inplace_impl! {AddAssign;add_assign;+=; 1 &Vec3win<'a>; for Vec3arr<N>; const N: usize; <'a>}
/// ```
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

/// Creates an implementation of a dot product (i.e. [Mul](https://doc.rust-lang.org/std/ops/trait.Mul.html)
/// ) for a given one element tuple struct.
///
/// **This macro is mostly for internal use to reduce repeated lines**
///
/// This macro is a consolidation of four [dot_impl](crate::dot_impl) calls where they 
/// go through the four possibilities for pointer or value for the given rhs and lhs.
///
/// Descriptions of input variables:
/// - rhs: This is the type on the right hand side of the operation.
/// - lhs: This is the type on the left hand side of the operation.
/// - gen: If your your types include one, and only one, const generic in total, then this
///    will be the name of it.
/// - gent: If your your types include one, and only one, const generic in total, then this
///    will be the type of that const generic.
/// - lt: If your your types include one, and only one, lifetime in total, then this will
///   be the name for it.
///
/// # Examples
///
/// ```rust
/// # extern crate lineq;
/// use std::ops::Mul;
/// use std::mem::MaybeUninit;
/// use lineq::pv_dot_impl;
/// use lineq::dot_impl;
/// use lineq::vec3arr::Vec3arr;
/// use lineq::vec3arr::Vec3win;
///
/// pv_dot_impl! {Dot Vec3win<'a>; for Vec3arr<N>; const N: usize; <'a>}
/// // The above macro instance has the same effect as the four below.
/// dot_impl! {Dot Vec3win<'a>; for Vec3arr<N>; const N: usize; <'a>}
/// dot_impl! {Dot &Vec3win<'a>; for Vec3arr<N>; const N: usize; <'a>}
/// dot_impl! {Dot Vec3win<'a>; for &Vec3arr<N>; const N: usize; <'a>}
/// dot_impl! {Dot &Vec3win<'a>; for &Vec3arr<N>; const N: usize; <'a>}
/// ```
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

/// Creates an implementation of some given allocating operation for a given one element tuple struct.
///
/// **This macro is mostly for internal use to reduce repeated lines**
///
/// This macro has three forms, marked with a 1, 2, and 3. These numbers mark in binary
/// about whether each type on each side is an array type or a single value (1 or 0 respectively).
/// For example for form 2, the first type mentioned, ie rhs, will be an array while the second type,
/// ie lhs, will be a value as the binary representation is 0b10.
///
/// Descriptions of input variables:
/// - imp: The name of the implementation for the operation that you are calling.
/// - func: The the name of the internal function for the implementation. It may be the
///   implementation name but lower case, or it may be more complex.
/// - op: The operation that is performed. If, for example, the implementation was Add, then
///   op would be +.
/// - rhs: This is the type on the right hand side of the operation.
/// - lhs: This is the type on the left hand side of the operation.
/// - out: This is the type that results after the operation, so if you add a f32 to a f32, your
///   output type is f32.
/// - gen: If your your types include one, and only one, const generic in total, then this
///    will be the name of it.
/// - gent: If your your types include one, and only one, const generic in total, then this
///    will be the type of that const generic.
/// - lt: If your your types include one, and only one, lifetime in total, then this will
///   be the name for it.
///
/// # Examples
///
/// ```rust
/// # extern crate lineq;
/// use std::ops::Add;
/// use std::mem::MaybeUninit;
/// use lineq::value_impl;
/// use lineq::vec3arr::Vec3arr;
/// use lineq::vec3arr::Vec3win;
///
/// value_impl! {Add;add;+; 3 Vec3win<'a>; for Vec3arr<N>; out: Vec3arr<N>; const N: usize; <'a>}
/// // The above macro instance has the same effect as all the code below except comments.
/// impl<'a,const N: usize> Add<Vec3win<'a>> for Vec3arr<N> {
/// 	type Output = Vec3arr<N>;
/// 	#[inline]
/// 	fn add(self, rhs: Vec3win<'a>) -> Vec3arr<N> {
/// 		if self.len() != rhs.len() { panic!("slice and array inequal length"); }
/// 		// The panic message and the inclusing of the panic! alltogether depend on the mode and the inputs.
/// 		let mut tmp: Vec3arr<N> = unsafe { MaybeUninit::uninit().assume_init() };
/// 		// The way that tmp is initialized and filled depends on the inputs to the macro.
/// 		for i in 0..N {
/// 			tmp[i] = self[i] + rhs[i];
/// 		}
/// 		unsafe { std::mem::transmute::<_, Vec3arr<N>>(tmp) }
/// 	}
/// }
/// ```
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

/// Creates an implementation of some given inplace operation for a given one element tuple struct.
///
/// **This macro is mostly for internal use to reduce repeated lines**
///
/// This macro has two forms, marked with a 0 or 1. These numbers mark in binary
/// about whether rhs is an array type or a single value (1 or 0 respectively).
///
/// Descriptions of input variables:
/// - imp: The name of the implementation for the operation that you are calling.
/// - func: The the name of the internal function for the implementation. It may be the
///   implementation name but lower case, or it may be more complex.
/// - op: The operation that is performed. If, for example, the implementation was AddAssign, then
///   op would be +=.
/// - rhs: This is the type on the right hand side of the operation.
/// - lhs: This is the type on the left hand side of the operation.
/// - gen: If your your types include one, and only one, const generic in total, then this
///    will be the name of it.
/// - gent: If your your types include one, and only one, const generic in total, then this
///    will be the type of that const generic.
/// - lt: If your your types include one, and only one, lifetime in total, then this will
///   be the name for it.
///
/// # Examples
///
/// ```rust
/// # extern crate lineq;
/// use std::ops::AddAssign;
/// use lineq::inplace_impl;
/// use lineq::vec3arr::Vec3arr;
/// use lineq::vec3arr::Vec3win;
///
/// inplace_impl! {AddAssign;add_assign;+=; 1 Vec3win<'a>; for Vec3arr<N>; const N: usize; <'a>}
/// // The above macro instance has the same effect as all the code below except comments.
/// impl<'a,const N: usize> AddAssign<Vec3win<'a>> for Vec3arr<N> {
/// 	#[inline]
/// 	fn add_assign(&mut self, rhs: Vec3win<'a>) {
/// 		if self.len() != rhs.len() { panic!("slice and array inequal length"); }
/// 		// The panic message and the inclusing of the panic! alltogether depend on the mode and the inputs.
/// 		for i in 0..self.len() {
/// 			self[i] += rhs[i];
/// 			// Depending on the input rhs[i] may be rhs.
/// 		}
/// 	}
/// }
/// ```
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

/// Creates an implementation of a dot product (i.e. [Mul](https://doc.rust-lang.org/std/ops/trait.Mul.html)
/// ) for a given one element tuple struct.
///
/// **This macro is mostly for internal use to reduce repeated lines**
///
/// This macro has two forms, marked with a 0 or 1. These numbers mark in binary
/// about whether rhs is an array type or a single value (1 or 0 respectively).
///
/// Descriptions of input variables:
/// - rhs: This is the type on the right hand side of the operation.
/// - lhs: This is the type on the left hand side of the operation.
/// - gen: If your your types include one, and only one, const generic in total, then this
///    will be the name of it.
/// - gent: If your your types include one, and only one, const generic in total, then this
///    will be the type of that const generic.
/// - lt: If your your types include one, and only one, lifetime in total, then this will
///   be the name for it.
///
/// # Examples
///
/// ```rust
/// # extern crate lineq;
/// use std::ops::Mul;
/// use std::mem::MaybeUninit;
/// use lineq::dot_impl;
/// use lineq::vec3arr::Vec3arr;
/// use lineq::vec3arr::Vec3win;
///
/// dot_impl! {Dot Vec3win<'a>; for Vec3arr<N>; const N: usize; <'a>}
/// // The above macro instance has the same effect as all the code below except comments.
/// impl<'a,const N: usize> Mul<Vec3win<'a>> for Vec3arr<N> {
/// 	type Output = [f32; N];
/// 	#[inline]
/// 	fn mul(self, rhs: Vec3win<'a>) -> [f32; N] {
/// 		if self.len() != rhs.len() { panic!("slice and array inequal length"); }
/// 		// The panic message and the inclusing of the panic! alltogether depend on the mode and the inputs.
/// 		let mut tmp: [f32; N] = unsafe { MaybeUninit::uninit().assume_init() };
/// 		// The way that tmp is initialized and filled depends on the inputs to the macro.
/// 		for i in 0..N {
/// 			tmp[i] = self[i]*rhs[i];
/// 		}
/// 		unsafe { std::mem::transmute::<_, [f32; N]>(tmp) }
/// 	}
/// }
/// ```
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
