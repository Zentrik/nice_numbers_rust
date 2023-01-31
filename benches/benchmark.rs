use criterion::{black_box, criterion_group, criterion_main, Criterion};

use malachite::Natural;
use malachite::num::arithmetic::traits::Pow;

use nice_numbers_rust::search_range;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("search_range 1 to 10^5 base 10", |b| b.iter(|| search_range(black_box(Natural::from(1 as u32)), black_box(Natural::from(10 as u32).pow(5)), black_box(10), false)));
    c.bench_function("search_range 1 to 10^5 base 23", |b| b.iter(|| search_range(black_box(Natural::from(1 as u32)), black_box(Natural::from(10 as u32).pow(5)), black_box(23), false)));
    c.bench_function("search_range 10^50 to 10^5 + 10^50 base 10", |b| b.iter(|| search_range(black_box(Natural::from(10 as u32).pow(50)), black_box(Natural::from(10 as u32).pow(5) + Natural::from(10 as u32).pow(50)), black_box(10), false)));
    c.bench_function("search_range 10^50 to 10^5 + 10^50 base 23", |b| b.iter(|| search_range(black_box(Natural::from(10 as u32).pow(50)), black_box(Natural::from(10 as u32).pow(5) + Natural::from(10 as u32).pow(50)), black_box(23), false)));

    // c.bench_function("search_range 52260814 to 91068707 base 28", |b| b.iter(|| search_range(black_box(Natural::from(52260814 as u32)), black_box(Natural::from(91068707 as u32)), black_box(28), false)));

    // c.bench_function("search_range 1 to 10^6 base 10", |b| b.iter(|| search_range(black_box(Natural::from(1 as u32)), black_box(Natural::from(10 as u32).pow(6)), black_box(10), false)));
    // c.bench_function("search_range 1 to 10^6 base 23", |b| b.iter(|| search_range(black_box(Natural::from(1 as u32)), black_box(Natural::from(10 as u32).pow(6)), black_box(23), false)));
    // c.bench_function("search_range 10^50 to 10^6 + 10^50 base 10", |b| b.iter(|| search_range(black_box(Natural::from(10 as u32).pow(50)), black_box(Natural::from(10 as u32).pow(6) + Natural::from(10 as u32).pow(50)), black_box(10), false)));
    // c.bench_function("search_range 10^50 to 10^6 + 10^50 base 23", |b| b.iter(|| search_range(black_box(Natural::from(10 as u32).pow(50)), black_box(Natural::from(10 as u32).pow(6) + Natural::from(10 as u32).pow(50)), black_box(23), false)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);