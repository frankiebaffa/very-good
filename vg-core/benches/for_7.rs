use {
    vg_core::Parser,
    criterion::{ criterion_group, criterion_main, Criterion, },
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("For 7", |b| b.iter(|| Parser::compile(
        "./test/for/7",
        "./test/for/7/template.jinja",
    )));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
