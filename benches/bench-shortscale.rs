use bencher::{Bencher, black_box, benchmark_main, benchmark_group};
use shortscale;

const NUM: u64 = 9_007_199_254_740_991;

fn bench_shortscale(b: &mut Bencher) {
    b.iter(|| {
        shortscale::shortscale(black_box(NUM));
    });
}

#[cfg(extra)]
fn bench_shortscale_vec_push(b: &mut Bencher) {
    b.iter(|| {
        shortscale::extra::shortscale_vec_push(black_box(NUM));
    });
}

#[cfg(extra)]
fn bench_shortscale_vec_concat(b: &mut Bencher) {
    b.iter(|| {
        shortscale::extra::shortscale_vec_concat(black_box(NUM));
    });
}

#[cfg(extra)]
fn bench_shortscale_string_join(b: &mut Bencher) {
    b.iter(|| {
        shortscale::extra::shortscale_string_join(black_box(NUM));
    });
}

#[cfg(extra)]
benchmark_group!(benches, bench_shortscale, bench_shortscale_vec_push, bench_shortscale_vec_concat, bench_shortscale_string_join);

#[cfg(not(extra))]
benchmark_group!(benches, bench_shortscale);

benchmark_main!(benches);