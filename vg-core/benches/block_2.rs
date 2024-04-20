use {
    vg_core::Parser,
    criterion::{ criterion_group, criterion_main, Criterion, },
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Block 2", |b| b.iter(|| Parser::compile(
        "./test/block/2",
        "./test/block/2/template.jinja",
    )));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
