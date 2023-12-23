use std::collections::HashSet;
use std::ops::DerefMut;
use crate::Direction::*;

pub fn part1(input: &str) -> usize {
    let grid = as_bytes(input);
    let start = find_start(&grid);
    let mut direction = find_connected(&grid, &start);

    let mut pos = start;
    let mut steps = 0;
    loop {
        pos = pos.dir(&direction).unwrap();
        steps += 1;

        let next_tile = at(&grid, &pos).unwrap();
        if next_tile == b'S' {
            break;
        }
        direction = travelling(&direction, next_tile);
    }
    steps / 2
}

fn travelling(towards: &Direction, tile: u8) -> Direction {
    match (tile, towards) {
        (b'|', North) => North,
        (b'|', South) => South,
        (b'-', East) => East,
        (b'-', West) => West,
        (b'7', East) => South,
        (b'7', North) => West,
        (b'J', East) => North,
        (b'J', South) => West,
        (b'L', West) => North,
        (b'L', South) => East,
        (b'F', West) => South,
        (b'F', North) => East,
        _ => unreachable!("Invalid tile {:?} towards {:?}", tile as char, towards)
    }
}

type Grid = Vec<Vec<u8>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn north(&self) -> Pos {
        Pos { row: self.row - 1, col: self.col }
    }
    fn east(&self) -> Pos {
        Pos { row: self.row, col: self.col + 1 }
    }

    fn south(&self) -> Pos {
        Pos { row: self.row + 1, col: self.col }
    }

    fn west(&self) -> Pos {
        Pos { row: self.row, col: self.col - 1 }
    }

    fn dir(&self, dir: &Direction) -> Option<Pos> {
        match dir {
            North => if self.row > 0 { Some(self.north()) } else { None },
            East => Some(self.east()),
            South => Some(self.south()),
            West => if self.col > 0 { Some(self.west()) } else { None },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn as_bytes(input: &str) -> Grid {
    input.lines()
        .map(|s| s.as_bytes().to_vec())
        .collect::<Vec<_>>()
}

fn at(grid: &Grid, pos: &Pos) -> Option<u8> {
    if pos.row < grid.len() && pos.col < grid[pos.row].len() {
        Some(grid[pos.row][pos.col])
    } else {
        None
    }
}

fn find_start(arr: &Grid) -> Pos {
    arr.iter().enumerate()
        .filter_map(|(row, line)|
            line.iter()
                .position(|&c| c == b'S')
                .map(|col| Pos { row, col })
        ).collect::<Vec<_>>()[0]
}

fn find_connected(grid: &Grid, pos: &Pos) -> Direction {
    [
        (North, [b'7', b'F', b'|']),
        (East, [b'J', b'7', b'-']),
        (South, [b'J', b'L', b'|']),
        (West, [b'L', b'F', b'-']),
    ].into_iter()
        .find(|(dir, targets)| {
            pos.dir(dir).map(|cell| {
                at(grid, &cell).filter(|c| targets.contains(c)).is_some()
            }).unwrap_or(false)
        }).unwrap().0
}

const EMPTY: u8 = b'.';

pub fn part2(input: &str) -> i32 {
    let mut grid = as_bytes(input);
    let start = find_start(&grid);
    let mut direction = find_connected(&grid, &start);

    let mut loop_pos: HashSet<Pos> = HashSet::new();

    let mut pos = start;
    loop {
        pos = pos.dir(&direction).unwrap();

        let next_tile = at(&grid, &pos).unwrap();
        loop_pos.insert(pos);
        if next_tile == b'S' {
            break;
        }
        direction = travelling(&direction, next_tile);
    }
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let pos = Pos {row, col};
            if !loop_pos.contains(&pos) {
                mark_pos(&mut grid, &pos, EMPTY);
            }
        }
    }

    // print the grid
    grid.iter().for_each(|line| {
        println!("{}", line.iter().map(|&c| c as char).collect::<String>());
    });

    grid.iter().map(|line| {
        parsing::parse_line(line) as i32
    }).sum()
}

fn mark_pos(grid: &mut Grid, pos: &Pos, target: u8) {
    grid.deref_mut()[pos.row].deref_mut()[pos.col] = target;
}

mod parsing {
    use std::str::from_utf8;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::IResult;
    use nom::multi::{fold_many1, many0, many1_count};
    use nom::sequence::separated_pair;

    enum Section {
        Wall,
        Area(usize),
        WallEnd,
    }
    struct State {
        inside: bool,
        count: usize,
    }

    pub fn parse_line(line: &[u8]) -> u8 {
        fn south_to_north(input: &str) -> IResult<&str, Section> {
            separated_pair(tag("F"), many0(tag("-")), tag("J"))(input)
                .map(|(remain, _)| (remain, Section::Wall))
        }
        fn north_to_south(input: &str) -> IResult<&str, Section> {
            separated_pair(tag("L"), many0(tag("-")), tag("7"))(input)
                .map(|(remain, _)| (remain, Section::Wall))
        }
        fn straight(input: &str) -> IResult<&str, Section> {
            tag("|")(input).map(|(remain, _)| (remain, Section::Wall))
        }
        fn north_end(input: &str) -> IResult<&str, Section> {
            separated_pair(tag("L"), many0(tag("-")), tag("J"))(input)
                .map(|(remain, _)| (remain, Section::WallEnd))
        }
        fn south_end(input: &str) -> IResult<&str, Section> {
            separated_pair(tag("F"), many0(tag("-")), tag("7"))(input)
                .map(|(remain, _)| (remain, Section::WallEnd))
        }
        fn area(input: &str) -> IResult<&str, Section> {
            many1_count(tag("."))(input)
                .map(|(remain, count)| (remain, Section::Area(count)))
        }
        fn sections(input: &str) -> IResult<&str, Section> {
            alt((south_to_north, north_to_south, straight, north_end, south_end, area))(input)
        }
        fold_many1(
            sections,
            || State { inside: false, count: 0 },
            |mut acc, section| {
                match section {
                    Section::Wall => acc.inside = !acc.inside,
                    Section::WallEnd => { /*do nothing*/ }
                    Section::Area(count) =>
                        if acc.inside { acc.count += count }
                }
                acc
            }
        )(from_utf8(line).unwrap()).unwrap().1.count as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    const TEST_INPUT_2: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[test]
    fn test_find_first() {
        let arr = as_bytes(TEST_INPUT);
        assert_eq!(find_start(&arr), Pos { row: 2, col: 0 })
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), 10);
    }
}