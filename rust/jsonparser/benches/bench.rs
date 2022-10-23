#![feature(test)]

extern crate test;

#[cfg(feature = "jemallocator")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use std::fs::File;
use std::io::Read;
use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

use jsonparser::*;


fn criterion_twitter_serde_benchmark(c: &mut Criterion) {
    let mut data = Vec::new();
    File::open(concat!("../../testdata/twitter.json"))
        .unwrap()
        .read_to_end(&mut data)
        .unwrap();

    let mut group = c.benchmark_group("twitter-serde");
    let size = data.len();
    group.throughput(Throughput::Bytes(size as u64));
    let v = serde_unmarshal(&data);

    group.bench_function("twitter-serde-marshal", |b| {
        b.iter(|| black_box(serde_marshal(&v)))
    });

    group.bench_function("twitter-serde-unmarshal", |b| {
        b.iter(|| black_box(serde_unmarshal(&data)))
    });
    
    group.finish();
}

// fn criterion_twitter_simd_json_benchmark(c: &mut Criterion) {
//     let data = Vec::new();
//     let mut data0 = data.clone();
//     File::open(concat!("../../testdata/twitter.json"))
//         .unwrap()
//         .read_to_end(&mut data0)
//         .unwrap();

//     let mut group = c.benchmark_group("twitter-simd_json");
//     let size = data.len();
//     group.throughput(Throughput::Bytes(size as u64));

//     let mut data0 = data.clone();
//     let v = simd_json_unmarshal(&mut data0);

//     group.bench_function("twitter-simd-json-marshal", |b| {
//         b.iter(|| black_box(simd_json_marshal(&v)))
//     });

    
//     group.bench_with_input("twitter-simd-json-unmarshal",  &data, |b, data| {
//         b.iter(|| black_box(simd_json_unmarshal(&mut data.clone())))
//     });
    
//     group.finish();
// }

criterion_group!{
    name = benches;
    config = Criterion::default().significance_level(0.05).sample_size(500).measurement_time(Duration::from_secs(10));
    targets = criterion_twitter_serde_benchmark
}

criterion_main!(benches);
