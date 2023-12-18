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

pub fn part2(input: &str) -> i64 {
    let (steps, mut nodes) = parsing::parse_file(input);
    nodes.sort_by_key(|node| node.label.chars().rev().collect::<String>());
    let last_a = nodes.iter().rposition(|node| node.label.ends_with("A")).unwrap();
    let first_z = nodes.iter().position(|node| node.label.ends_with("Z")).unwrap();
    let (left_targets, right_targets): (Vec<usize>, Vec<usize>) = nodes.iter().map(|node| (
        nodes.iter().position(|other| other.label == node.left).unwrap(),
        nodes.iter().position(|other| other.label == node.right).unwrap())
    ).unzip();

    let mut count = 0;
    let mut current = (0..=last_a).collect::<Vec<_>>();
    for step in steps.iter().cycle() {
        println!("{}: {:?}", count, current);
        let targets = match step {
            Step::Left => &left_targets,
            Step::Right => &right_targets
        };
        current = current.iter().map(|node_idx| {
            targets[*node_idx]
        }).collect();
        count += 1;

        if current.iter().all(|&idx| idx >= first_z) { break }
    }
    count
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

    #[test]
    fn test_part2() {
        let input_text =
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(part2(input_text), 6);
    }

    /*
0 11A = (11B, XXX) (2, 5)
1 22A = (22B, XXX) (3, 5)
2 11B = (XXX, 11Z) (5, 6)
3 22B = (22C, 22C) (4, 4)
4 22C = (22Z, 22Z) (7, 7)
5 XXX = (XXX, XXX) (5, 5)
6 11Z = (11B, XXX) (2, 5)
7 22Z = (22B, 22B) (3, 3)
     */

}
