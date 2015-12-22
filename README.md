moreops
=======

A set of useful simple additional methods

Usage:

```toml
[dependencies]
moreops = "*"
```

Examples:

```rust
use moreops::*;

// Simple wrapping into Option
let some_num = 123.some();
let none_num = none::<i32>();

// Simple wrapping into Result
let ok = 123.ok();
let err = "Error!".to_owned().err();

// If-like operations with Option
let x = 42;
let answer = (x % 2 == 0).option("even").unwrap_or("odd");

// Tap into some result (like `<|` and `|>` operators from Scalaz)
fn f() -> i32 {
    123
}
assert_eq!(f().tap(|x| println!("{:?}", x)), 123);
assert_eq!(f().then(|x| x * 2), 246);
```
