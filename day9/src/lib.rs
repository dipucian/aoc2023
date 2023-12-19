
pub fn part1(input: &str) -> i32 {
    input.lines().map(|line| {
        let series: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        extrapolate(&series)
    }).sum()
}

fn extrapolate(series: &[i32]) -> i32 {
    if series.iter().all(|&n| n == series[0]) {
        return series[0];
    }

    let diffs: Vec<i32> = series.windows(2).map(|w| w[1] - w[0]).collect();
    extrapolate(&diffs) + series[series.len() - 1]
}

pub fn part2(input: &str) -> i32 {
    input.lines().map(|line| {
        let series: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        extrapolate_backwards(&series)
    }).sum()
}

fn extrapolate_backwards(series: &[i32]) -> i32 {
    if series.iter().all(|&n| n == series[0]) {
        return series[0];
    }

    let diffs: Vec<i32> = series.windows(2).map(|w| w[1] - w[0]).collect();
    series[0] - extrapolate_backwards(&diffs)
}

#[cfg(test)]
mod tests {
    use super::*;

const TEST_INPUT: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
const TEST_NEXT_TERMS: [i32; 3] = [18, 28, 68];

    #[test]
    fn test_extrapolate() {
        TEST_INPUT.lines().zip(TEST_NEXT_TERMS.iter()).for_each(|(line, next_term)| {
            let series: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            assert_eq!(extrapolate(&series), *next_term);
        });
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2);
    }
}