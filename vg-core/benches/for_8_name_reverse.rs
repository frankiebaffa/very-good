use {
    vg_core::Parser,
    criterion::{ criterion_group, criterion_main, Criterion, },
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("For 8 By Name Reverse", |b| b.iter(|| Parser::compile(
        "./test/for/8",
        "./test/for/8/by_name_reverse.jinja",
    )));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
