fn main() {
    let input = include_str!("input.txt");

    let almanac = parsing::parse_almanac(input).unwrap().1;

    println!("part1: {}", part1(&almanac));
    println!("part2: {}", part2(&almanac));
}

#[derive(Debug, PartialEq)]
pub struct Almanac {
    seeds: Vec<i64>,
    mappings: Vec<Mapping>
}

impl Almanac {
    fn chain_lookup(&self, from: &str, value: i64, to: &str) -> i64 {
        let mut current_value = value;
        let mut current_type = from;
        while current_type != to {
            let mapping = self.mappings.iter().find(|mapping| mapping.from == current_type).unwrap();
            current_value = mapping.apply(current_value);
            current_type = &mapping.to;
        }
        current_value
    }
}

#[derive(Debug, PartialEq)]
pub struct Mapping {
    from: String,
    to: String,
    sections: Vec<Section>
}

impl Mapping {
    fn apply(&self, value: i64) -> i64 {
        for section in &self.sections {
            if value >= section.source_start && value < section.source_start + section.size {
                return section.destination_start + (value - section.source_start)
            }
        }
        return value
    }
}

#[derive(Debug, PartialEq)]
pub struct Section {
    destination_start: i64,
    source_start: i64,
    size: i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("testcase1.txt");
        let almanac = parsing::parse_almanac(input).unwrap().1;
        assert_eq!(part1(&almanac), 35);
    }

    #[test]
    fn test_mapping() {
        let input = include_str!("testcase1.txt");
        let almanac = parsing::parse_almanac(input).unwrap().1;
        let seed_to_soil = almanac.mappings.iter().find(|mapping| mapping.from == "seed").unwrap();

        assert_eq!(seed_to_soil.apply(79), 81);
        assert_eq!(seed_to_soil.apply(14), 14);
        assert_eq!(seed_to_soil.apply(55), 57);
        assert_eq!(seed_to_soil.apply(13), 13);

        assert_eq!(seed_to_soil.apply(0), 0);
        assert_eq!(seed_to_soil.apply(50), 52);
        assert_eq!(seed_to_soil.apply(51), 53);
        assert_eq!(seed_to_soil.apply(96), 98);
        assert_eq!(seed_to_soil.apply(98), 50);
        assert_eq!(seed_to_soil.apply(99), 51);
    }

    #[test]
    fn test_chain_lookup() {
        let input = include_str!("testcase1.txt");
        let almanac = parsing::parse_almanac(input).unwrap().1;

        assert_eq!(almanac.chain_lookup("seed", 79, "location"), 82);
        assert_eq!(almanac.chain_lookup("seed", 14, "location"), 43);
        assert_eq!(almanac.chain_lookup("seed", 55, "location"), 86);
        assert_eq!(almanac.chain_lookup("seed", 13, "location"), 35);
    }
}

fn part1(almanac: &Almanac) -> i64 {
    almanac
        .seeds.iter()
        .map(|&seed| almanac.chain_lookup("seed", seed, "location"))
        .min().unwrap()
}

fn part2(_almanac: &Almanac) -> i64 {
    0
}

mod parsing {
    use super::*;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, space1, i64};
    use nom::IResult;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair, terminated};

    fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
        preceded(tag("seeds: "), separated_list1(tag(" "), i64))(input)
    }

    fn parse_section(input: &str) -> IResult<&str, Section> {
        separated_list1(space1, i64)(input)
            .map(|(remaining, section)| (remaining, Section { destination_start: section[0], source_start: section[1], size: section[2] }))
    }

    fn parse_map_heading(input: &str) -> IResult<&str, (&str, &str)> {
        terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:"))(input)
    }

    fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
        separated_pair(parse_map_heading, tag("\n"), separated_list1(tag("\n"), parse_section))(input)
            .map(|(remaining, (heading, sections))| (remaining, Mapping { from: heading.0.to_string(), to: heading.1.to_string(), sections }))
    }

    fn parse_mappings(input: &str) -> IResult<&str, Vec<Mapping>> {
        separated_list1(tag("\n\n"), parse_mapping)(input)
    }

    pub(crate) fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
        separated_pair(parse_seeds, tag("\n\n"), parse_mappings)(input)
            .map(|(remaining, (seeds, mappings))| (remaining, Almanac { seeds, mappings }))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_seeds() {
            let input = "seeds: 1 2 3 4 5 6 7 8 9 10";
            assert_eq!(parse_seeds(input), Ok(("", vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));
        }

        #[test]
        fn test_parse_section() {
            let input = "0 1 2";
            assert_eq!(parse_section(input), Ok(("", Section { destination_start: 0, source_start: 1, size: 2 })));
        }

        #[test]
        fn test_parse_mapping() {
            let input = "A-to-B map:\n50 98 2\n52 50 48";
            assert_eq!(parse_mapping(input), Ok(("", Mapping {
                from: "A".to_string(),
                to: "B".to_string(),
                sections: vec![
                    Section { destination_start: 50, source_start: 98, size: 2 },
                    Section { destination_start: 52, source_start: 50, size: 48 },
                ]
            })));
        }
    }
}