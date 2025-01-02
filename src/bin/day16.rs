use std::collections::HashMap;
use itertools::Itertools;
use adventofcode2015::build_main;

type Sue<'a> = HashMap<&'a str, usize>;

mod parse {
    use crate::Sue;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, digit1, newline};
    use nom::combinator::{map, map_res};
    use nom::IResult;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair, tuple};

    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn property(input: &str) -> IResult<&str, (&str, usize)> {
        separated_pair(alpha1, tag(": "), number)(input)
    }

    fn sue(input: &str) -> IResult<&str, Sue> {
        map(
            preceded(
                tuple((tag("Sue "), number, tag(": "))),
                separated_list1(tag(", "), property)
            ),
            |props| props.into_iter().collect()
        )(input)
    }

    pub fn sues(input: &str) -> IResult<&str, Vec<Sue>> {
        separated_list1(newline, sue)(input)
    }
}

fn matches_exact(poss: &Sue) -> bool {
    poss.iter()
        .all(|(&k, &v)| {
            v == match k {
                "children" => 3,
                "cats" =>  7,
                "samoyeds" =>  2,
                "pomeranians" => 3,
                "akitas" => 0,
                "vizslas" => 0,
                "goldfish" => 5,
                "trees" => 3,
                "cars" => 2,
                "perfumes" => 1,
                _ => unreachable!()
            }
        })
}

fn matches_range(poss: &Sue) -> bool {
    poss.iter()
        .all(|(&k, &v)| {
            match k {
                "children" => v == 3,
                "cats" =>  v > 7,
                "samoyeds" =>  v == 2,
                "pomeranians" => v < 3,
                "akitas" => v == 0,
                "vizslas" => v == 0,
                "goldfish" => v < 5,
                "trees" => v > 3,
                "cars" => v == 2,
                "perfumes" => v == 1,
                _ => unreachable!()
            }
        })
}

fn part1(input: &str) -> usize {
    let sues = parse::sues(input).unwrap().1;
    sues.iter().find_position(|&sue| matches_exact(sue)).unwrap().0 + 1
}

fn part2(input: &str) -> usize {
    let sues = parse::sues(input).unwrap().1;
    sues.iter().find_position(|&sue| matches_range(sue)).unwrap().0 + 1
}

build_main!("day16.txt", "Part 1" => part1, "Part 2" => part2);