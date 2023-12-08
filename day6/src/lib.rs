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

const REAL_TESTCASE: Race = Race { time_limit: 71530, record: 940200 };
pub const REAL_INPUT: Race = Race { time_limit: 53837288, record: 333163512891532 };

pub fn part1(races: &[Race]) -> i64 {
    races.iter()
        .map(|race| race.ways_to_beat_record())
        .reduce(|a, b| a * b)
        .unwrap()
}

pub fn part2(race: &Race) -> i64 {
    race.ways_to_beat_record()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TESTCASE), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&REAL_TESTCASE), 71503);
    }
}