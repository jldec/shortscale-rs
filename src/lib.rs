/// Convert numbers into English words
/// using the [short scale](https://en.wikipedia.org/wiki/Long_and_short_scales#Comparison).
///
/// Supports positive integers from 0 to 999_999_999_999_999_999.
///
/// # Example
/// ```
/// assert_eq!(
///     shortscale::shortscale(420_000_999_015),
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

    String::from(vec.join(" "))
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

#[cfg(test)]
mod tests {

    const TESTS: [(u64, &str); 46] = [
    /* 00 */
    (0, "zero"),
    (1, "one"),
    (10, "ten"),
    (11, "eleven"),
    (18, "eighteen"),
    (20, "twenty"),
    (22, "twenty two"),
    (30, "thirty"),
    (33, "thirty three"),
    (111, "one hundred and eleven"),
    /* 10 */
    (120, "one hundred and twenty"),
    (121, "one hundred and twenty one"),
    (300, "three hundred"),
    (999, "nine hundred and ninety nine"),
    (1_000, "one thousand"),
    (2_000, "two thousand"),
    (2_004, "two thousand and four"),
    (2_011, "two thousand and eleven"),
    (2_020, "two thousand and twenty"),
    (2_050, "two thousand and fifty"),
    /* 20 */
    (2_300, "two thousand three hundred"),
    (2_301, "two thousand three hundred and one"),
    (30_020, "thirty thousand and twenty"),
    (430_020, "four hundred and thirty thousand and twenty"),
    (430_920, "four hundred and thirty thousand nine hundred and twenty"),
    (999_001, "nine hundred and ninety nine thousand and one"),
    (999_120, "nine hundred and ninety nine thousand one hundred and twenty"),
    (1_000_000, "one million"),
    (1_001_000, "one million one thousand"),
    (1_002_000, "one million two thousand"),
    /* 30 */
    (1_002_004, "one million two thousand and four"),
    (1_002_011, "one million two thousand and eleven"),
    (1_002_020, "one million two thousand and twenty"),
    (1_002_050, "one million two thousand and fifty"),
    (1_002_300, "one million two thousand three hundred"),
    (1_002_301, "one million two thousand three hundred and one"),
    (1_030_020, "one million thirty thousand and twenty"),
    (1_430_020, "one million four hundred and thirty thousand and twenty"),
    (1_430_920, "one million four hundred and thirty thousand nine hundred and twenty"),
    (1_999_001, "one million nine hundred and ninety nine thousand and one"),
    /* 40 */
    (100_999_120, "one hundred million nine hundred and ninety nine thousand one hundred and twenty"),
    (999_999_120, "nine hundred and ninety nine million nine hundred and ninety nine thousand one hundred and twenty"),
    (420_000_999_015, "four hundred and twenty billion nine hundred and ninety nine thousand and fifteen"),
    (9_007_199_254_740_999, "nine quadrillion seven trillion one hundred and ninety nine billion \
        two hundred and fifty four million seven hundred and fourty thousand nine hundred and ninety nine"),
    (999_999_999_999_999_999, "nine hundred and ninety nine quadrillion nine hundred and ninety nine trillion \
        nine hundred and ninety nine billion nine hundred and ninety nine million nine hundred and ninety nine thousand \
        nine hundred and ninety nine"),
    (1_999_999_999_999_999_999, "big number")
    ];

    #[test]
    fn it_works() {
        for (num, expected) in TESTS.iter() {
            println!("test {}", num);
            assert_eq!(crate::shortscale(*num), String::from(*expected));
        }
    }
}
