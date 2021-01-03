// https://github.com/bheisler/criterion.rs
// https://bheisler.github.io/criterion.rs/book/getting_started.html

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

use shortscale;

// 777_777_777_777_777_777 - longest result String
// 999_999_999_999_999_999 - largest number
//   9_007_199_254_740_991 - JavaScript Number.MAX_SAFE_INTEGER
const NUM: u64 = 9_007_199_254_740_991;

fn bench_shortscale(c: &mut Criterion) {
    let mut group = c.benchmark_group("benchmark");
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(500));
    group.sample_size(10);

    let mut calls: u64 = 0;
    let mut bytes: usize = 0;

    group.bench_function("shortscale", |b| {
        b.iter(|| {
            let s = shortscale::shortscale(black_box(NUM));
            bytes += s.len();
            calls += 1;
        });
        // println!("shortscale {} calls, {} bytes", calls, bytes);
        calls = 0;
        bytes = 0;
    });

    group.bench_function("shortscale_vec_push", |b| {
        b.iter(|| {
            let s = shortscale::shortscale_vec_push(black_box(NUM));
            bytes += s.len();
            calls += 1;
        });
        // println!("shortscale_string {} calls, {} bytes", calls, bytes);
        calls = 0;
        bytes = 0;
    });

    group.bench_function("shortscale_vec_concat", |b| {
        b.iter(|| {
            let s = shortscale::shortscale_vec_concat(black_box(NUM));
            bytes += s.len();
            calls += 1;
        });
        // println!("shortscale {} calls, {} bytes", calls, bytes);
        calls = 0;
        bytes = 0;
    });

    group.bench_function("shortscale_string_join", |b| {
        b.iter(|| {
            let s = shortscale::shortscale_string_join(black_box(NUM));
            bytes += s.len();
            calls += 1;
        });
        // println!("shortscale {} calls, {} bytes", calls, bytes);
        calls = 0;
        bytes = 0;
    });
}

criterion_group!(benches, bench_shortscale);
criterion_main!(benches);
