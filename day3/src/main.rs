use std::ops::Range;

fn main() {
    let input = include_str!("input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    lines.iter().enumerate().map(|(row, &line)| {
        let mut result = 0;
        for (number, range) in number_with_range(line) {
            if neighbours_of_range(&lines, row, &range, |c| !c.is_digit(10) && c != '.') {
                result += number;
            }
        }
        result
    }).sum()
}

fn number_with_range(input: &str) -> Vec<(i32, Range<usize>)> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut number = 0;
    let mut in_number = false;
    for (i, c) in input.chars().enumerate() {
        if c.is_digit(10) {
            if !in_number {
                start = i;
                in_number = true;
            }
            number = number * 10 + c.to_digit(10).unwrap() as i32;
        } else {
            if in_number {
                in_number = false;
                result.push((number, start..i));
                number = 0;
            }
        }
    }
    if in_number {
        result.push((number, start..input.len()));
    }
    result
}

fn neighbours_of_range<F>(lines: &Vec<&str>, row: usize, range: &Range<usize>, predicate: F) -> bool
    where F: Fn(char) -> bool {

    let mut target_lines = Vec::new();
    if row > 0 {
        target_lines.push(row - 1);
    }
    target_lines.push(row);
    if row < lines.len() - 1 {
        target_lines.push(row + 1);
    }

    let should_check_left = range.start > 0;
    let should_check_right = range.end < lines[row].len();
    let start = if should_check_left { range.start - 1 } else { range.start };
    let end = if should_check_right { range.end + 1 } else { range.end };

    for row in target_lines {
        let line = lines[row];
        let matched = line[start..end].chars().any(&predicate);
        if matched {
            return true;
        }
    }
    return false;
}

fn part2(input: &str) -> i32 {
    let numbers: Vec<Vec<(i32, Range<usize>)>> = input.lines().map(number_with_range).collect();
    input.lines().enumerate().map(|(row, line)| {
        line.chars().enumerate().filter_map(|(col, c)| {
            if c == '*' {
                find_gear_ratio(row, col, &numbers)
            } else {
                None
            }
        }).sum::<i32>()
    }).sum()
}

fn find_gear_ratio(row: usize, col: usize, numbers: &Vec<Vec<(i32, Range<usize>)>>) -> Option<i32> {
    let left = col - 1;
    let right = col + 1;
    let mut target_rows = Vec::new();
    if row > 0 {
        target_rows.push(row - 1);
    }
    target_rows.push(row);
    if row < numbers.len() - 1 {
        target_rows.push(row + 1);
    }
    // dbg!(row, col, &target_rows, left, right);

    let part_numbers: Vec<i32> = target_rows.iter().flat_map(|&row| {
        numbers[row].iter().filter_map(|(number, range)| {
            // dbg!(number, range.start, range.end, range.start <= right && range.end > left);
            if range.start <= right && range.end > left {
                Some(*number)
            } else {
                None
            }
        })
    }).collect();

    if part_numbers.len() == 2 {
        Some(part_numbers[0] * part_numbers[1])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn test_part2() {
        let input =
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part2(input), 467835);
    }
}