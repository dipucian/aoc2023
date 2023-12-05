fn main() {
    let input = include_str!("input.txt");

    let almanac = parsing::parse_almanac(input).unwrap().1;

    dbg!(almanac);

    // println!("part1: {}", part1(data));
    // println!("part2: {}", part2(data));
}

#[derive(Debug, PartialEq)]
pub struct Almanac {
    seeds: Vec<u32>,
    mappings: Vec<Mapping>
}

#[derive(Debug, PartialEq)]
pub struct Mapping {
    from: String,
    to: String,
    sections: Vec<Section>
}

#[derive(Debug, PartialEq)]
pub struct Section {
    destination_start: u32,
    source_start: u32,
    size: u32
}

mod parsing {
    use super::*;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, space1, u32};
    use nom::IResult;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair, terminated};

    fn parse_seeds(input: &str) -> IResult<&str, Vec<u32>> {
        preceded(tag("seeds: "), separated_list1(tag(" "), u32))(input)
    }

    fn parse_section(input: &str) -> IResult<&str, Section> {
        separated_list1(space1, u32)(input)
            .map(|(remaining, section)| (remaining, Section { source_start: section[0], destination_start: section[1], size: section[2] }))
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
            let input = "0 0 1";
            assert_eq!(parse_section(input), Ok(("", Section { destination_start: 0, source_start: 0, size: 1 })));
        }

        #[test]
        fn test_parse_mapping() {
            let input = "A-to-B map:\n0 0 1\n1 1 2";
            assert_eq!(parse_mapping(input), Ok(("", Mapping {
                from: "A".to_string(),
                to: "B".to_string(),
                sections: vec![
                    Section { destination_start: 0, source_start: 0, size: 1 },
                    Section { destination_start: 1, source_start: 1, size: 2 },
                ]
            })));
        }
    }
}