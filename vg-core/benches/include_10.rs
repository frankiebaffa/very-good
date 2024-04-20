use {
    vg_core::Parser,
    criterion::{ criterion_group, criterion_main, Criterion, },
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Include 10", |b| b.iter(|| Parser::compile(
        "./test/include/10",
        "./test/include/10/page.jinja",
    )));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
