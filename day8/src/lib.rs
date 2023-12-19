use std::collections::HashMap;

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
    let (steps, nodes) = parsing::parse_file(input);
    let Atlas { last_a, first_z, left_targets, right_targets } = Atlas::from(nodes);

    let end_patterns = (0..=last_a).map(|start| {
        end_pattern(start, first_z, &left_targets, &right_targets, steps.iter())
    }).collect::<Vec<_>>();

    // X = s1 + n1*p1

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

#[derive(Debug)]
struct Atlas {
    first_z: usize,
    last_a: usize,
    left_targets: Vec<usize>,
    right_targets: Vec<usize>,
}

impl Atlas {
    fn from(nodes: Vec<Node>) -> Self {
        let mut nodes = nodes;
        nodes.sort_by_key(|node| node.label.chars().rev().collect::<String>());
        let last_a = nodes.iter().rposition(|node| node.label.ends_with("A")).unwrap();
        let first_z = nodes.iter().position(|node| node.label.ends_with("Z")).unwrap();
        let (left_targets, right_targets): (Vec<usize>, Vec<usize>) = nodes.iter().map(|node| (
            nodes.iter().position(|other| other.label == node.left).unwrap(),
            nodes.iter().position(|other| other.label == node.right).unwrap())
        ).unzip();
        Atlas { last_a, first_z, left_targets, right_targets }
    }
}

#[derive(Debug)]
struct EndPattern {
    ends: Vec<usize>,
    start: usize,
    period: usize
}
fn end_pattern<'a, I>(start_pos: usize, first_z: usize, left_targets: &[usize], right_targets: &[usize], steps: I) -> EndPattern
    where I: Iterator<Item=&'a Step> {
    let mut ends = vec!();
    let mut start = 0;
    let mut period = 0;

    let mut current_pos = start_pos;
    let mut overall_idx_map: HashMap<(usize, usize), usize> = HashMap::new();     // position and step idx to overall_idx
    let mut overall_idx: usize = 0;
    for (idx, step) in steps.enumerate().collect::<Vec<_>>().into_iter().cycle() {
        // dbg!(current_pos, idx, &step, overall_idx);
        if let Some(last_idx) = overall_idx_map.insert((current_pos, idx), overall_idx) {
            start = last_idx;
            period = overall_idx - last_idx;
            break;
        }
        // dbg!(&overall_idx_map);
        overall_idx += 1;

        let targets = match step {
            Step::Left => left_targets,
            Step::Right => right_targets
        };

        current_pos = targets[current_pos];
        if current_pos >= first_z {
            ends.push(overall_idx);
        }
    }

    EndPattern {ends, start, period}
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

const INPUT_TEXT_2: &str =
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT_TEXT_2), 6);
    }

    #[test]
    fn test_atlas() {
        let atlas = Atlas::from(parsing::parse_file(INPUT_TEXT_2).1);
        assert_eq!(atlas.last_a, 1);
        assert_eq!(atlas.first_z, 6);
        assert_eq!(atlas.left_targets, vec![2, 3, 5, 4, 7, 5, 2, 3]);
        assert_eq!(atlas.right_targets, vec![5, 5, 6, 4, 7, 5, 5, 3]);
    }

    #[test]
    fn test_end_pattern() {
        let (steps, nodes) = parsing::parse_file(INPUT_TEXT_2);
        let atlas = Atlas::from(nodes);
        let end_pattern = end_pattern(0, atlas.first_z, &atlas.left_targets, &atlas.right_targets, steps.iter());
        assert_eq!(end_pattern.ends, vec![2]);
        assert_eq!(end_pattern.start, 1);
        assert_eq!(end_pattern.period, 2);
    }

    #[test]
    fn test_end_pattern_real() {
        let (steps, nodes) = parsing::parse_file(include_str!("input.txt"));
        let atlas = Atlas::from(nodes);
        dbg!(&atlas);
        (0..=5).for_each(|i| {
            let end_pattern = end_pattern(i, atlas.first_z, &atlas.left_targets, &atlas.right_targets, steps.iter());
            // dbg!(&end_pattern);
            println!("start: {}, period: {}, ends: {:?}, {} x {} = {}", end_pattern.start, end_pattern.period, &end_pattern.ends, end_pattern.period / steps.len(), steps.len(), (end_pattern.period / steps.len()) * steps.len());
            /*
            start: 2, period: 15989, ends: [15989], 59 x 271 = 15989
            start: 6, period: 18157, ends: [18157], 67 x 271 = 18157
            start: 2, period: 19783, ends: [19783], 73 x 271 = 19783
            start: 2, period: 14363, ends: [14363], 53 x 271 = 14363
            start: 7, period: 12737, ends: [12737], 47 x 271 = 12737
            start: 3, period: 19241, ends: [19241], 71 x 271 = 19241
            can also be viewed as start = ends.head, period stays the same
            then the answer is just LCM of periods, which is 59 x 67 x 73 x 53 x 47 x 71 x 271 = 13830919117339
             */
        });
    }
    /*
    a0 + n_a * p_a == b0 + n_b * p_b
    n_a = (b0 - a0 + n_b * p_b) / p_a

    123456789X103456789X103456789X103456789X103456789X103456789X103456789X103456789X103456789X103456789X10
    12345X6745X6745X6745X6745X6745X6745X6745X6745X6745X6745X6745X6745X6745X6745X6745X6745X6745X67

    x_pos(n) = first + n * period
     */

}
