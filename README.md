moreops
=======
[![license](http://img.shields.io/crates/l/moreops.png)](https://crates.io/crates/moreops)
[![version](http://img.shields.io/crates/v/moreops.png)](https://crates.io/crates/moreops)

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
assert_eq!(f().pipe(|x| x * 2), 246);

// Swap result
assert_eq!(123.ok().swap(), 123.err());
assert_eq!(123.ok().swap().swap(), 123.ok());

// Apply functions to tuples of args directly
let x = (2, 3, 4).apply(|a, b, c| a * b * c);
assert_eq!(x, 24);

// Use twice() to build map
let map = (1..10).map(twice).collect::<::std::collections::BTreeMap<_, _>>();
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
