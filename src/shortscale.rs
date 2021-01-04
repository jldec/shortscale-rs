// shortscale.rs
//
//! Converts numbers into English words.
//!
//! The [short scale](https://en.wikipedia.org/wiki/Long_and_short_scales#Comparison),
//! has different words for each power of 1000.
//!
//! This library expresses numbers from zero to thousands,
//! millions, billions, trillions, and quadrillions, up to 999_999_999_999_999_999.
//!
//! ### Sample benchmarks
//!
//! cargo bench on MacOS Catalina 2.6 GHz Intel Core i7
//! ```txt
//! shortscale                250.56 ns
//! shortscale_vec_push       356.90 ns
//! shortscale_vec_concat   4.686 us
//! shortscale_string_join  5.395 us
//! ```
//!
//! cargo bench on GitHub Actions Ubuntu 18.04
//! ```txt
//! shortscale                271.79 ns
//! shortscale_vec_push       258.43 ns
//! shortscale_vec_concat   1.4559 us
//! shortscale_string_join  1.6146 us
//! ```
//!
//! [github](https://github.com/jldec/shortscale-rs) | [crates.io](https://crates.io/crates/shortscale)
//!
//! Copyright 2021, Jürgen Leschner - github.com/jldec - MIT license

/// Returns String with words given an unsigned integer.  
/// Builder implementation mutates pre-allocated String by calling push_str().  
///
/// Supports positive integers from 0 to 999_999_999_999_999_999.  
/// Larger values return "(big number)".
///
/// # Example
/// ```
/// use shortscale::shortscale;
///
/// assert_eq!(
///     shortscale(420_000_999_015),
///     "four hundred and twenty billion nine hundred \
///     and ninety nine thousand and fifteen"
///     );
/// ```
pub fn shortscale(num: u64) -> String {
    // simple lookup in map
    if num <= 20 || num > 999_999_999_999_999_999 {
        return String::from(map(num));
    }

    let mut s = String::with_capacity(238);

    push_scale(&mut s, num, 1_000_000_000_000_000); // quadrillions
    push_scale(&mut s, num, 1_000_000_000_000); // trillions
    push_scale(&mut s, num, 1_000_000_000); // billions
    push_scale(&mut s, num, 1_000_000); // millions
    push_scale(&mut s, num, 1_000); // thousands
    push_hundreds(&mut s, num);
    let and_word: bool = s.len() > 0;
    push_tens_and_units(&mut s, num, and_word);

    return s;
}

fn push_word(s: &mut String, word: &str) {
    if s.len() > 0 {
        s.push_str(" ");
    }
    s.push_str(word);
}

fn push_tens_and_units(s: &mut String, num: u64, and_word: bool) {
    let num = num % 100;
    if num == 0 {
        return;
    }
    if and_word {
        push_word(s, "and");
    }
    match num {
        1..=20 => push_word(s, map(num)),
        _ => {
            push_word(s, map(num / 10 * 10));
            let num = num % 10;
            match num {
                0 => (),
                _ => push_word(s, map(num)),
            };
        }
    };
}

fn push_hundreds(s: &mut String, num: u64) {
    let num = num / 100 % 10;
    if num == 0 {
        return;
    }
    push_word(s, map(num));
    push_word(s, map(100))
}

fn push_scale(s: &mut String, num: u64, thousands: u64) {
    let num = num / thousands % 1_000;
    if num == 0 {
        return;
    }
    push_hundreds(s, num);
    let and_word: bool = (num / 100 % 10) > 0;
    push_tens_and_units(s, num, and_word);
    push_word(s, map(thousands));
}

fn map(num: u64) -> &'static str {
    match num {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "thirty",
        40 => "fourty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        100 => "hundred",
        1_000 => "thousand",
        1_000_000 => "million",
        1_000_000_000 => "billion",
        1_000_000_000_000 => "trillion",
        1_000_000_000_000_000 => "quadrillion",
        _ => "(big number)",
    }
}

/* ******************************************************************** */

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
/// This is ~20x slower than the String builder and ~5x slower than JavaScript.
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
