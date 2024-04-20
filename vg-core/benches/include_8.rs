use {
    vg_core::Parser,
    criterion::{ criterion_group, criterion_main, Criterion, },
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Include 8", |b| b.iter(|| Parser::compile(
        "./test/include/8",
        "./test/include/8/page.jinja",
    )));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
