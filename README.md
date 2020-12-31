[![CI](https://github.com/jldec/shortscale-rs/workflows/CI/badge.svg)](https://github.com/jldec/shortscale-rs/actions)

[Rust function](https://docs.rs/shortscale)  
Converts numbers into English words using the [short scale](https://en.wikipedia.org/wiki/Long_and_short_scales#Comparison).

Supports positive integers from 0 to 999_999_999_999_999_999.

### library function signature
```rust
pub fn shortscale(num: u64) -> String
```

### Example
```rust
assert_eq!(
    shortscale(420_000_999_015),
    "four hundred and twenty billion nine hundred and ninety nine thousand and fifteen"
);
```

For the JavaScript version see [jldec/shortscale](https://github.com/jldec/shortscale).
