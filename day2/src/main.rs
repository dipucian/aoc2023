use nom::Parser;

#[derive(Debug, PartialEq)]
#[derive(Clone)]
pub struct Game {
    id: i32,
    draws: Vec<Vec<(i32, Color)>>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Color {
    Red,
    Blue,
    Green,
}

mod parsing {
    use super::*;

    use nom::branch::alt;
    use nom::character::complete::i32;
    use nom::bytes::complete::tag;
    use nom::combinator::value;
    use nom::{IResult, Parser};
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair};

    fn parse_game_id(input: &str) -> IResult<&str, i32> {
        preceded(tag("Game "), i32)(input)
    }

    fn parse_color(input: &str) -> IResult<&str, Color> {
        alt((value(Color::Red, tag("red")), value(Color::Blue, tag("blue")), value(Color::Green, tag("green"))))(input)
    }

    fn parse_color_count(input: &str) -> IResult<&str, (i32, Color)> {
        separated_pair(i32, tag(" "), parse_color)(input)
    }

    fn parse_draw(input: &str) -> IResult<&str, Vec<(i32, Color)>> {
        separated_list1(tag(", "), parse_color_count)(input)
    }

    fn parse_draws(input: &str) -> IResult<&str, Vec<Vec<(i32, Color)>>> {
        separated_list1(tag("; "), parse_draw)(input)
    }

    pub fn parse_game(input: &str) -> IResult<&str, Game> {
        separated_pair(parse_game_id, tag(": "), parse_draws)(input)
            .map(|(remaining, (id, draws))| (remaining, Game { id, draws }))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_game() {
            let input = "Game 1: 1 red, 3 blue, 11 green; 1 blue, 5 red; 3 blue, 5 green, 13 red; 6 red, 1 blue, 4 green; 16 red, 12 green";
            assert_eq!(parse_game(input), Ok(("", Game { id: 1, draws: vec![
                vec![(1, Color::Red), (3, Color::Blue), (11, Color::Green)],
                vec![(1, Color::Blue), (5, Color::Red)],
                vec![(3, Color::Blue), (5, Color::Green), (13, Color::Red)],
                vec![(6, Color::Red), (1, Color::Blue), (4, Color::Green)],
                vec![(16, Color::Red), (12, Color::Green)],
            ] })));
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    fn parse_line(input: &str) -> Game {
        parsing::parse_game(input).unwrap().1
    }

    let games = input.lines().map(parse_line).collect::<Vec<_>>();

    println!("part1: {}", part1(games.clone()));
    println!("part2: {}", part2(games));
}

fn part1(games: Vec<Game>) -> i32 {
    // maximum 12 red cubes, 13 green cubes, and 14 blue cubes
    games.iter().filter_map(|game| {
        let red = game.draws.iter().flatten().filter_map(|&(count, color)| if color == Color::Red { Some(count) } else { None }).max().unwrap_or(0);
        let green = game.draws.iter().flatten().filter_map(|&(count, color)| if color == Color::Green { Some(count) } else { None }).max().unwrap_or(0);
        let blue = game.draws.iter().flatten().filter_map(|&(count, color)| if color == Color::Blue { Some(count) } else { None }).max().unwrap_or(0);
        if red <= 12 && green <= 13 && blue <= 14 {
            Some(game.id)
        } else {
            None
        }
    }).sum::<i32>()
}

fn part2(games: Vec<Game>) -> i32 {
    games.iter().map(|game| {
        let red = game.draws.iter().flatten().filter_map(|&(count, color)| if color == Color::Red { Some(count) } else { None }).max().unwrap_or(0);
        let green = game.draws.iter().flatten().filter_map(|&(count, color)| if color == Color::Green { Some(count) } else { None }).max().unwrap_or(0);
        let blue = game.draws.iter().flatten().filter_map(|&(count, color)| if color == Color::Blue { Some(count) } else { None }).max().unwrap_or(0);
        red * green * blue
    }).sum::<i32>()
}
