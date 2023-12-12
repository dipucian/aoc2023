use crate::{Node, Step};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending};
use nom::combinator::value;
use nom::multi::{many1, separated_list1};
use nom::{IResult, Parser};
use nom::sequence::{delimited, separated_pair};

fn parse_node(input: &str) -> IResult<&str, Node> {
    separated_pair(
        alpha1, tag(" = "),
        delimited(
            tag("("), separated_pair(alpha1, tag(", "), alpha1), tag(")"),
        ),
    )(input).map(|(remain, (label, (x, y)))|
        (remain,
         Node {
             label: label.to_string(),
             left: x.to_string(),
             right: y.to_string(),
         })
    )
}

pub fn parse_file(input: &str) -> (Vec<Step>, Vec<Node>) {
    let left_or_right = alt((
        value(Step::Left, tag("L")), value(Step::Right, tag("R"))
    ));
    let steps = many1(left_or_right);
    let nodes = separated_list1(line_ending, parse_node);

    separated_pair(steps, many1(line_ending), nodes).parse(input).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_node() {
        assert_eq!(parse_node("AAA = (BBB, BBB)").unwrap().1,
                   Node { label: "AAA".to_string(), left: "BBB".to_string(), right: "BBB".to_string() });
    }

}
