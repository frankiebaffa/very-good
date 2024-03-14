use {
    vg_core::Parser,
    criterion::{ criterion_group, criterion_main, Criterion, },
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("full 2 page", |b| b.iter(|| Parser::compile(
        "./test/full/2",
        "./test/full/2/page.jinja",
    )));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
