use char_index::IndexedChars;

use core::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::{seq::SliceRandom, thread_rng};

pub fn perf(c: &mut Criterion) {
    let mut base = ['e'; 1000]
        .into_iter()
        .chain(
            [400, 300, 200]
                .into_iter()
                .map(|n| char::from_u32(n).unwrap()),
        )
        .collect::<Vec<char>>();

    base.shuffle(&mut thread_rng());

    let base_str = String::from_iter(&base);

    let indexed = IndexedChars::new(&base_str);

    let mut group = c.benchmark_group("index char 200");

    group.bench_function("indexed_chars", |b| {
        b.iter(|| black_box(indexed.get_char(200)))
    });
    group.bench_function("char_iter", |b| {
        b.iter(|| black_box(base_str.chars().nth(200)))
    });
    group.bench_function("vec_char", |b| b.iter(|| black_box(base.get(200))));

    println!(
        "IndexedChars: {} bytes",
        indexed.len() + indexed.chars().count()
    );
    println!("String: {} bytes", indexed.len());
    println!("Vec<char>: {} bytes", 4 * indexed.chars().count());
}

criterion_group!(benches, perf);
criterion_main!(benches);
