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

#[cfg(any(extra, doc))]
pub mod extra;
