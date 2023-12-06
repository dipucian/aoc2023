use criterion::{black_box, Criterion, criterion_group, criterion_main};

use day5::*;

fn bench_part2_small(c: &mut Criterion) {
    let input = include_str!("../src/testcase1.txt");
    let almanac = parsing::parse_almanac(input).unwrap().1;

    c.bench_function("part2", |b| b.iter(||
        part2_full(black_box(&almanac))
    ));
}

fn bench_part2_limited(c: &mut Criterion) {
    let input = include_str!("../src/input.txt");
    let almanac = parsing::parse_almanac(input).unwrap().1;

    c.bench_function("part2_limited", |b| b.iter(||
        part2(black_box(&almanac), black_box(100))
    ));
}

criterion_group!(benches, bench_part2_small, bench_part2_limited);
criterion_main!(benches);