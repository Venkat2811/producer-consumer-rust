// Credit for this simple eval setup goes to this post: https://medium.com/@trunghuynh/rust-101-rust-crossbeam-vs-java-disruptor-a-wow-feeling-27eaffcda9cb
// However, this benchmark proves that, disruptor implementation in rust still beats crossbeam on Intel CPUs.  On M1 & AMD, crossbeam is faster in MPSC.
// For more details, see: https://github.com/nicholassm/disruptor-rs/issues/7#issuecomment-2094341906

use criterion::{criterion_group, criterion_main, Criterion};
use producer_consumer_rust::{std_spsc, std_mpsc, crossbeam_spsc, crossbeam_mpsc, disruptor_spsc, disruptor_mpsc};

fn std_spsc_benchmark(c: &mut Criterion) {
    c.bench_function("std_spsc", |b| {
        b.iter(|| {
            std_spsc(false);
        });
    });
}

fn std_mpsc_benchmark(c: &mut Criterion) {
    c.bench_function("std_mpsc", |b| {
        b.iter(|| {
            std_mpsc(false);
        });
    });
}


fn crossbeam_spsc_benchmark(c: &mut Criterion) {
    c.bench_function("crossbeam_spsc", |b| {
        b.iter(|| {
            crossbeam_spsc(false);
        });
    });
}

fn crossbeam_mpsc_benchmark(c: &mut Criterion) {
    c.bench_function("crossbeam_mpsc", |b| {
        b.iter(|| {
            crossbeam_mpsc(false);
        });
    });
}


fn disruptor_spsc_benchmark(c: &mut Criterion) {
    c.bench_function("disruptor_spsc", |b| {
        b.iter(|| {
            disruptor_spsc(false);
        });
    });
}

fn disruptor_mpsc_benchmark(c: &mut Criterion) {
    c.bench_function("disruptor_mpsc", |b| {
        b.iter(|| {
            disruptor_mpsc(false);
        });
    });
}

criterion_group!(benches, std_spsc_benchmark, crossbeam_spsc_benchmark, disruptor_spsc_benchmark, std_mpsc_benchmark, crossbeam_mpsc_benchmark, disruptor_mpsc_benchmark);
criterion_main!(benches);
