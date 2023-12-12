use day7::*;

fn main() {
    let mut input = include_str!("input.txt");
    let mut hand_and_bids = parsing::parse_file(&mut input);
    // dbg!(input);
    println!("part1: {}", part1(&mut hand_and_bids));
    println!("part2: {}", part2(input));
}
