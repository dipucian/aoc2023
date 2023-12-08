
pub struct Race {
    pub time_limit: i64,
    pub record: i64,
}

impl Race {
    pub fn ways_to_beat_record(&self) -> i64 {
        let mut ways = 0;
        for charge_time in 1..self.time_limit {
            if Self::distance(self.time_limit, charge_time) > self.record {
                ways += 1;
            }
        }
        ways
    }

    pub fn distance(time_limit: i64, charge_time: i64) -> i64 {
        (time_limit - charge_time) * charge_time
    }
}

#[cfg(test)]
mod test {
    use crate::TESTCASE;
    use super::*;

    #[test]
    fn test_ways_to_beat_record() {
        let expected = [4, 8, 9];
        TESTCASE.iter().zip(expected).for_each(|(race, answer)| {
            assert_eq!(race.ways_to_beat_record(), answer);
        })
    }
}