use std::iter;
use std::str::from_utf8;

pub fn part1(input: &str) -> usize {
    input.lines()
        .map(Record::from_str)
        .enumerate()
        .map(|(idx, r)| {
            let count = possible_configurations(&"".to_owned(), &r);
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

fn possible_configurations(so_far: &String, record: &Record) -> usize {
    // println!("possible_configurations({}|{})", so_far, record);
    if record.counts.len() == 1 {
        let starts = possible_starts(record.counts[0], true, &record.slots);
        // println!("last starts: {:?}", starts);
        starts.iter().for_each(|&start| {
            println!("{}{}{}{}",
                so_far,
                ".".repeat(start),
                "#".repeat(record.counts[0]),
                ".".repeat(record.slots.len() - start - record.counts[0]),
            );
        });
        return starts.len();
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

    // recurse with each leftmost
    starts.iter().map(|&start| {
        let new_record = Record {
            slots: record.slots[(start+count+1)..].to_vec(),
            counts: record.counts[1..].to_vec(),
        };
        let new_so_far = format!(
            "{}{}{}{}",
            so_far,
            ".".repeat(start),
            "#".repeat(count),
            "."
        );
        possible_configurations(&new_so_far, &new_record)
    }).sum::<usize>()
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
    // println!("start: {}, end: {}", start, end);
    // let slots = &slots[start..end];

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

    // let expanded = iter::once(b'.')
    //     .chain(slots.iter().copied())
    //     .chain(iter::once(b'.'))
    //     .collect::<Vec<_>>();
    println!("expanded: {:?}", from_utf8(&expanded).unwrap());

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

    #[test]
    fn test_line() {
        let lines = TEST_INPUT.lines().collect::<Vec<_>>();
        assert_eq!(possible_configurations(&"".to_owned(), &Record::from_str(lines[0])), 1);
        assert_eq!(possible_configurations(&"".to_owned(), &Record::from_str(lines[1])), 4);
        assert_eq!(possible_configurations(&"".to_owned(), &Record::from_str(lines[2])), 1);
        assert_eq!(possible_configurations(&"".to_owned(), &Record::from_str(lines[3])), 1);
        assert_eq!(possible_configurations(&"".to_owned(), &Record::from_str(lines[4])), 4);
        assert_eq!(possible_configurations(&"".to_owned(), &Record::from_str(lines[5])), 10);
    }

    macro_rules! test_configuration {
        ($name: ident, $line: expr, $answer: expr) => {
            #[test]
            fn $name() {
                let record = &Record::from_str($line);
                println!("{}", record);
                assert_eq!(possible_configurations(&"".to_owned(), record), $answer);
            }
        };
    }
    test_configuration!(line961, "#?????????.#? 2,3,1", 5);
    test_configuration!(line985, "???????##??#?.?#?#?? 1,9,2,1", 5);
    test_configuration!(line985_subcase, ".?#?#?? 2,1", 1);

    test_configuration!(all_5_2, "????? 1,1", 6);
    test_configuration!(all_5_3, "????? 1,1,1", 1);
    test_configuration!(all_7_3, "??????? 1,1,1", 10);
    test_configuration!(line477, "?.????#..??#? 1,1,2,3", 4);

    #[test]
    fn test_possible_start() {
        assert_eq!(possible_starts(1, true, b"???.#?"), vec![4]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(""), 0);
    }
}
