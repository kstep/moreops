
pub trait OptionOps {
    fn some(self) -> Option<Self> where Self: Sized {
        Some(self)
    }
}

impl<T> OptionOps for T {}

pub fn none<T>() -> Option<T> {
    None
}

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

#[test]
fn it_works() {
    assert_eq!(1.some(), Some(1));
    assert_eq!(none::<i32>(), None);

    assert_eq!(true.option(1), Some(1));
    assert_eq!(false.option(1), None);
}
