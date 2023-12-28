use std::iter;
use std::str::from_utf8;

pub fn part1(input: &str) -> usize {
    input.lines()
        .map(Record::from_str)
        .enumerate()
        .map(|(idx, r)| {
            let count = possible_configurations(&r).len();
            println!("{} -> {}  (line {})", r, count, idx+1);
            count
        })
        .sum()
}

#[derive(Debug)]
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
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let slots = self.slots.iter().map(|&b| b as char).collect::<String>();
        let counts = self.counts.iter().map(|&n| n.to_string()).collect::<Vec<_>>().join(",");
        write!(f, "{} {}", slots, counts)
    }
}

fn possible_configurations(record: &Record) -> Vec<String> {
    // println!("possible_configurations({}|{})", so_far, record);
    if record.counts.len() == 1 {
        let starts = possible_starts(record.counts[0], true, &record.slots);
        // println!("last starts: {:?}", starts);
        return starts.iter().map(|&start| format!(
            "{}{}{}",
            ".".repeat(start),
            "#".repeat(record.counts[0]),
            ".".repeat(record.slots.len() - start - record.counts[0]),
        )).collect()
    }
    let count = record.counts[0];

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
        let new_record = Record {
            slots: record.slots[(start+count+1)..].to_vec(),
            counts: record.counts[1..].to_vec(),
        };
        let configs = possible_configurations(&new_record);
        configs.into_iter().map(|c| format!(
            "{}{}.{}",
            ".".repeat(start),
            "#".repeat(count),
            c
        )).collect::<Vec<_>>()
    }).collect()
}

fn possible_starts(count: usize, must_consume: bool, slots: &[u8]) -> Vec<usize> {
    // println!("possible_starts({}, {:?})", count, from_utf8(slots).unwrap());
    let (start, end) = if let Some(first_sharp) = slots.iter().position(|&b| b == b'#') {
        let start = if must_consume { first_sharp.saturating_sub(count - 1) } else { 0 };
        let end = (first_sharp + count).min(slots.len());
        (start, end)
    } else {
        (0, slots.len())
    };
    if let Some(last_hash) = slots.iter().rposition(|&b| b == b'#') {
        if must_consume && last_hash >= end {
            return vec![];
        }
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
        .map(|(idx, _w)| idx + start)
        .collect()
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

pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
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
                let record = &Record::from_str($line);
                println!("{}", record);
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
                let record = &Record::from_str($line);
                println!("{}", record);
                let configs = possible_configurations(record);
                configs.iter().foreach(|c| println!("{}", c));
                assert_unique(&configs);
                assert_dot_and_hash_preserved(record, &configs);
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
        // config should have dots and hashes in the same positions, but other positions can be different
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
            let configs = possible_configurations(record);
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
        assert_eq!(part2(""), 0);
    }
}
