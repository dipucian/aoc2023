use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    let mut galaxies = list_galaxies(input);
    // println!("before");
    // galaxies.iter().for_each(|g| println!("{:?}", g));
    expand_universe(&mut galaxies);
    // println!("after expansion");
    // galaxies.iter().for_each(|g| println!("{:?}", g));

    (0..galaxies.len()).flat_map(|i| {
        let a = &galaxies[i];
        (i+1..galaxies.len()).map(|j| {
            let b = &galaxies[j];
            calculate_distance(a, b)
        })
    }).sum()
}

#[derive(Debug)]
struct Pos {
    row: usize,
    col: usize,
}

fn list_galaxies(input: &str) -> Vec<Pos> {
    let mut galaxies = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Pos { row, col });
            }
        }
    }
    galaxies
}

fn expand_universe(galaxies: &mut Vec<Pos>) {
    let row_expansion = calculate_expansion(galaxies.iter().map(|g| g.row));
    let col_expansion = calculate_expansion(galaxies.iter().map(|g| g.col));

    galaxies.iter_mut().for_each(|g| {
        g.row += row_expansion[g.row];
        g.col += col_expansion[g.col];
    });
}

fn calculate_expansion<I>(positions: I) -> Vec<usize> where I: Iterator<Item=usize> {
    let occupied = positions.collect::<HashSet<_>>();
    let &max = occupied.iter().max().unwrap();
    let mut expansion = Vec::new();
    expansion.resize(max + 1, 0);
    let mut expansion_so_far = 0;
    for i in 1..=max {
        if !occupied.contains(&i) {
            expansion_so_far += 1;
        }
        expansion[i] = expansion_so_far;
    }
    expansion
}

fn calculate_distance(a: &Pos, b: &Pos) -> usize {
    a.row.abs_diff(b.row) + a.col.abs_diff(b.col)
}

pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(""), 0);
    }
}