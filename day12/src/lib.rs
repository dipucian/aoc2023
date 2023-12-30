mod memoization;

use std::cell::RefCell;
use std::iter;
use std::str::from_utf8;
use itertools::Itertools;
use memoize::memoize;
use crate::memoization::Memoization;

pub fn part1(input: &str) -> usize {
    input.lines()
        .map(Record::from_str)
        .map(|r| {
            possible_configurations(r).len()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input.lines()
        .map(Record::from_str)
        .inspect(|r| println!("{}", r))
        .map(Record::unfold)
        .inspect(|r| println!("{}", r))
        .map(|r| {
            possible_configurations(r).len()
            // count_configurations(&r)
        })
        .sum()
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Record {
    slots: Vec<u8>,
    counts: Vec<usize>,
}
impl Record {
    fn from_str(line: &str) -> Self {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        Record {
            slots: parts[0].bytes().collect(),
            counts: parts[1].split(',').map(|s| s.parse().unwrap()).collect(),
        }
    }

    fn unfold(mut self) -> Self {
        self.counts = self.counts.repeat(5);

        self.slots.push(b'?');
        self.slots = self.slots.repeat(5);
        self.slots.pop();

        self
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let slots = from_utf8(&self.slots).unwrap();
        let counts = self.counts.iter().map(|&n| n.to_string()).collect::<Vec<_>>().join(",");
        write!(f, "{} {}", slots, counts)
    }
}

fn possible_configurations(record: Record) -> Vec<String> {
    possible_sub_configurations(record.clone())
        .iter().map(|c| {
        let content = record.counts.iter().zip(c.iter())
            .map(|(&count, &start)| {
                format!(
                    "{}{}",
                    ".".repeat(start),
                    "#".repeat(count),
                )
            })
            .join(".");
        format!("{content:.<width$}", width=record.slots.len())
    }).collect()
}

#[memoize]
fn possible_sub_configurations(record: Record) -> Vec<Vec<usize>> {
    // println!("possible_configurations({})", record);
    let count = record.counts[0];
    if record.counts.len() == 1 {
        let starts = possible_starts(count, true, &record.slots);
        // println!("last starts: {:?}", starts);
        return starts.into_iter().map(|start| vec![start]).collect()
    }

    let min_hold = itertools::intersperse(record.counts.iter(), &1)
        .skip(1).sum::<usize>();
    let starts = if let Some(hold) = record.slots
        .iter()
        .rev()
        .enumerate()
        // to guard splitting continuous block of #s
        .position(|(idx, &b)| idx >= min_hold-1 && b != b'#') {

        possible_starts(count, false, &record.slots[..record.slots.len() - hold-1])
    } else { vec![] };
    // println!("starts: {:?}", starts);

    starts.iter().flat_map(|&start| {
        let local_offset = start + count + 1;
        let new_record = Record {
            slots: record.slots[local_offset..].to_vec(),
            counts: record.counts[1..].to_vec(),
        };
        let configs = possible_sub_configurations(new_record);
        configs.into_iter().map(|mut c| {
            c.insert(0, start);
            c
        }).collect::<Vec<_>>()
    }).collect()
}

fn count_configurations(record: &Record) -> usize {
    count_configurations_raw(&record.counts, &record.slots)
}
fn tupled_count_configurations_raw(t: (&[usize], &[u8])) -> usize {
    count_configurations_raw(t.0, t.1)
}

fn count_configurations_raw(counts: &[usize], slots: &[u8]) -> usize {
    let count = counts[0];
    if counts.len() == 1 {
        return count_possible_starts(count, true, &slots);
    }

    let min_hold = itertools::intersperse(counts.iter(), &1)
        .skip(1).sum::<usize>();
    let starts = if let Some(hold) = slots
        .iter()
        .rev()
        .enumerate()
        // to guard splitting continuous block of #s
        .position(|(idx, &b)| idx >= min_hold-1 && b != b'#') {

        possible_starts(count, false, &slots[..slots.len() - hold-1])
    } else { vec![] };
    // println!("starts: {:?}", starts);

    starts.iter().map(|&start| {
        let local_offset = start + count + 1;
        count_configurations_raw(&counts[1..], &slots[local_offset..])
    }).sum()
}

fn possible_starts(count: usize, must_consume: bool, slots: &[u8]) -> Vec<usize> {
    let offset = 0;
    // println!("possible_starts({}, {:?})", count, from_utf8(slots).unwrap());
    let (start, end) = if let Some(first_sharp) = slots.iter().position(|&b| b == b'#') {
        let last_sharp = slots.iter().rposition(|&b| b == b'#').unwrap();
        let start = if must_consume { last_sharp.saturating_sub(count - 1) } else { 0 };
        let end = (first_sharp + count).min(slots.len());
        (start, end)
    } else {
        (0, slots.len())
    };

    if end < start {
        return vec![];
    }
    // println!("start: {}, end: {}", start, end);

    let expanded = if end == slots.len() {
        iter::once(b'.')
            .chain(slots[start..].iter().copied())
            .chain(iter::once(b'.'))
            .collect::<Vec<_>>()
    } else {
        iter::once(b'.')
            .chain(slots[start..=end].iter().copied())
            .collect::<Vec<_>>()
    };

    // println!("expanded: {:?}", from_utf8(&expanded).unwrap());

    expanded.windows(count + 2)
        .enumerate()
        .filter(|(_idx, w)| {
            // println!("idx: {}: {:?}", _idx, from_utf8(w).unwrap());
            if let [before, center@ .., after] = w {
                could_be_empty(before) && could_be_empty(after) && full(center)
            }
            else { false }
        })
        .map(|(idx, _w)| offset + idx + start)
        .collect()
}

fn count_possible_starts(count: usize, must_consume: bool, slots: &[u8]) -> usize {
    let offset = 0;
    // println!("possible_starts({}, {:?})", count, from_utf8(slots).unwrap());
    let (start, end) = if let Some(first_sharp) = slots.iter().position(|&b| b == b'#') {
        let last_sharp = slots.iter().rposition(|&b| b == b'#').unwrap();
        let start = if must_consume { last_sharp.saturating_sub(count - 1) } else { 0 };
        let end = (first_sharp + count).min(slots.len());
        (start, end)
    } else {
        (0, slots.len())
    };

    if end < start {
        return 0;
    }
    // println!("start: {}, end: {}", start, end);

    let expanded = if end == slots.len() {
        iter::once(b'.')
            .chain(slots[start..].iter().copied())
            .chain(iter::once(b'.'))
            .collect::<Vec<_>>()
    } else {
        iter::once(b'.')
            .chain(slots[start..=end].iter().copied())
            .collect::<Vec<_>>()
    };

    // println!("expanded: {:?}", from_utf8(&expanded).unwrap());

    expanded.windows(count + 2)
        .enumerate()
        .filter(|(_idx, w)| {
            // println!("idx: {}: {:?}", _idx, from_utf8(w).unwrap());
            if let [before, center@ .., after] = w {
                could_be_empty(before) && could_be_empty(after) && full(center)
            }
            else { false }
        })
        .count()
}

fn full(slots: &[u8]) -> bool {
    slots.iter().all(could_be_full)
}

fn could_be_full(slot: &u8) -> bool {
    *slot == b'?' || *slot == b'#'
}
fn could_be_empty(slot: &u8) -> bool {
    *slot == b'?' || *slot == b'.'
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fmt::Pointer;
    use itertools::Itertools;
    use super::*;

    const TEST_INPUT: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 21);
    }

    macro_rules! test_configuration {
        ($name: ident, $line: expr, $answer: expr) => {
            #[test]
            fn $name() {
                let record = Record::from_str($line);
                println!("{}", &record);
                assert_eq!(possible_configurations(record).len(), $answer);
            }
        };
    }
    test_configuration!(test_line1, "???.### 1,1,3", 1);
    test_configuration!(test_line2, ".??..??...?##. 1,1,3", 4);
    test_configuration!(test_line3, "?#?#?#?#?#?#?#? 1,3,1,6", 1);
    test_configuration!(test_line4, "????.#...#... 4,1,1", 1);
    test_configuration!(test_line5, "????.######..#####. 1,6,5", 4);
    test_configuration!(test_line6, "?###???????? 3,2,1", 10);

    test_configuration!(line961, "#?????????.#? 2,3,1", 5);
    test_configuration!(line985, "???????##??#?.?#?#?? 1,9,2,1", 5);
    test_configuration!(line985_subcase, ".?#?#?? 2,1", 1);

    test_configuration!(all_5_2, "????? 1,1", 6);
    test_configuration!(all_5_3, "????? 1,1,1", 1);
    test_configuration!(all_7_3, "??????? 1,1,1", 10);
    test_configuration!(line477, "?.????#..??#? 1,1,2,3", 4);

    macro_rules! acceptance {
        ($name: ident, $line: expr) => {
            #[test]
            fn $name() {
                let record = Record::from_str($line);
                println!("{}", &record);
                let configs = possible_configurations(record.clone());
                configs.iter().for_each(|c| println!("{}", c));
                assert_unique(&configs);
                assert_dot_and_hash_preserved(&record, &configs);
            }
        };
    }

    acceptance!(acceptance_line22, ".?#?#.?????#?.#? 4,1,2,2");
    acceptance!(acceptance_line160, "##?.??#?#? 2,3");

    fn assert_unique(configs: &[String]) {
        let mut unique = configs.to_vec();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), configs.len());
    }
    fn assert_dot_and_hash_preserved(record: &Record, configs: &[String]) {
        let dot_positions = record.slots.iter().positions(|&b| b == b'.').collect::<HashSet<_>>();
        let hash_positions = record.slots.iter().positions(|&b| b == b'#').collect::<HashSet<_>>();

        for config in configs {
            let config = config.as_bytes();
            let config_dot_positions = config.iter().positions(|&b| b == b'.').collect::<HashSet<_>>();
            let config_hash_positions = config.iter().positions(|&b| b == b'#').collect::<HashSet<_>>();
            assert!(config_dot_positions.is_superset(&dot_positions));
            assert!(config_hash_positions.is_superset(&hash_positions));
        }
    }

    #[test]
    fn acceptance_test_all() {
        let input = include_str!("input.txt");
        for (idx, line) in input.lines().enumerate() {
            let record = &Record::from_str(line);
            println!("line {}:  {}", idx+1, record);
            let configs = possible_configurations(record.clone());
            assert_unique(&configs);
            assert_dot_and_hash_preserved(record, &configs);
        }
    }

    #[test]
    fn test_possible_start() {
        assert_eq!(possible_starts(1, true, b"???.#?"), vec![4]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 525152);
    }

    // #[test]
    fn try_reference_equal() {
        let v = vec![1, 2, 3, 4, 5];

        let r1 = &v[1..];
        let r2 = &r1[1..3];
        let r3 = &v[2..4];
        let r4 = r2.clone();

        let str = "abc";
        let string = str.to_string();

        try_clone(str);
        try_clone(&string);

        println!("{:p} {:p} {:p} {:p}", r1, r2, r3, r4);

        assert_eq!(r3, r2);
        assert!(false)
    }
    fn try_clone<T: Clone + Pointer>(t: T) {
        let t2 = t.clone();
        println!("{:p} {:p}", t, t2);
    }
}
