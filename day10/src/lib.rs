use std::str::Chars;
use crate::Direction::*;

pub fn part1(input: &str) -> usize {
    let grid = as_bytes(input);
    let start = find_start(&grid);
    let mut direction = find_connected(&grid, &start);

    let mut pos = start;
    let mut steps = 0;
    loop {
        let from = pos;
        pos = pos.dir(&direction);
        steps += 1;

        let next_tile = at(&grid, &pos).unwrap();
        // println!("{}: {:?} -- {:?} -> {:?} @ {:?}", steps, from, direction, next_tile as char, pos);
        if next_tile == b'S' {
            break;
        }
        direction = travelling(&direction, next_tile);
    }
    // dbg!(steps);
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

type Grid<'a> = Vec<&'a [u8]>;

#[derive(Debug, Copy, Clone, PartialEq)]
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

    fn dir(&self, dir: &Direction) -> Pos {
        match dir {
            Direction::North => self.north(),
            Direction::East => self.east(),
            Direction::South => self.south(),
            Direction::West => self.west(),
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
        .map(|s| s.as_bytes())
        .collect::<Vec<_>>()
}

fn at(grid: &Grid, pos: &Pos) -> Option<u8> {
    return if pos.row >= 0 && pos.row < grid.len() && pos.col >= 0 && pos.col < grid[pos.row].len() {
        Some(grid[pos.row][pos.col])
    } else {
        None
    }
}

fn find_start(arr: &Grid) -> Pos {
    arr.iter().enumerate()
        .filter_map(|(row, &line)|
            line.iter()
                .position(|&c| c == b'S')
                .map(|col| Pos { row, col })
        ).collect::<Vec<_>>()[0]
}

fn find_connected(grid: &Grid, pos: &Pos) -> Direction {
    // North => 7, F, |
    // East => J, 7, -
    // South => J, L, |
    // West => L, F, -
    [
        (North, [b'7', b'F', b'|']),
        (East, [b'J', b'7', b'-']),
        (South, [b'J', b'L', b'|']),
        (West, [b'L', b'F', b'-']),
    ].into_iter()
        .find(|(dir, targets)| {
            let cell = pos.dir(dir);
            at(grid, &cell).filter(|c| targets.contains(c)).is_some()
        }).unwrap().0
}

pub fn part2(input: &str) -> usize {
    0
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
        assert_eq!(part2(""), 0);
    }
}