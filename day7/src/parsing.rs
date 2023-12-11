use winnow::ascii::{digit1, line_ending, space1};
use winnow::{Parser, PResult};
use winnow::combinator::{alt, repeat, separated, separated_pair};
use winnow::token::one_of;
use crate::{Hand, HandAndBid};

fn parse_card(input: &mut &str) -> PResult<u8> {
    one_of(('2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A')).map(|c| match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }).parse_next(input)
}

fn parse_hand(input: &mut &str) -> PResult<Hand> {
    let result = repeat(1.., parse_card).map(Hand).parse_next(input);
    result
}

fn parse_hand_and_bid(input: &mut &str) -> PResult<HandAndBid> {
    let result = separated_pair(parse_hand, space1, digit1.parse_to::<i32>()).map(|(hand, bid)| HandAndBid { hand, bid }).parse_next(input);
    result
}

// fn line_endings(input: &mut &str) -> PResult<()> {
//     let result = repeat(1.., line_ending).map(|_| ()).parse_next(input);
//     result
// }

pub fn parse_file(input: &mut &str) -> Vec<HandAndBid> {
    let result = separated(1.., parse_hand_and_bid, repeat(1.., line_ending).map(|a: String| ())).parse(input);
    result.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        assert_eq!(parse_card(&mut "22").unwrap(), 2);
        assert_eq!(parse_card(&mut "T").unwrap(), 10);
        assert_eq!(parse_card(&mut "A").unwrap(), 14);
    }

    #[test]
    fn test_parse_hand() {
        assert_eq!(parse_hand(&mut "23456").unwrap(), Hand(vec![2, 3, 4, 5, 6]));
    }

    #[test]
    fn test_parse_hand_and_bid() {
        assert_eq!(parse_hand_and_bid(&mut "23456 123").unwrap(), HandAndBid { hand: Hand(vec![2, 3, 4, 5, 6]), bid: 123 });
    }

    #[test]
    fn test_parse_file() {
        // let mut input = "4K8J9 314\n\n6789T 456";
        let mut input = include_str!("input.txt");
        // input.chars().take(20).for_each(|c| println!("{}", c));
        let mut hand_and_bids = parse_file(&mut input);
        dbg!(input, hand_and_bids);
    }
}