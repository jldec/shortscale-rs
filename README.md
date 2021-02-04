# shortscale

[![CI](https://github.com/jldec/shortscale-rs/workflows/CI/badge.svg)](https://github.com/jldec/shortscale-rs/actions)  
[Rust docs](https://docs.rs/shortscale) | [crates.io](https://crates.io/crates/shortscale)

Rust lib to convert numbers into English words.

This module was written as an exploration of JavaScript and Rust [documented here](https://jldec.me/forays-from-node-to-rust).

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

let mut my_string = String::from("The number 27 in words is ");
my_string.reserve(1024); // pre-allocate capacity (for performance only)

shortscale_string_writer(&mut my_string, 27);
assert_eq!(my_string, "The number 27 in words is twenty seven");
```

### Extra
As a record of my first foray into rust, older implementations are preserved under
[shortscale::extra](https://docs.rs/shortscale/latest/shortscale/extra/index.html).
For benchmarks run `RUSTFLAGS="--cfg extra" cargo bench`. 

GitHub Actions, running on Ubuntu.
```txt
test a_shortscale                        ... bench:         290 ns/iter (+/- 20)
test b_shortscale_string_writer_no_alloc ... bench:         265 ns/iter (+/- 47)
test c_str_push                          ... bench:         278 ns/iter (+/- 44)
test d_vec_push                          ... bench:         264 ns/iter (+/- 30)
test e_display_no_alloc                  ... bench:         687 ns/iter (+/- 283)
test f_vec_concat                        ... bench:       1,622 ns/iter (+/- 72)
test g_string_join                       ... bench:       1,751 ns/iter (+/- 57)
```

On MacOS Catalina 2.6 GHz Intel Core i7 memory allocation appears to be a lot slower.
```txt
test a_shortscale                        ... bench:         251 ns/iter (+/- 18)
test b_shortscale_string_writer_no_alloc ... bench:         191 ns/iter (+/- 11)
test c_str_push                          ... bench:         247 ns/iter (+/- 22)
test d_vec_push                          ... bench:         363 ns/iter (+/- 26)
test e_display_no_alloc                  ... bench:         498 ns/iter (+/- 21)
test f_vec_concat                        ... bench:       4,459 ns/iter (+/- 344)
test g_string_join                       ... bench:       5,549 ns/iter (+/- 378)
```

### JavaScript

Running `npm run bench` on the JavaScript version at [jldec/shortscale](https://github.com/jldec/shortscale)
shows that JavaScript is really fast as well - faster on MacOS than my first 2 naive rust implementations.

Ubuntu, Node v14
```
20000 calls, 3280000 bytes, 2211 ns/call
20000 calls, 3280000 bytes, 2180 ns/call
20000 calls, 3280000 bytes, 2172 ns/call
```
MacOS
```txt
20000 calls, 3280000 bytes, 1342 ns/call
20000 calls, 3280000 bytes, 1363 ns/call
20000 calls, 3280000 bytes, 1342 ns/call
```

Copyright 2021, Jürgen Leschner - github.com/jldec - MIT license
