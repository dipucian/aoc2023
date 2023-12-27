use day12::*;

fn main() {
    let input = include_str!("input.txt");
    let answer1 = part1(input);
    println!("part1: {}", answer1);
    assert!(answer1 < 10613);
    assert!(answer1 < 8563);
    assert!(answer1 < 8322);
    assert_ne!(answer1, 8273);
    println!("part2: {}", part2(input));
}
