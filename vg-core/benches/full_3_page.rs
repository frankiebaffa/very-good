use {
    vg_core::Parser,
    criterion::{ criterion_group, criterion_main, Criterion, },
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Full 3 Page", |b| b.iter(|| Parser::compile(
        "./test/full/3",
        "./test/full/3/pages/page.jinja",
    )));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
