use criterion::{criterion_group, Criterion};
use wgpu_npo2::cpu_next_power_of_two;
use wgpu_npo2::npo2;

fn npo2_benchmark(c: &mut Criterion) {
    c.bench_function("npo2 ten", |b| {
        b.iter_batched(
            initial,
            |_| npo2(vec![1,2,3,4,5,6,7,8,9,10]),
            criterion::BatchSize::LargeInput,
        )
    });
    c.bench_function("npo2 twenty", |b| {
        b.iter_batched(
            initial,
            |_| npo2(vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]),
            criterion::BatchSize::LargeInput,
        )
    });
}

fn initial() {}

criterion_group!(benches, npo2_benchmark);
