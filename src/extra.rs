//! Conditionally-compiled for benchmarks and historic reference.   
//! To include in tests and benchmarks set RUSTFLAGS="--cfg extra"
//!
//! ### Sample benchmarks
//!
//! cargo bench on MacOS Catalina 2.6 GHz Intel Core i7
//! ```txt
//! test a_shortscale                        ... bench:         239 ns/iter (+/- 20)
//! test b_shortscale_string_writer_no_alloc ... bench:         180 ns/iter (+/- 13)
//! test c_str_push                          ... bench:         239 ns/iter (+/- 28)
//! test d_vec_push                          ... bench:         356 ns/iter (+/- 20)
//! test e_display_no_alloc                  ... bench:         510 ns/iter (+/- 53)
//! test f_vec_concat                        ... bench:       4,571 ns/iter (+/- 426)
//! test g_string_join                       ... bench:       5,573 ns/iter (+/- 347)
//! ```
//!
//! cargo bench on GitHub Actions Ubuntu 18.04
//! ```txt
//! test a_shortscale                        ... bench:         276 ns/iter (+/- 22)
//! test b_shortscale_string_writer_no_alloc ... bench:         256 ns/iter (+/- 10)
//! test c_str_push                          ... bench:         275 ns/iter (+/- 6)
//! test d_vec_push                          ... bench:         264 ns/iter (+/- 8)
//! test e_display_no_alloc                  ... bench:         640 ns/iter (+/- 53)
//! test f_vec_concat                        ... bench:       1,600 ns/iter (+/- 85)
//! test g_string_join                       ... bench:       1,754 ns/iter (+/- 9)
//! ```
//!

use crate::map;

use std::fmt;
use std::fmt::Formatter as Fmt;
use std::fmt::Result;
use std::fmt::Write;

/// Implementation writes into a pre-allocated String  
/// using NumWords Display trait.  
/// ...
pub fn shortscale_display(num: u64) -> String {
    // FIXME: better tradeoff perf against space.
    let mut s = String::with_capacity(238);
    write!(&mut s, "{}", NumWords::new(num)).unwrap();
    return s;
}

/// Expose shortscale Display trait implementation  
/// for easy to_string() or write!() into an existing String.
///
/// This experiment was supposed to be faster than shortscale
/// because we can eliminate the allocation of a new String. However,
/// the overhead of making multiple fmt::Formatter.write_str() calls turned
/// out to be greater than the cost of String allocation.
///
/// The simpler solution was to add a string writer function to shortscale
/// directly mutating an existing string, rother than going 
/// through the formatter code.
///
/// # Example
/// ```
/// use shortscale;
/// use std::fmt::Write;
///
/// let mut buf = String::with_capacity(100);
/// let numwords = shortscale::NumWords::new(420_000_999_015);
///
/// // write to buffer
/// println!("buf.len() before: {}", buf.len());
/// write!(&mut buf, "{}", numwords).unwrap();
/// println!("buf.len()  after: {}", buf.len());
///
/// // or simply convert to String (may do multiple allocs)
/// assert_eq!(
///     numwords.to_string(),
///     "four hundred and twenty billion nine hundred \
///     and ninety nine thousand and fifteen"
///     );
/// ```
#[derive(Debug)]
pub struct NumWords {
    n: u64,
}

impl NumWords {
    pub fn new(n: u64) -> Self {
        Self { n }
    }

    fn display(&self, f: &mut Fmt<'_>) -> Result {
        // short circuit single words
        if self.n <= 20 || self.n > 999_999_999_999_999_999 {
            return write!(f, "{}", map(self.n));
        }

        let mut len: usize = 0;
        self.scale(f, &mut len, 1_000_000_000_000_000)?; // quadrillions
        self.scale(f, &mut len, 1_000_000_000_000)?; // trillions
        self.scale(f, &mut len, 1_000_000_000)?; // billions
        self.scale(f, &mut len, 1_000_000)?; // millions
        self.scale(f, &mut len, 1_000)?; // thousands
        self.hundreds(f, self.n, &mut len)?;
        self.tens_and_units(f, self.n, len > 0, &mut len)?;
        Ok(())
    }

    fn tens_and_units(&self, f: &mut Fmt<'_>, num: u64, and_word: bool, len: &mut usize) -> Result {
        let num = num % 100;
        if num == 0 {
            return Ok(());
        }
        if and_word {
            self.write_word(f, "and", len)?;
        };
        match num {
            1..=20 => self.write_word(f, map(num), len)?,
            _ => {
                self.write_word(f, map(num / 10 * 10), len)?;
                let num = num % 10;
                match num {
                    0 => (),
                    _ => self.write_word(f, map(num), len)?,
                }
            }
        }
        Ok(())
    }

    fn hundreds(&self, f: &mut Fmt<'_>, num: u64, len: &mut usize) -> Result {
        let num = num / 100 % 10;
        if num == 0 {
            return Ok(());
        }
        self.write_word(f, map(num), len)?;
        self.write_word(f, map(100), len)?;
        Ok(())
    }

    fn scale(&self, f: &mut Fmt<'_>, len: &mut usize, thousands: u64) -> Result {
        let num = self.n / thousands % 1_000;
        if num == 0 {
            return Ok(());
        }
        self.hundreds(f, num, len)?;
        let and_word: bool = (num / 100 % 10) > 0;
        self.tens_and_units(f, num, and_word, len)?;
        self.write_word(f, map(thousands), len)?;
        Ok(())
    }

    fn write_word(&self, f: &mut Fmt<'_>, word: &str, len: &mut usize) -> Result {
        if *len > 0 {
            f.write_str(" ")?;
            *len += " ".len();
        }
        f.write_str(word)?;
        *len += word.len();
        Ok(())
    }
}

impl fmt::Display for NumWords {
    fn fmt(&self, f: &mut Fmt<'_>) -> Result {
        self.display(f)
    }
}


/* ******************************************************************** */

/// Implementation pushes str's directly into a preallocated String.  
/// ...
pub fn shortscale_str_push(num: u64) -> String {
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

/* ******************************************************************** */

type Strvec = Vec<&'static str>;

/// Reimplementation of shortscale_str_push.  
/// Uses Vec builder instead of String builder.  
/// On MacOS this implementation is ~60% slower than building a String directly.  
/// On GitHub Actions Ubuntu it is slightly faster.  
/// ...
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
/// ...
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

/// Reimplementation of shortscale_vec_concat  
/// Composition pushing Strings returned from functions.  
/// This is even slower than concatenating Vecs.  
/// ...
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
