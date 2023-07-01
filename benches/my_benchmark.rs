use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use random_word::lang::Lang;

fn fast_benchmark(c: &mut Criterion) {
    #[cfg(feature = "de")]
    let lang = Lang::De;
    #[cfg(feature = "en")]
    let lang = Lang::En;
    #[cfg(feature = "es")]
    let lang = Lang::Es;
    #[cfg(feature = "fr")]
    let lang = Lang::Fr;

    let mut group = c.benchmark_group("Fast");

    group.bench_with_input(
        BenchmarkId::new("gen", format!("{:?}", lang)),
        &lang,
        |b, &_| b.iter(|| random_word::gen(lang)),
    );

    let len = 4;

    group.bench_with_input(
        BenchmarkId::new("all len", format!("{:?}", (lang, len))),
        &(lang, len),
        |b, &(_lang, len)| b.iter(|| random_word::all_len(black_box(len), lang)),
    );

    let ch = 'c';

    group.bench_with_input(
        BenchmarkId::new("all starts with", format!("{:?}", (lang, ch))),
        &(lang, ch),
        |b, &(_lang, ch)| b.iter(|| random_word::all_starts_with(black_box(ch), lang)),
    );

    group.finish();
}

criterion_group!(benches, fast_benchmark);
criterion_main!(benches);
