use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vec3_rs::Vector3;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("vec3 add", |b| {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        b.iter(|| black_box(&v1) + black_box(&v2))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
