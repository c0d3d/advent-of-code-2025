use advent_of_code_2025::day2;
use advent_of_code_2025::day3;
use advent_of_code_2025::day3::get_input;
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

fn bench_day3(c: &mut Criterion) {
    let banks = get_input();

    c.bench_function("day3 p1", |b| {
        b.iter(|| {
            day3::run_challenge::<2>(black_box(&banks));
        })
    });

    c.bench_function("day3 p2", |b| {
        b.iter(|| {
            day3::run_challenge::<12>(black_box(&banks));
        })
    });
}

criterion_main!(benches);
criterion_group!(benches, bench_day2, bench_day3);
