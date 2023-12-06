use day5::*;

fn main() {
    let input = include_str!("input.txt");

    let almanac = parsing::parse_almanac(input).unwrap().1;

    println!("part1: {}", part1(&almanac));
    println!("part2: {}", part2_full(&almanac));
}
