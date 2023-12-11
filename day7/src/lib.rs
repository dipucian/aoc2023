pub mod parsing;

#[derive(Debug, PartialEq)]
pub struct Hand(Vec<u8>);

#[derive(Debug, PartialEq)]
pub struct HandAndBid {
    hand: Hand,
    bid: i32,
}

enum HandType {
    FiveOfAKind=6,
    FourOfAKind=5,
    FullHouse=4,
    ThreeOfAKind=3,
    TwoPair=2,
    OnePair=1,
    HighCard=0,
}

impl Hand {
    fn strength(&self) -> u64 {
        let mut power = Hand::determine_type(self) as u64;
        for &card in &self.0 {
            power *= 100;
            power += card as u64;
        }
        power
    }
    /*
    Every hand is exactly one type. From strongest to weakest, they are:
    Five of a kind, where all five cards have the same label: AAAAA
    Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    High card, where all cards' labels are distinct: 23456
     */
    fn determine_type(&self) -> HandType {
        let mut counts = [0; 15];
        for &card in &self.0 {
            counts[card as usize] += 1;
        }
        // let mut counts = counts.iter().enumerate().filter(|(_, &count)| count > 0).collect::<Vec<_>>();
        // counts.sort_by_key(|&(_, count)| -count);
        counts.sort_by_key(|&count| -count);
        let counts = counts.iter().filter(|&&count| count > 0).collect::<Vec<_>>();
        match counts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

pub fn part1(input: &mut [HandAndBid]) -> i64 {
    dbg!(&input);
    input.sort_by_key(|x| x.hand.strength());
    input
        .iter().enumerate()
        .map(|(i, x)| x.bid as i64 * (i+1) as i64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input_text = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        let mut input = parsing::parse_file(input_text);
        assert_eq!(part1(&mut input), 6440);
    }
}
