use advent_of_code_2025::day2;
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

fn bench_day2(c: &mut Criterion) {
    let ranges: Vec<day2::IdRange> = include_str!("../src/day2-input.txt")
        .trim_end()
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    c.bench_function("day2 p1", |b| {
        b.iter(|| {
            day2::run_challenge(black_box(&ranges), true);
        });
    });

    c.bench_function("day2 p2", |b| {
        b.iter(|| {
            day2::run_challenge(black_box(&ranges), false);
        });
    });
}

criterion_main!(benches);
criterion_group!(benches, bench_day2);
