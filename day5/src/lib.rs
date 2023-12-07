use std::cmp::min;

pub fn part1(almanac: &Almanac) -> i64 {
    almanac
        .seeds.iter()
        .map(|&seed| almanac.chain_lookup("seed", seed, "location"))
        .min().unwrap()
}

pub fn part2_full(almanac: &Almanac) -> i64 {
    part2(almanac, i64::MAX)
}

pub fn part2(almanac: &Almanac, limit: i64) -> i64 {
    almanac
        .seeds
        .windows(2).step_by(2)
        .filter_map(|window| {
            let [start, count] = *window else { unreachable!() };
            let size = min(count, limit);
            (start..(start+size)).map(|idx| {
                almanac.chain_lookup("seed", idx, "location")
            }).min()
        })
        .min().unwrap()
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

    #[test]
    fn test_window() {
        let arr = [1, 2, 3, 4];
        assert_eq!(arr.windows(2).step_by(2).collect::<Vec<_>>(), vec![&[1, 2], &[3, 4]]);
    }

    fn assert_equivalent(mapping: &Mapping, mappings: &Vec<Mapping>, from: &str, to: &str) {
        (0..100).for_each(|idx|
            assert_eq!(mapping.apply(idx), Almanac::chain_lookup_with(mappings, from, idx, to), "idx={}", idx)
        );
    }

    #[test]
    fn test_mapping_simple() {
        let a_to_b = Mapping {
            from: "A".to_string(),
            to: "B".to_string(),
            sections: vec![
                Section { destination_start: 10, source_start: 0, size: 10 },
                Section { destination_start: 0, source_start: 10, size: 10 },
            ]
        };
        let identity = Mapping {
            from: "B".to_string(),
            to: "C".to_string(),
            sections: vec![]
        };

        assert_equivalent(&a_to_b.combine(&identity), &vec![a_to_b, identity], "A", "C");
    }

    #[test]
    fn test_mapping_combine() {
        let input = include_str!("testcase1.txt");
        let almanac = parsing::parse_almanac(input).unwrap().1;
        let seed_to_soil = almanac.mappings.iter().find(|mapping| mapping.from == "seed").unwrap();
        let soil_to_fertilizer = almanac.mappings.iter().find(|mapping| mapping.from == "soil").unwrap();
        /*
        seed-to-soil map:
        0 0 50          // 0 -> 0, 1 -> 1, 2 -> 2 ... 49 -> 49
        52 50 48        // 50 -> 52, 51 -> 53, 52 -> 54 ... 96 -> 98, 97 -> 99
        50 98 2         // 98 -> 50, 99 -> 51

        soil-to-fertilizer map:
        39 0 15         // 0 -> 39, 1 -> 40, 2 -> 41 ... 14 -> 53
        0 15 37         // 15 -> 0, 16 -> 1, 17 -> 2 ... 51 -> 36
        37 52 2         // 52 -> 37, 53 -> 38

        seed-to-fertilizer map:
        39 0 15         // 0 -> 39, 1 -> 40, 2 -> 41 ... 14 -> 53
        0 15 35         // 15 -> 0, 16 -> 1, 17 -> 2 ... 49 -> 34
        37 50 2         // 50 -> 37, 51 -> 38
        52 52 48        // 52 -> 52, 53 -> 53, 54 -> 54 ... 99 -> 99
         */

        let combined = seed_to_soil.combine(soil_to_fertilizer);

        assert_eq!(combined.from, "seed");
        assert_eq!(combined.to, "fertilizer");

        assert_equivalent(&combined, &almanac.mappings, "seed", "fertilizer")
    }
}

#[derive(Debug, PartialEq)]
pub struct Almanac {
    pub seeds: Vec<i64>,
    pub mappings: Vec<Mapping>
}

impl Almanac {

    pub fn chain_lookup_with(mappings: &Vec<Mapping>, from: &str, value: i64, to: &str) -> i64 {
        let mut current_value = value;
        let mut current_type = from;
        while current_type != to {
            let mapping = mappings.iter().find(|mapping| mapping.from == current_type).unwrap();
            current_value = mapping.apply(current_value);
            current_type = &mapping.to;
        }
        current_value
    }
    pub fn chain_lookup(&self, from: &str, value: i64, to: &str) -> i64 {
        Almanac::chain_lookup_with(&self.mappings, from, value, to)
    }
}

#[derive(Debug, PartialEq)]
pub struct Mapping {
    pub from: String,
    to: String,
    sections: Vec<Section>
}

impl Mapping {

    fn find_section(&self, value: i64) -> Option<&Section> {
        for section in &self.sections {
            if value >= section.source_start && value < section.source_start + section.size {
                return Some(section)
            }
        }
        return None
    }

    pub fn apply(&self, value: i64) -> i64 {
        if let Some(section) = self.find_section(value) {
            return section.apply(value)
        }
        return value
    }
    pub fn combine(&self, other: &Mapping) -> Mapping {
        assert_eq!(self.to, other.from);

        let mut sections: Vec<Section> = Vec::new();
        let mut current = 0_i64;

        fn find_next_start(sections: &Vec<Section>, value: i64) -> i64 {
            sections.iter()
                .find(|section| section.source_start >= value)
                .map(|section| section.source_start)
                .unwrap_or(i64::MAX)
        }
        fn mapped_and_remain(mapping: &Mapping, value: i64) -> (i64, i64) {
            if let Some(section) = mapping.find_section(value) {
                (section.apply(value), section.size_from(value))
            } else {
                let next_start = find_next_start(&mapping.sections, value);
                (value, next_start - value)
            }
        }

        while current < i64::MAX {
            let (b, a_remain) = mapped_and_remain(self, current);
            let (c, b_remain) = mapped_and_remain(other, b);
            let size = min(a_remain, b_remain);
            let new_section = Section { destination_start: c, source_start: current, size };
            sections.push(new_section);
            current += size;
        }

        Mapping {
            from: self.from.clone(),
            to: other.to.clone(),
            sections: sections
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Section {
    destination_start: i64,
    source_start: i64,
    size: i64
}

impl Section {
    fn apply(&self, value: i64) -> i64 {
        self.destination_start + (value - self.source_start)
    }

    pub fn size_from(&self, value: i64) -> i64 {
        self.size - (value - self.source_start)
    }
}

pub mod parsing {
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

    pub fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
        separated_pair(parse_seeds, tag("\n\n"), parse_mappings)(input)
            .map(|(remaining, (seeds, mut mappings))| {
                mappings.iter_mut().for_each(|mapping| mapping.sections.sort_by_key(|section| section.source_start));
                (remaining, Almanac { seeds, mappings })
            })
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
