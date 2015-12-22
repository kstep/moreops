
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
}

impl<T> TapOps for T {}

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
}
