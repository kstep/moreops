//! # A set of useful simple additional methods
//!
//! Examples:
//!
//! ```rust
//! use moreops::*;
//!
//! // Simple wrapping into Option
//! let some_num = 123.some();
//! let none_num = none::<i32>();
//!
//! // Simple wrapping into Result
//! let ok = 123.ok();
//! let err = "Error!".to_owned().err();
//!
//! // If-like operations with Option
//! let x = 42;
//! let answer = (x % 2 == 0).option("even").unwrap_or("odd");
//!
//! // Tap into some result (like `<|` and `|>` operators from Scalaz)
//! fn f() -> i32 {
//!     123
//! }
//! assert_eq!(f().tap(|x| println!("{:?}", x)), 123);
//! assert_eq!(f().then(|x| x * 2), 246);
//!
//! // Swap result
//! assert_eq!(123.ok().swap(), 123.err());
//! assert_eq!(123.ok().swap().swap(), 123.ok());
//!
//! // Wrap into tuple
//! let one = 123.once();
//! let two = 123.twice();
//! let three = 123.thrice();
//!
//! // Apply functions to tuples of args directly
//! let x = (2, 3, 4).apply(|a, b, c| a * b * c);
//! assert_eq!(x, 24);
//! ```

pub trait OptionOps {
    fn some(self) -> Option<Self> where Self: Sized {
        Some(self)
    }
}

impl<T> OptionOps for T {}

pub fn none<T>() -> Option<T> {
    None
}

pub trait ResultOps<T = ()> {
    fn ok(self) -> Result<Self, T> where Self: Sized {
        Ok(self)
    }

    fn err(self) -> Result<T, Self> where Self: Sized {
        Err(self)
    }
}

impl<T> ResultOps for T {}

pub trait BoolOps {
    fn option<T>(self, x: T) -> Option<T>;
}

impl BoolOps for bool {
    fn option<T>(self, x: T) -> Option<T> {
        if self {
            Some(x)
        } else {
            None
        }
    }   
}

pub trait TapOps {
    fn tap<R, F: Fn(&Self) -> R>(self, f: F) -> Self where Self: Sized {
        let _ = f(&self);
        self
    }

    fn then<R, F: FnOnce(Self) -> R>(self, f: F) -> R where Self: Sized {
        f(self)
    }
}

impl<T> TapOps for T {}

pub trait TupleOps: Sized {
    fn once(self) -> (Self,) {
        (self,)
    }

    fn twice(self) -> (Self, Self) where Self: Clone {
        (self.clone(), self)
    }

    fn thrice(self) -> (Self, Self, Self) where Self: Clone {
        (self.clone(), self.clone(), self)
    }
}

impl<T> TupleOps for T {}

pub fn once<A>(a: A) -> (A,) { (a,) }
pub fn twice<A: Clone>(a: A) -> (A, A) { (a.clone(), a) }
pub fn thrice<A: Clone>(a: A) -> (A, A, A) { (a.clone(), a.clone(), a) }

pub trait ResultExt<T, E>: Sized {
    fn swap(self) -> Result<E, T>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn swap(self) -> Result<E, T> {
        match self {
            Ok(v) => Err(v),
            Err(e) => Ok(e),
        }
    }
}

macro_rules! impl_apply_ops {
    ($name:ident, $($t:ident),+) => {
        pub trait $name<$($t),*> {
            fn apply<R, F: FnOnce($($t),*) -> R>(self, f: F) -> R;
        }
        impl<$($t),*> $name<$($t),*> for ($($t),*,) {
            #[allow(non_snake_case)]
            fn apply<R, F: FnOnce($($t),*) -> R>(self, f: F) -> R {
                let ($($t),*,) = self;
                f($($t),*)
            }
        }
    }
}
impl_apply_ops!(Apply1Ops, A1);
impl_apply_ops!(Apply2Ops, A1, A2);
impl_apply_ops!(Apply3Ops, A1, A2, A3);
impl_apply_ops!(Apply4Ops, A1, A2, A3, A4);
impl_apply_ops!(Apply5Ops, A1, A2, A3, A4, A5);
impl_apply_ops!(Apply6Ops, A1, A2, A3, A4, A5, A6);
impl_apply_ops!(Apply7Ops, A1, A2, A3, A4, A5, A6, A7);
impl_apply_ops!(Apply8Ops, A1, A2, A3, A4, A5, A6, A7, A8);
impl_apply_ops!(Apply9Ops, A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_apply_ops!(Apply10Ops, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);
impl_apply_ops!(Apply11Ops, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11);
impl_apply_ops!(Apply12Ops, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12);

#[test]
fn it_works() {
    assert_eq!(1.some(), Some(1));
    assert_eq!(none::<i32>(), None);

    assert_eq!(true.option(1), Some(1));
    assert_eq!(false.option(1), None);

    assert_eq!(1.tap(|&n| {
        assert_eq!(n, 1);
        2
    }), 1);

    assert_eq!(123.ok().swap(), 123.err());

    assert_eq!(123.once(), (123,));
    assert_eq!(123.twice(), (123, 123));
    assert_eq!(123.thrice(), (123, 123, 123));

    assert_eq!((2, 3).apply(::std::ops::Mul::mul), 6);
}
