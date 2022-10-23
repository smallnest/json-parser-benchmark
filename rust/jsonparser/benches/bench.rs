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

// criterion
fn criterion_twitter_benchmark(c: &mut Criterion) {
    let mut data = Vec::new();
    File::open(concat!("../../testdata/twitter.json"))
        .unwrap()
        .read_to_end(&mut data)
        .unwrap();

    let mut group = c.benchmark_group("twitter-unmarshal");
    let size = data.len();
    group.throughput(Throughput::Bytes(size as u64));

    group.bench_function("twitter-serde-unmarshal", |b| {
        b.iter(|| black_box(serde_unmarshal(&data)))
    });
    
    group.finish();
}

criterion_group!{
    name = benches;
    config = Criterion::default().significance_level(0.05).sample_size(500).measurement_time(Duration::from_secs(10));
    targets = criterion_twitter_benchmark
}

criterion_main!(benches);
