use adventofcode2015::build_main;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
enum InputSpec<'a> {
    Literal(u16),
    Wire(&'a str)
}
use InputSpec::*;

#[derive(Copy, Clone, Debug)]
enum Gate<'a> {
    Direct(InputSpec<'a>),
    Not(InputSpec<'a>),
    And(InputSpec<'a>, InputSpec<'a>),
    Or(InputSpec<'a>, InputSpec<'a>),
    LShift(InputSpec<'a>, InputSpec<'a>),
    RShift(InputSpec<'a>, InputSpec<'a>)
}
use Gate::*;

struct Diagram<'a> {
    wires: HashMap<&'a str, Gate<'a>>,
    values: HashMap<&'a str, u16>
}

impl<'a> Diagram<'a> {
    fn eval(&mut self, input_spec: InputSpec<'a>) -> u16 {
        match input_spec {
            Literal(x) => x,
            Wire(w) => {
                if !self.values.contains_key(w) {
                    let result = match self.wires[w] {
                        Direct(u) => self.eval(u),
                        Not(u) => !self.eval(u),
                        And(u1, u2) => self.eval(u1) & self.eval(u2),
                        Or(u1, u2) => self.eval(u1) | self.eval(u2),
                        LShift(u1, u2) => self.eval(u1).checked_shl(self.eval(u2) as u32).unwrap_or(0),
                        RShift(u1, u2) => self.eval(u1).checked_shr(self.eval(u2) as u32).unwrap_or(0),
                    };
                    self.values.insert(w, result);
                }

                self.values[&w]
            }
        }
    }

    fn clear(&mut self) {
        self.values.clear();
    }
}

mod parse {
    use crate::Gate::*;
    use crate::InputSpec::{Literal, Wire};
    use crate::{Diagram, Gate, InputSpec};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alphanumeric1, newline};
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair};
    use nom::IResult;
    use std::collections::HashMap;

    fn input_spec(input: &str) -> IResult<&str, InputSpec> {
        map(
            alphanumeric1,
            |s: &str| {
                match s.parse::<u16>() {
                    Ok(n) => Literal(n),
                    Err(_) => Wire(s)
                }
            }
        )(input)
    }

    fn gate(input: &str) -> IResult<&str, Gate> {
        alt((
            map(preceded(tag("NOT "), input_spec), Not),
            map(separated_pair(input_spec, tag(" AND "), input_spec), |(a, b)| And(a, b)),
            map(separated_pair(input_spec, tag(" OR "), input_spec), |(a, b)| Or(a, b)),
            map(separated_pair(input_spec, tag(" LSHIFT "), input_spec), |(a, b)| LShift(a, b)),
            map(separated_pair(input_spec, tag(" RSHIFT "), input_spec), |(a, b)| RShift(a, b)),
            map(input_spec, Direct)
        ))(input)
    }

    pub fn diagram(input: &str) -> IResult<&str, Diagram> {
        map(
            separated_list1(
                newline,
                map(separated_pair(gate, tag(" -> "), alphanumeric1), |(a, b)| (b, a))
            ),
            |v| {
                let wires: HashMap<&str, Gate> = v.into_iter().collect();
                let values: HashMap<&str, u16> = HashMap::new();
                Diagram { wires, values }
            }
        )(input)
    }
}

fn part1(input: &str) -> u16 {
    let mut diagram = parse::diagram(input).unwrap().1;
    diagram.eval(Wire("a"))
}

fn part2(input: &str) -> u16 {
    let mut diagram = parse::diagram(input).unwrap().1;
    let orig_a = diagram.eval(Wire("a"));

    diagram.clear();
    diagram.wires.insert("b", Direct(Literal(orig_a)));

    diagram.eval(Wire("a"))
}

build_main!("day07.txt", "Part 1" => part1, "Part 2" => part2);