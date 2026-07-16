//! Vector3<f64> benchmarks.

use criterion::{Criterion, criterion_group, criterion_main};
use std::{hint::black_box, time::Duration};
use vec3_rs::Vector3;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec3 operations");
    group.sample_size(1000);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(2));

    let v1: Vector3<f64> = Vector3::random();
    let v2: Vector3<f64> = Vector3::random();
    let scalar: f64 = 2.5;

    // Creation
    group.bench_function("new", |b| {
        b.iter(|| Vector3::<f64>::new(black_box(1.0), black_box(2.0), black_box(3.0)));
    });
    group.bench_function("from_spherical", |b| {
        b.iter(|| Vector3::<f64>::from_spherical(black_box(1.0), black_box(0.5), black_box(0.5)));
    });
    group.bench_function("random", |b| b.iter(Vector3::<f64>::random));

    // Arithmetic
    group.bench_function("add", |b| b.iter(|| black_box(v1) + black_box(v2)));
    group.bench_function("sub", |b| b.iter(|| black_box(v1) - black_box(v2)));
    group.bench_function("mul (scalar)", |b| {
        b.iter(|| black_box(v1) * black_box(scalar));
    });
    group.bench_function("div (scalar)", |b| {
        b.iter(|| black_box(v1) / black_box(scalar));
    });
    group.bench_function("neg", |b| b.iter(|| -black_box(v1)));

    // Vector operations
    group.bench_function("dot", |b| b.iter(|| black_box(v1).dot(black_box(v2))));
    group.bench_function("cross", |b| b.iter(|| black_box(v1).cross(black_box(v2))));
    group.bench_function("magnitude", |b| b.iter(|| black_box(v1).magnitude()));
    group.bench_function("distance", |b| {
        b.iter(|| black_box(v1).distance(black_box(v2)));
    });
    group.bench_function("normalized", |b| b.iter(|| black_box(v1).normalized()));
    group.bench_function("normalize (mut)", |b| {
        b.iter_batched(
            || v1,
            |mut v| {
                v.normalize();
                black_box(v)
            },
            criterion::BatchSize::SmallInput,
        );
    });
    group.bench_function("angle", |b| b.iter(|| black_box(v1).angle(black_box(v2))));
    group.bench_function("angle_deg", |b| {
        b.iter(|| black_box(v1).angle_deg(black_box(v2)));
    });
    group.bench_function("project", |b| {
        b.iter(|| black_box(v1).project(black_box(v2)));
    });
    group.bench_function("reflect", |b| {
        b.iter(|| black_box(v1).reflect(black_box(v2)));
    });
    group.bench_function("rotated", |b| {
        b.iter(|| black_box(v1).rotated(black_box(v2), black_box(0.5)));
    });
    group.bench_function("lerp", |b| {
        b.iter(|| black_box(v1).lerp(black_box(v2), black_box(0.5)));
    });
    group.bench_function("fuzzy_equal", |b| {
        b.iter(|| black_box(v1).fuzzy_equal(black_box(v2), black_box(0.1)));
    });

    // Component-wise operations
    group.bench_function("inverse", |b| b.iter(|| black_box(v1).inverse()));
    group.bench_function("abs", |b| b.iter(|| black_box(v1).abs()));
    group.bench_function("ceil", |b| b.iter(|| black_box(v1).ceil()));
    group.bench_function("floor", |b| b.iter(|| black_box(v1).floor()));
    group.bench_function("round", |b| b.iter(|| black_box(v1).round()));
    group.bench_function("clamp", |b| {
        b.iter(|| black_box(v1).clamp(black_box(0.2), black_box(0.8)));
    });
    group.bench_function("max", |b| b.iter(|| black_box(v1).max(black_box(v2))));
    group.bench_function("min", |b| b.iter(|| black_box(v1).min(black_box(v2))));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
