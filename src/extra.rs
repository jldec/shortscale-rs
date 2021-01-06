//! Conditionally-compiled for benchmarks and historic reference.   
//! To include in tests and benchmarks set RUSTFLAGS="--cfg extra"
//!
//! ### Sample benchmarks
//!
//! cargo bench on MacOS Catalina 2.6 GHz Intel Core i7
//! ```txt
//! shortscale                228 ns
//! shortscale_vec_push       353 ns
//! shortscale_vec_concat    4602 ns
//! shortscale_string_join   5337 ns
//! ```
//!
//! cargo bench on GitHub Actions Ubuntu 18.04
//! ```txt
//! shortscale                271 ns
//! shortscale_vec_push       258 ns
//! shortscale_vec_concat    1455 ns
//! shortscale_string_join   1614 ns
//! ```
//!

use crate::map;

type Strvec = Vec<&'static str>;

/// Reimplementation of `shortscale`.  
/// Uses Vec builder instead of String builder.  
/// On MacOS this implementation is ~60% slower than building a String directly.  
/// On GitHub Actions Ubuntu it is slightly faster.
pub fn shortscale_vec_push(num: u64) -> String {
    // simple lookup in map
    if num <= 20 || num > 999_999_999_999_999_999 {
        return String::from(map(num));
    }

    let mut v: Strvec = Vec::with_capacity(35);

    vec_push_scale(&mut v, num, 1_000_000_000_000_000); // quadrillions
    vec_push_scale(&mut v, num, 1_000_000_000_000); // trillions
    vec_push_scale(&mut v, num, 1_000_000_000); // billions
    vec_push_scale(&mut v, num, 1_000_000); // millions
    vec_push_scale(&mut v, num, 1_000); // thousands
    vec_push_hundreds(&mut v, num);
    let and_word: bool = v.len() > 0;
    vec_push_tens_and_units(&mut v, num, and_word);

    return v.join(" ");
}

fn vec_push_tens_and_units(v: &mut Strvec, num: u64, and_word: bool) {
    let num = num % 100;
    if num == 0 {
        return;
    }
    if and_word {
        v.push("and");
    }
    match num {
        1..=20 => v.push(map(num)),
        _ => {
            v.push(map(num / 10 * 10));
            let num = num % 10;
            match num {
                0 => (),
                _ => v.push(map(num)),
            };
        }
    };
}

fn vec_push_hundreds(v: &mut Strvec, num: u64) {
    let num = num / 100 % 10;
    if num == 0 {
        return;
    }
    v.push(map(num));
    v.push(map(100))
}

fn vec_push_scale(v: &mut Strvec, num: u64, thousands: u64) {
    let num = num / thousands % 1_000;
    if num == 0 {
        return;
    }
    vec_push_hundreds(v, num);
    let and_word: bool = (num / 100 % 10) > 0;
    vec_push_tens_and_units(v, num, and_word);
    v.push(map(thousands));
}

/* ******************************************************************** */

/// First Rust implementation (ever for me)  
/// Modeled after [javascript version](https://github.com/jldec/shortscale)  
/// Functional composition by allocating and concatenating Vecs.  
/// This is much slower than using a String builder and slower than JavaScript on MacOS.
pub fn shortscale_vec_concat(num: u64) -> String {
    // simple lookup in map
    if num <= 20 || num > 999_999_999_999_999_999 {
        return String::from(map(num));
    }

    // build a Vec of words for supported scales
    let vec = [
        scale(num, 1_000_000_000_000_000), // quadrillions
        scale(num, 1_000_000_000_000),     // trillions
        scale(num, 1_000_000_000),         // billions
        scale(num, 1_000_000),             // millions
        scale(num, 1_000),                 // thousands
        hundreds(num),
    ]
    .concat();

    // special case: "and" separator word before tens and units
    let vec = concat_and(vec, tens_and_units(num));

    // convert final Vec to String
    vec.join(" ")
}

// 0 represented with empty Vec for composition using [Vec].concat()
fn lookup(num: u64) -> Strvec {
    match num {
        0 => vec![],
        _ => vec![map(num)],
    }
}

fn tens_and_units(num: u64) -> Strvec {
    let num = num % 100;
    match num {
        0..=20 => lookup(num),
        _ => [lookup(num / 10 * 10), lookup(num % 10)].concat(),
    }
}

fn hundreds(num: u64) -> Strvec {
    let num = num / 100 % 10;
    match num {
        0 => vec![],
        _ => [lookup(num), lookup(100)].concat(),
    }
}

fn scale(num: u64, thousands: u64) -> Strvec {
    let num = num / thousands % 1_000;
    match num {
        0 => vec![],
        _ => [one_to_999(num), lookup(thousands)].concat(),
    }
}

fn one_to_999(num: u64) -> Strvec {
    concat_and(hundreds(num), tens_and_units(num))
}

// concatenate 2 Strvec's, separated with "and" if both have length
fn concat_and(v1: Strvec, v2: Strvec) -> Strvec {
    match (v1.len(), v2.len()) {
        (_, 0) => v1,
        (0, _) => v2,
        (_, _) => [v1, vec!["and"], v2].concat(),
    }
}

/* ******************************************************************** */

/// Reimplementation of `shortscale_vec_concat`  
/// Composition pushing Strings returned from functions.  
/// This is even slower than concatenating Vecs.
pub fn shortscale_string_join(num: u64) -> String {
    // simple lookup in map
    if num <= 20 || num > 999_999_999_999_999_999 {
        return String::from(map(num));
    }

    let mut s = String::with_capacity(238);

    join_words(&mut s, " ", scale_words(num, 1_000_000_000_000_000));
    join_words(&mut s, " ", scale_words(num, 1_000_000_000_000));
    join_words(&mut s, " ", scale_words(num, 1_000_000_000));
    join_words(&mut s, " ", scale_words(num, 1_000_000));
    join_words(&mut s, " ", scale_words(num, 1_000));
    join_words(&mut s, " ", hundreds_words(num));
    join_words(&mut s, " and ", tens_and_units_words(num));

    s
}

fn join_words(s: &mut String, sep: &str, words: String) {
    match (s.len(), words.len()) {
        (_, 0) => (),
        (0, _) => s.push_str(&words),
        (_, _) => {
            s.push_str(sep);
            s.push_str(&words);
        }
    }
}

// 0 represented with empty Vec for composition using [Vec].concat()
fn lookup_word(num: u64) -> String {
    match num {
        0 => String::from(""),
        _ => String::from(map(num)),
    }
}

fn tens_and_units_words(num: u64) -> String {
    let num = num % 100;
    let tens = num / 10 * 10;
    let units = num % 10;
    match (tens, units) {
        (0, _) => lookup_word(units),
        (10, _) => lookup_word(num),
        (_, 0) => lookup_word(tens),
        (_, _) => [lookup_word(tens), String::from(" "), lookup_word(units)].concat(),
    }
}

fn hundreds_words(num: u64) -> String {
    let num = num / 100 % 10;
    match num {
        0 => lookup_word(num),
        _ => [lookup_word(num), String::from(" "), lookup_word(100)].concat(),
    }
}

fn scale_words(num: u64, thousands: u64) -> String {
    let num = num / thousands % 1_000;
    match num {
        0 => lookup_word(num),
        _ => [
            one_to_999_words(num),
            String::from(" "),
            lookup_word(thousands),
        ]
        .concat(),
    }
}

fn one_to_999_words(num: u64) -> String {
    let h = hundreds_words(num);
    let tu = tens_and_units_words(num);
    match (h.len(), tu.len()) {
        (0, _) => tu,
        (_, 0) => h,
        (_, _) => [h, String::from(" and "), tu].concat(),
    }
}
