use {
    vg_core::Parser,
    criterion::{ criterion_group, criterion_main, Criterion, },
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Block 4", |b| b.iter(|| Parser::compile(
        "./test/block/4",
        "./test/block/4/template.jinja",
    )));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
