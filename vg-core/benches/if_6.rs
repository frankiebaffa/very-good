use {
    vg_core::Parser,
    criterion::{ criterion_group, criterion_main, Criterion, },
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("If 6", |b| b.iter(|| Parser::compile(
        "./test/if/6",
        "./test/if/6/template.jinja",
    )));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);