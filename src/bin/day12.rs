use adventofcode2015::build_main;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
enum Json {
    JNumber(isize),
    JString(String),
    JArray(Vec<Json>),
    JObject(HashMap<String, Json>)
}
use Json::*;

impl Json {
    fn sum(&self) -> isize {
        match self {
            JNumber(n) => *n,
            JString(_) => 0,
            JArray(elems) => elems.iter().map(|j| j.sum()).sum(),
            JObject(objs) => objs.values().map(|j| j.sum()).sum()
        }
    }

    fn sum_no_red(&self) -> isize {
        match self {
            JNumber(n) => *n,
            JString(_) => 0,
            JArray(elems) => elems.iter().map(|j| j.sum_no_red()).sum(),
            JObject(objs) => {
                if objs.values().contains(&JString("red".to_owned())) {
                    0
                }
                else {
                    objs.values().map(|j| j.sum_no_red()).sum()
                }
            }
        }
    }
}

mod parse {
    use crate::Json;
    use crate::Json::*;
    use nom::branch::alt;
    use nom::bytes::complete::is_not;
    use nom::character::complete::{char, digit1, multispace0};
    use nom::combinator::{map, map_res, opt};
    use nom::multi::separated_list1;
    use nom::sequence::{delimited, pair, separated_pair, tuple};
    use nom::IResult;

    fn number(input: &str) -> IResult<&str, isize> {
        map(
            tuple((
                map(opt(char('-')), |x| if x.is_some() { -1isize } else { 1isize }),
                map_res(digit1, |s: &str| s.parse::<isize>())
            )),
            |(signum, value)| signum * value
        )(input)
    }

    fn jnumber(input: &str) -> IResult<&str, Json> {
        map(number, JNumber)(input)
    }

    fn string(input: &str) -> IResult<&str, String> {
        // The input doesn't require any fancy worries about escaping, so... forget that.
        map(
            delimited(char('"'), is_not("\""), char('"')),
            |s: &str| s.to_owned()
        )(input)
    }

    fn jstring(input: &str) -> IResult<&str, Json> {
        map(string, JString)(input)
    }

    fn jarray(input: &str) -> IResult<&str, Json> {
        map(
            delimited(
                pair(char('['), multispace0),
                separated_list1(
                    tuple((multispace0, char(','), multispace0)),
                    json
                ),
                pair(multispace0, char(']'))
            ),
            JArray
        )(input)
    }

    fn jobject_item(input: &str) -> IResult<&str, (String, Json)> {
        separated_pair(
            string,
            tuple((multispace0, char(':'), multispace0)),
            json
        )(input)
    }

    fn jobject(input: &str) -> IResult<&str, Json> {
        map(
            delimited(
                pair(char('{'), multispace0),
                separated_list1(
                    tuple((multispace0, char(','), multispace0)),
                    jobject_item
                ),
                pair(multispace0, char('}'))
            ),
            |kvs| JObject(kvs.into_iter().collect())
        )(input)
    }

    pub fn json(input: &str) -> IResult<&str, Json> {
        alt((
            jstring,
            jnumber,
            jobject,
            jarray
        ))(input)
    }
}

fn part1(input: &str) -> isize {
    let parsed = parse::json(input.trim()).unwrap().1;
    parsed.sum()
}

fn part2(input: &str) -> isize {
    let parsed = parse::json(input.trim()).unwrap().1;
    parsed.sum_no_red()
}

build_main!("day12.txt", "Part 1" => part1, "Part 2" => part2);