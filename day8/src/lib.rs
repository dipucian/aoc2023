pub mod parsing;

#[derive(Debug, PartialEq, Clone)]
pub enum Step { Left, Right }

#[derive(Debug, PartialEq)]
pub struct Node {
    pub label: String,
    pub left: String,
    pub right: String,
}

pub fn part1(input: &str) -> i64 {
    let (steps, nodes) = parsing::parse_file(input);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input_text =
"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(input_text), 6);
    }

}
