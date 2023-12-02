use std::iter::Iterator;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));

    println!("{}", part2(input));
}

fn part1(input: &str) -> i32 {
    input.lines().map(|line| {
        to_int(&str_to_int(line))
    }).sum()
}

fn part2(input: &str) -> i32 {
    input.lines().map(|line| {
        let converted = convert_digit_name(line);
        let vec = &str_to_int(&converted);
        print!(" -> {:?}", vec);
        let result = to_int(vec);
        println!(" -> {}", result);
        result
    }).sum()
}

fn to_int(input: &[i32]) -> i32 {
    input[0] * 10 + input[input.len() - 1]
}

fn str_to_int(input: &str) -> Vec<i32> {
    input.chars().filter_map(|c| match c {
        '0'..='9' => Some(c.to_digit(10).unwrap() as i32),
        _ => None
    }).collect()
}
const DIGIT_NAMES_MAP: [(&str, &str); 9] = [("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9"),];

fn convert_digit_name(input: &str) -> String {
    let mut input = input.to_owned();
    print!("{}", input);

    let indexes: Vec<(usize, (&str, &str))> = DIGIT_NAMES_MAP
        .iter().filter_map(|&(name, value)| input.find(name).map(|idx| (idx, (name, value))))
        .collect();

    if let Some((idx, (name, value))) = DIGIT_NAMES_MAP
        .iter().filter_map(|(name, value)| input.find(name).map(|idx| (idx, (name, value))))
        .min_by_key(|(idx, _)| *idx) {
        input.replace_range(idx..=idx, value);
        // input.replace_range(idx..(idx+name.len()), value);
        print!(" -> {:?}", input)
    }
    if let Some((idx, (name, value))) = DIGIT_NAMES_MAP
        .iter().filter_map(|(name, value)| input.find(name).map(|idx| (idx, (name, value))))
        .max_by_key(|(idx, _)| *idx) {
        input.replace_range(idx..=idx, value);
        // input.replace_range(idx..(idx+name.len()), value);
        print!(" -> {:?}", input)
    }

    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input =
"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
fivezg8jmf6hrxnhgxxttwoneg
";
        let expected = vec![29, 83, 13, 24, 42, 14, 76, 51];

        input.lines().map(convert_digit_name).map(|v| to_int(&str_to_int(&v))).zip(expected).for_each(|(actual, expected)| {
            assert_eq!(actual, expected);
        });
    }

    #[test]
    fn test_part2_single() {
        assert_eq!(to_int(&str_to_int(&convert_digit_name("two1nine"))), 29)
    }

    // #[test]
    // fn test_parser() {
    //     let input = "eightwofiveight";
    //
    //     let names = alt(DIGIT_NAMES.map(tag).collect());
    //     let name_and_name0 = tuple((names.clone(), alphanumeric0, names, alphanumeric0));
    //     alt((names, anychar));
    //     fn parsers(s: &str) -> IResult<&str, Vec<&str>> {
    //
    //     }
    //
    //     assert_eq!(parsers(input), Ok(("", vec!["eight", "wofiv", "eight"])));
    // }
}
