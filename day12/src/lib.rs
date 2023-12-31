// mod memoization;
mod record;

use std::iter;
use std::str::from_utf8;
use itertools::Itertools;
use crate::record::Record;

pub fn part1(input: &str) -> usize {
    input.lines()
        .map(Record::from_str)
        .inspect(|r| println!("inspect {}", r))
        .map(|r| {
            // possible_configurations(r).len()
            possible_configuration_count(r)
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input.lines()
        .map(Record::from_str)
        .inspect(|r| println!("inspect  {}", r))
        .map(Record::unfold)
        .inspect(|r| println!("unfolded {}", r))
        .map(|r| {
            possible_configuration_count(r)
        })
        .inspect(|c| println!("count: {}", c))
        .sum()
}

fn possible_configuration_count(record: Record) -> usize {
    let counts = length_and_counts(record);
    counts.iter().map(|(_len, count)| count).sum()
}

/*
return a list of (required_length, possible_configuration_count) in descending order of required_length
 */
fn length_and_counts(record: Record) -> Vec<(usize, usize)> {
    let result = if record.counts.len() == 1 {
        let starts = possible_ends(record.counts[0], &record.slots);
        let len = record.slots.len();
        let start_count = starts.len();
        starts
            .into_iter()
            .enumerate()
            .map(|(idx, start)| (len - start, 1))
            .collect()
    } else {
        let (first, rest) = record.split();
        let counts = length_and_counts(rest);
        let count = first.counts[0];
        let slot_len = record.slots.len();
        counts.iter().flat_map(|(required_length, config_counts)| {
            possible_ends(count, &first.slots[..slot_len-required_length-1])
                .into_iter()
                .map(|start| (slot_len - start, *config_counts))
        }).collect()
    };
    println!("{record} -> length_and_counts: {:?}", result);
    result
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

fn possible_starts(count: usize, must_consume: bool, slots: &[u8]) -> Vec<usize> {
    let offset = 0;
    let (start, end) = if let Some(first_sharp) = slots.iter().position(|&b| b == b'#') {
        let last_sharp = slots.iter().rposition(|&b| b == b'#').unwrap();
        let start = if must_consume { last_sharp.saturating_sub(count - 1) } else { 0 };
        let end = (first_sharp + count).min(slots.len());
        (start, end)
    } else {
        (0, slots.len())
    };
    // println!("start: {}, end: {}", start, end);

    if end < start {
        return vec![];
    }

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

    let result = expanded.windows(count + 2)
        .enumerate()
        .filter(|(_idx, w)| {
            // println!("idx: {}: {:?}", _idx, from_utf8(w).unwrap());
            if let [before, center@ .., after] = w {
                could_be_empty(before) && could_be_empty(after) && full(center)
            }
            else { false }
        })
        .map(|(idx, _w)| offset + idx + start)
        .collect();
    println!("possible_starts({count}, {must_consume}, {:?}) -> {result:?}", from_utf8(slots).unwrap());
    result
}

fn possible_ends(count: usize, slots: &[u8]) -> Vec<usize> {
    let start = slots.iter().rposition(|&b| b == b'#')
        .map(|last_sharp| last_sharp.saturating_sub(count - 1))
        .unwrap_or(0);

    let expanded =
        iter::once(b'.')
            .chain(slots[start..].iter().copied())
            .chain(iter::once(b'.'))
            .collect::<Vec<_>>()
        ;

    // println!("expanded: {:?}", from_utf8(&expanded).unwrap());

    let result = expanded.windows(count + 2)
        .enumerate()
        .filter(|(_idx, w)| {
            // println!("idx: {}: {:?}", _idx, from_utf8(w).unwrap());
            if let [before, center@ .., after] = w {
                could_be_empty(before) && could_be_empty(after) && full(center)
            }
            else { false }
        })
        .map(|(idx, _w)| idx + start)
        .collect();
    // println!("possible_ends({count}, {:?}) -> {result:?}", from_utf8(slots).unwrap());
    result
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
                assert_eq!(possible_configuration_count(record), $answer);
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

    // #[test]
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

    // #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 525152);
    }

}
