use bencher::{benchmark_group, benchmark_main, black_box, Bencher};
use shortscale;

#[cfg(extra)]
use std::fmt::Write;

const NUM: u64 = 740_991;

fn a_shortscale(b: &mut Bencher) {
    let mut cnt: u64 = 0;
    let mut bytes: usize = 0;
    b.iter(|| {
        let new_string = shortscale::shortscale(black_box(NUM));
        cnt += 1;
        bytes += new_string.len();
    });
    // println!("a_shortscale {} iterations, {} bytes", cnt, bytes);
}

fn b_shortscale_string_writer_no_alloc(b: &mut Bencher) {
    let mut buf = String::with_capacity(238);
    let mut cnt: u64 = 0;
    let mut bytes: usize = 0;
    b.iter(|| {
        buf.clear();
        shortscale::shortscale_string_writer(&mut buf, black_box(NUM));
        cnt += 1;
        bytes += buf.len();
    });
    // println!("b_shortscale_string_writer_no_alloc {} iterations, {} bytes", cnt, bytes);
}

#[cfg(extra)]
fn e_display_no_alloc(b: &mut Bencher) {
    let mut buf = String::with_capacity(238);
    b.iter(|| {
        buf.clear();
        let words = shortscale::extra::NumWords::new(black_box(NUM));
        write!(&mut buf, "{}", words).unwrap();
    });
}

#[cfg(extra)]
fn c_str_push(b: &mut Bencher) {
    b.iter(|| {
        shortscale::extra::shortscale_str_push(black_box(NUM));
    });
}

#[cfg(extra)]
fn d_vec_push(b: &mut Bencher) {
    b.iter(|| {
        shortscale::extra::shortscale_vec_push(black_box(NUM));
    });
}

#[cfg(extra)]
fn f_vec_concat(b: &mut Bencher) {
    b.iter(|| {
        shortscale::extra::shortscale_vec_concat(black_box(NUM));
    });
}

#[cfg(extra)]
fn g_string_join(b: &mut Bencher) {
    b.iter(|| {
        shortscale::extra::shortscale_string_join(black_box(NUM));
    });
}

#[cfg(extra)]
benchmark_group!(
    benches,
    a_shortscale,
    b_shortscale_string_writer_no_alloc,
    c_str_push,
    d_vec_push,
    e_display_no_alloc,
    f_vec_concat,
    g_string_join,
);

#[cfg(not(extra))]
benchmark_group!(benches, a_shortscale, b_shortscale_string_writer_no_alloc);

benchmark_main!(benches);

/*
// writer code for experimental performance evaluation

for word in WORDS.iter() {
    f.write_str(word)?;
    f.write_str(" ")?;
};
Ok(())

#[allow(dead_code)]
const WORDS: [&str; 35] = [
    "nine",
    "hundred",
    "and",
    "ninety",
    "nine",
    "quadrillion",
    "nine",
    "hundred",
    "and",
    "ninety",
    "nine",
    "trillion",
    "nine",
    "hundred",
    "and",
    "ninety",
    "nine",
    "billion",
    "nine",
    "hundred",
    "and",
    "ninety",
    "nine",
    "million",
    "nine",
    "hundred",
    "and",
    "ninety",
    "nine",
    "thousand",
    "nine",
    "hundred",
    "and",
    "ninety",
    "nine",
];
*/
