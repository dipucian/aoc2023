use std::collections::HashSet;
use std::ops::Range;

fn main() {
    let input = include_str!("input.txt");

    let cards: Vec<Card> = input.lines().map(|line| parsing::parse_card(line).unwrap().1).collect();

    println!("part1: {}", part1(&cards));
    println!("part2: {}", part2(&cards));
}

#[derive(Debug, PartialEq)]
pub struct Card {
    id: i32,
    winning: HashSet<i32>,
    present: Vec<i32>,
}

impl Card {
    fn match_count(&self) -> usize {
        let present = self.present.iter().cloned().collect::<HashSet<_>>();
        self.winning.intersection(&present).count()
    }
}

mod parsing {
    use nom::bytes::complete::tag;
    use nom::character::complete::{i32, space1};
    use nom::{IResult};
    use nom::multi::separated_list0;
    use nom::sequence::{preceded, tuple};
    use super::*;

    // example line
    // Card   1: 82 41 56 54 18 62 29 55 34 20 | 37 14 10 80 58 11 65 96 90  8 59 32 53 21 98 83 17  9 87 25 71 77 70 73 24

    pub(crate) fn parse_card(input: &str) -> IResult<&str, Card> {
        tuple((parse_card_id, preceded(tuple((tag(":"), space1)), separated_list0(space1, i32)), preceded(tuple((tag(" |"), space1)), separated_list0(space1, i32))))(input)
            .map(|(remaining, (id, winning, present))| (remaining, Card { id, winning: winning.into_iter().collect(), present }))
    }

    fn parse_card_id(input: &str) -> IResult<&str, i32> {
        preceded(tuple((tag("Card"), space1)), i32)(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_card() {
            let input = "Card   1: 82 41 56 54 18 62 29 55 34 20 | 37 14 10 80 58 11 65 96 90  8 59 32 53 21 98 83 17  9 87 25 71 77 70 73 24";
            assert_eq!(parse_card(input), Ok(("", Card {
                id: 1,
                winning: vec![82, 41, 56, 54, 18, 62, 29, 55, 34, 20].into_iter().collect(),
                present: vec![37, 14, 10, 80, 58, 11, 65, 96, 90, 8, 59, 32, 53, 21, 98, 83, 17, 9, 87, 25, 71, 77, 70, 73, 24]
            })));
        }
    }
}

fn part1(input: &Vec<Card>) -> i32 {
    input.iter().map(|card| {
        let matched = card.match_count();
        if matched > 0 {
            2_i32.pow((matched - 1) as u32)
        } else { 0 }
    }).sum()
}

// Vec<Card> -> Vec<Range<usize>> -> fold((1, Vec::new<Range<usize>>())) to Vec<(i32, Range<usize>)>

fn part2(input: &Vec<Card>) -> i32 {
    let mut card_copies: Vec<i32> = input.iter().map(|_| 1).collect();
    let ranges: Vec<_> = input.iter().enumerate()
        .map(|(idx, card)| (idx+1)..(idx+card.match_count()+1))
        .collect();
    ranges.iter().enumerate()
        .for_each(|(idx, r)| {
            let copies = card_copies[idx];
            card_copies[r.to_owned()].iter_mut().for_each(|c| *c += copies)
        });
    card_copies.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input =
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = part2(&input.lines().map(|line| parsing::parse_card(line).unwrap().1).collect());
        assert_eq!(result, 30);
    }
}