// shortscale.rs
//
//! Converts numbers into English words
//! using the [short scale](https://en.wikipedia.org/wiki/Long_and_short_scales#Comparison).
//!
//! [crates.io/crates/shortscale](https://crates.io/crates/shortscale)  
//! Copyright 2021, Jürgen Leschner - github.com/jldec - MIT license

/// Returns String with words given an unsigned integer.
///
/// Supports positive integers from 0 to 999_999_999_999_999_999.  
/// Larger values return "big number".
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

    // build a Vec of words
    let vec = [
        jillions(num, 1_000_000_000_000_000), // quadrillions
        jillions(num, 1_000_000_000_000),     // trillions
        jillions(num, 1_000_000_000),         // billions
        jillions(num, 1_000_000),             // millions
        jillions(num, 1_000),                 // thousands
        hundreds(num),
    ]
    .concat();

    // special case "and" separator word before tens and units
    let vec = concat_and(vec, tens_and_units(num));

    // return String
    vec.join(" ")
}

type Strvec = Vec<&'static str>;

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
    let num = num % 1000 / 100;
    match num {
        0 => vec![],
        _ => [lookup(num), lookup(100)].concat(),
    }
}

fn jillions(num: u64, scale: u64) -> Strvec {
    let num = num % (scale * 1000) / scale;
    match num {
        0 => vec![],
        _ => [one_to_999(num), lookup(scale)].concat(),
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
        80 => "eightty",
        90 => "ninety",
        100 => "hundred",
        1_000 => "thousand",
        1_000_000 => "million",
        1_000_000_000 => "billion",
        1_000_000_000_000 => "trillion",
        1_000_000_000_000_000 => "quadrillion",
        _ => "big number",
    }
}
