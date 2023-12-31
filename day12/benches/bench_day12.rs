use std::time::Duration;
use criterion::{black_box, Criterion, criterion_group, criterion_main};

use day12::*;

fn bench_part2_line4(c: &mut Criterion) {
    let input = include_str!("../src/input.txt");
    let input = &input.lines().skip(3).take(1).collect::<Vec<_>>().join("\n");

    c.bench_function("part2_line4", |b| b.iter(||
        part2(black_box(input))
    ));
}

fn bench_part2_20(c: &mut Criterion) {
    let input = include_str!("../src/input.txt");
    let input = &input.lines().take(20).collect::<Vec<_>>().join("\n");

    c.bench_function("part2", |b| b.iter(||
        part2(black_box(input))
    ));
}

criterion_group!{
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().sample_size(10);
    targets = bench_part2_line4
}
// criterion_group!(benches, bench_part2_line4);
criterion_main!(benches);