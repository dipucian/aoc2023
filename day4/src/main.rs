use std::collections::HashSet;

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
        let present = card.present.iter().cloned().collect::<HashSet<_>>();
        let matched = card.winning.intersection(&present).count();
        if matched > 0 {
            2_i32.pow((matched - 1) as u32)
        } else { 0 }
    }).sum()
}

fn part2(_input: &Vec<Card>) -> i32 {
    0
}
