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
    let mut count = 0;
    let mut current = find_node(&nodes, "AAA");
    for step in steps.iter().cycle() {
        let next = match step {
            Step::Left => &current.left,
            Step::Right => &current.right
        };
        current = find_node(&nodes, next);
        count += 1;
        if current.label == "ZZZ" { break }
    }
    count
}

fn find_node<'a>(nodes: &'a [Node], label: &str) -> &'a Node {
    nodes.iter().find(|node| node.label == label).unwrap()
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
