use {
    vg_core::Parser,
    criterion::{ criterion_group, criterion_main, Criterion, },
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Extends 1", |b| b.iter(|| Parser::compile(
        "./test/extends/1",
        "./test/extends/1/fragment.jinja",
    )));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
