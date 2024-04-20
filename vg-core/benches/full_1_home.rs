use {
    vg_core::Parser,
    criterion::{ criterion_group, criterion_main, Criterion, },
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Full 1 Home", |b| b.iter(|| Parser::compile(
        "./test/full/1",
        "./test/full/1/home.jinja",
    )));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
