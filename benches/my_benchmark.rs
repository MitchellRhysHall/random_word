use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use random_word::{Words, Lang};

fn criterion_benchmark(c: &mut Criterion) {
    let languages = &[Lang::En, Lang::De, Lang::Es, Lang::Fr];
    let mut group = c.benchmark_group("Words");

    for lang in languages {
        group.sample_size(500).measurement_time(std::time::Duration::new(16, 0));

        group.bench_with_input(BenchmarkId::new("from", format!("{:?}", lang)), &lang, |b, &lang| {
            b.iter(|| Words::from(black_box(lang)))
        });

        let en = Words::from(lang).unwrap();

        group.bench_with_input(BenchmarkId::new("all", lang), &lang, |b, &lang| {
            b.iter(|| en.all())
        });

        for len in 1..=10 {
            group.bench_with_input(BenchmarkId::new("all len", format!("{:?}", (lang, len))), &(lang, len), |b, &(lang, len)| {
                b.iter(|| en.all_len(black_box(len)))
            });
        }

        for ch in ['a', 'c', 't'].iter() {
            group.bench_with_input(BenchmarkId::new("all starts with", format!("{:?}", (lang, *ch))), &(lang, *ch), |b, &(lang, ch)| {
                b.iter(|| en.all_starts_with(black_box(ch)))
            });
        }

        for len in 1..=10 {
            for ch in ['a', 'c', 't'].iter() {
                group.bench_with_input(BenchmarkId::new("all len starts with", format!("{:?}", (lang, len, *ch))), &(lang, len, *ch), |b, &(lang, len, ch)| {
                    b.iter(|| en.all_len_starts_with(black_box(len), black_box(ch)))
                });
            }
        }

        group.bench_with_input(BenchmarkId::new("gen", lang), &lang, |b, &lang| {
            b.iter(|| en.gen())
        });

        for len in 1..=10 {
            group.bench_with_input(BenchmarkId::new("gen len", format!("{:?}", (lang, len))), &(lang, len), |b, &(lang, len)| {
                b.iter(|| en.gen_len(black_box(len)))
            });
        }

        for ch in ['a', 'c', 't'].iter() {
            group.bench_with_input(BenchmarkId::new("gen starts with", format!("{:?}", (lang, *ch))), &(lang, *ch), |b, &(lang, ch)| {
                b.iter(|| en.gen_starts_with(black_box(ch)))
            });
        }

        for len in 1..=10 {
            for ch in ['a', 'c', 't'].iter() {
                group.bench_with_input(BenchmarkId::new("gen len starts with", format!("{:?}", (lang, len, *ch))), &(lang, len, *ch), |b, &(lang, len, ch)| {
                    b.iter(|| en.gen_len_starts_with(black_box(len), black_box(ch)))
                });
            }
        }
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
