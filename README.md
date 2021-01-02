# shortscale

[![CI](https://github.com/jldec/shortscale-rs/workflows/CI/badge.svg)](https://github.com/jldec/shortscale-rs/actions)  
[Rust docs](https://docs.rs/shortscale) | [crates.io](https://crates.io/crates/shortscale)

Rust lib to convert numbers into English words.

The [short scale](https://en.wikipedia.org/wiki/Long_and_short_scale_words#Comparison),
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

For the JavaScript version see [jldec/shortscale](https://github.com/jldec/shortscale).

Copyright 2021, Jürgen Leschner - github.com/jldec - MIT license
