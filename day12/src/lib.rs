use std::iter;
use std::str::from_utf8;

pub fn part1(input: &str) -> usize {
    input.lines()
        .map(Record::from_str)
        .map(possible_configurations)
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

fn possible_configurations(record: Record) -> usize {
    // println!("{}", record);
    if record.counts.len() == 1 {
        return possible_starts(record.counts[0], &record.slots).len();
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
        possible_starts(count, &record.slots[..record.slots.len() - hold])
    } else { vec![] };
    // dbg!(&starts);

    // recurse with each leftmost
    starts.iter().map(|&start| {
        let new_record = Record {
            slots: record.slots[(start+count+1)..].to_vec(),
            counts: record.counts[1..].to_vec(),
        };
        possible_configurations(new_record)
    }).sum::<usize>()
}

fn possible_starts(count: usize, slots: &[u8]) -> Vec<usize> {
    let expanded = iter::once(b'.')
        .chain(slots.iter().copied())
        .chain(iter::once(b'.'))
        .collect::<Vec<_>>();

    expanded.windows(count + 2)
        .enumerate()
        .filter(|(_idx, w)| {
            // println!("idx: {}: {:?}", _idx, from_utf8(w).unwrap());
            if let [before, center@ .., after] = w {
                could_be_empty(before) && could_be_empty(after) && full(center)
            }
            else { false }
        })
        .map(|(idx, _w)| idx)
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
    const LINE_ANSWERS: [usize; 6] = [1, 4, 1, 1, 4, 10];

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 21);
    }

    #[test]
    fn test_line() {
        let lines = TEST_INPUT.lines().collect::<Vec<_>>();
        assert_eq!(possible_configurations(Record::from_str(lines[0])), 1);
        assert_eq!(possible_configurations(Record::from_str(lines[1])), 4);
        assert_eq!(possible_configurations(Record::from_str(lines[2])), 1);
        assert_eq!(possible_configurations(Record::from_str(lines[3])), 1);
        assert_eq!(possible_configurations(Record::from_str(lines[4])), 4);
        assert_eq!(possible_configurations(Record::from_str(lines[5])), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(""), 0);
    }
}
