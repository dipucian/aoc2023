use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::{IResult, Parser};
use nom::character::complete::{i32, line_ending, space1};
use nom::multi::{many_m_n, separated_list1};
use nom::sequence::separated_pair;
use crate::{Hand, HandAndBid};

fn parse_card(input: &str) -> IResult<&str, u8> {
    alt((
        value(2, tag("2")),
        value(3, tag("3")),
        value(4, tag("4")),
        value(5, tag("5")),
        value(6, tag("6")),
        value(7, tag("7")),
        value(8, tag("8")),
        value(9, tag("9")),
        value(10, tag("T")),
        value(11, tag("J")),
        value(12, tag("Q")),
        value(13, tag("K")),
        value(14, tag("A")),
        ))(input)
}

fn parse_hand_and_bid(input: &str) -> IResult<&str, HandAndBid> {
    separated_pair(many_m_n(5, 5, parse_card), space1, i32)(input)
        .map(|(remain, (hand, bid))| (remain, HandAndBid { hand: Hand(hand), bid }))
}

pub fn parse_file(input: &str) -> Vec<HandAndBid> {
    separated_list1(line_ending, parse_hand_and_bid).parse(input).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        assert_eq!(parse_card("22").unwrap().1, 2);
        assert_eq!(parse_card("T").unwrap().1, 10);
        assert_eq!(parse_card("A").unwrap().1, 14);
    }

    #[test]
    fn test_parse_hand_and_bid() {
        assert_eq!(parse_hand_and_bid("23456 123").unwrap().1, HandAndBid { hand: Hand(vec![2, 3, 4, 5, 6]), bid: 123 });
    }
}