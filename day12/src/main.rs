use day12::*;

fn main() {
    let input = include_str!("input.txt");
    let answer1 = part1(input);
    assert!(answer1 < 10613);
    assert!(answer1 < 8563);
    assert!(answer1 < 8322);
    println!("part1: {}", answer1);
    println!("part2: {}", part2(input));
}
