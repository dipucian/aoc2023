use crate::model::Race;

mod model;

const TESTCASE: [Race; 3] = [
    Race { time_limit: 7, record: 9 },
    Race { time_limit: 15, record: 40 },
    Race { time_limit: 30, record: 200 },
];
pub const INPUT: [Race; 4] = [
    Race { time_limit: 53, record: 333 },
    Race { time_limit: 83, record: 1635 },
    Race { time_limit: 72, record: 1289 },
    Race { time_limit: 88, record: 1532 },
];

pub fn part1(races: &[Race]) -> i64 {
    races.iter()
        .map(|race| race.ways_to_beat_record())
        .reduce(|a, b| a * b)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTCASE), 288);
    }
}