# shortscale

[![CI](https://github.com/jldec/shortscale-rs/workflows/CI/badge.svg)](https://github.com/jldec/shortscale-rs/actions)  
[Rust docs](https://docs.rs/shortscale) | [crates.io](https://crates.io/crates/shortscale)

Rust lib to convert numbers into English words.

The [short scale](https://en.wikipedia.org/wiki/Long_and_short_scales#Comparison),
has different words for each power of 1000.

This library expresses numbers from zero to thousands,
millions, billions, trillions, and quadrillions, up to 999_999_999_999_999_999.

### Function
```rust
pub fn shortscale(num: u64) -> String
```

### Example
```rust
use shortscale::shortscale;

assert_eq!(
    shortscale(420_000_999_015),
    "four hundred and twenty billion nine hundred \
    and ninety nine thousand and fifteen"
);
```

### String writer
For efficient writing into a mutable pre-allocated string.  
The performance difference is small and varies across systems.

```rust
pub fn shortscale_string_writer(s: &mut String, num: u64)
```

```rust
use shortscale::shortscale_string_writer;
let mut my_string = String::new();
shortscale_string_writer(&mut my_string, 0);
assert_eq!(my_string, "zero");
```

### Extra
As a record of my first foray into rust, older implementations are preserved under
[shortscale::extra](https://docs.rs/shortscale/latest/shortscale/extra/index.html).
Benchmarks are also visible in the GitHub Actions logs. For benchmarks on other systems run:
```sh
RUSTFLAGS="--cfg extra" cargo bench
```

For the JavaScript version see [jldec/shortscale](https://github.com/jldec/shortscale).


Copyright 2021, Jürgen Leschner - github.com/jldec - MIT license
