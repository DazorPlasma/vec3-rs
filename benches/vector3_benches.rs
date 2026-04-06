//! Vector3<f64> benchmarks.

use criterion::{Criterion, criterion_group, criterion_main};
use std::{hint::black_box, time::Duration};
use vec3_rs::Vector3;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec3 add group");
    group.sample_size(10000);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(30));
    group.bench_function("vec3 add", |b| {
        b.iter(|| {
            let v1: Vector3<f64> = Vector3::random();
            let v2 = Vector3::random();
            black_box(v1 + v2)
        });
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
