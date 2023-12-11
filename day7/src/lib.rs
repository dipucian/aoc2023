pub mod parsing;

#[derive(Debug, PartialEq)]
pub struct Hand(Vec<u8>);

#[derive(Debug, PartialEq)]
pub struct HandAndBid {
    hand: Hand,
    bid: i32,
}
pub fn part1(input: &mut [HandAndBid]) -> i32 {
    dbg!(&input);
    input.sort_by_key(|x| x.hand.0.len());
    input
        .iter().enumerate()
        .map(|(i, x)| x.bid * (i+1) as i32)
        .sum()
}
