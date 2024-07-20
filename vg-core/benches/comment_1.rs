use {
    vg_core::Parser,
    criterion::{ criterion_group, criterion_main, Criterion, },
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Comment 1", |b| b.iter(|| Parser::compile(
        "./test/comment/1",
        "./test/comment/1/template.jinja",
    )));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);