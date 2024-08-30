use criterion::*;

fn bench(c: &mut Criterion) {
    c.bench_function("benchmark", |b| b.iter(|| {
    }));
}

criterion_group!(
    benches,
    bench,
);
criterion_main!(benches);