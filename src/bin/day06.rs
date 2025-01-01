use std::cmp::{max, min};
use itertools::Itertools;
use adventofcode2015::build_main;

#[derive(Copy, Clone)]
enum Operation { TurnOff, Toggle, TurnOn }
use Operation::*;

type Pos = (usize, usize);

struct Instruction {
    operation: Operation,
    from: Pos,
    to: Pos
}

mod parse {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, digit1, newline, space1};
    use nom::combinator::{map, map_res, value};
    use nom::IResult;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair, tuple};
    use crate::{Instruction, Operation, Pos};
    use crate::Operation::{Toggle, TurnOff, TurnOn};

    fn operation(input: &str) -> IResult<&str, Operation> {
        alt((
            value(TurnOff, tag("turn off")),
            value(TurnOn, tag("turn on")),
            value(Toggle, tag("toggle"))
        ))(input)
    }

    fn pos(input: &str) -> IResult<&str, Pos> {
        separated_pair(
            map_res(digit1, str::parse::<usize>),
            char(','),
            map_res(digit1, str::parse::<usize>)
        )(input)
    }

    fn instruction(input: &str) -> IResult<&str, Instruction> {
        map(
            tuple((
                operation,
                preceded(space1, pos),
                preceded(tag(" through "), pos)
            )),
            |(op, from, to)| Instruction { operation: op, from, to}
        )(input)
    }

    pub fn input(input: &str) -> IResult<&str, Vec<Instruction>> {
        separated_list1(newline, instruction)(input)
    }
}

fn part1(input: &str) -> usize {
    let mut total = 0;
    let mut state = [[false; 1000]; 1000];

    for instr in parse::input(input).unwrap().1.into_iter() {
        let a = min(instr.from.0, instr.to.0);
        let b = max(instr.from.0, instr.to.0);
        let c = min(instr.from.1, instr.to.1);
        let d = max(instr.from.1, instr.to.1);

        for (i, j) in (a..=b).cartesian_product(c..=d) {
            match (instr.operation, state[i][j]) {
                (TurnOn, false) => {
                    state[i][j] = true;
                    total += 1;
                },
                (TurnOff, true) => {
                    state[i][j] = false;
                    total -= 1;
                }
                (Toggle, true) => {
                    state[i][j] = false;
                    total -= 1;
                },
                (Toggle, false) => {
                    state[i][j] = true;
                    total += 1;
                },
                _ => { continue }
            }
        }
    }

    total
}

fn part2(input: &str) -> usize {
    let mut state = [[0; 1000]; 1000];

    for instr in parse::input(input).unwrap().1.into_iter() {
        let a = min(instr.from.0, instr.to.0);
        let b = max(instr.from.0, instr.to.0);
        let c = min(instr.from.1, instr.to.1);
        let d = max(instr.from.1, instr.to.1);

        for (i, j) in (a..=b).cartesian_product(c..=d) {
            match (instr.operation, state[i][j]) {
                (TurnOn, _) => {
                    state[i][j] += 1;

                },
                (TurnOff, b) if b > 0 => {
                    state[i][j] -= 1;
                }
                (Toggle, _) => {
                    state[i][j] += 2;
                },
                _ => { continue }
            }
        }
    }

    state.iter().map(|s| s.iter().sum::<usize>()).sum::<usize>()
}

build_main!("day06.txt", "Part 1" => part1, "Part 2" => part2);