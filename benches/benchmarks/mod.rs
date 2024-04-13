use criterion::{criterion_group, Criterion};
use pollster::block_on;
use pollster::FutureExt;
use wgpu_npo2::cpu_next_power_of_two;
use wgpu_npo2::npo2;

fn npo2_benchmark(c: &mut Criterion) {
    c.bench_function("cpu_next_power_of_two", |b| {
        b.iter_batched(
            initial,
            |_| wgpu_npo2::cpu_next_power_of_two(vec![1; 65535]),
            criterion::BatchSize::LargeInput,
        )
    });
    c.bench_function("exc run", |b| {
        b.iter_batched(
            initial,
            |exc| exc.run(vec![1; 65535]).block_on().unwrap(),
            criterion::BatchSize::LargeInput,
        )
    });
}

fn initial() -> wgpu_npo2::Excutor {
    wgpu_npo2::Excutor::new().block_on().unwrap()
}

criterion_group!(benches, npo2_benchmark);
