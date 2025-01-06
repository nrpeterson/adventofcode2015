use std::ops::{Index, IndexMut};
use adventofcode2015::build_main;

#[derive(Copy, Clone)]
enum Register { A, B }

#[derive(Copy, Clone)]
enum Offset { Forward(usize), Backward(usize) }
use Offset::*;

#[derive(Copy, Clone)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(Offset),
    Jie(Register, Offset),
    Jio(Register, Offset)
}
use Instruction::*;


struct State { a: usize, b: usize, cur: Option<usize> }

impl Index<Register> for State {
    type Output = usize;

    fn index(&self, index: Register) -> &Self::Output {
        match index {
            Register::A => &self.a,
            Register::B => &self.b
        }
    }
}

impl IndexMut<Register> for State {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        match index {
            Register::A => &mut self.a,
            Register::B => &mut self.b
        }
    }
}

fn run(program: &[Instruction], (a, b, cur): (usize, usize, Option<usize>)) -> usize {
    let mut state = State { a, b, cur };

    loop {
        match state.cur {
            None => return state.b,
            Some(i) => {
                let instr = program[i];
                let offset = match instr {
                    Hlf(r) => { state[r] /= 2; Forward(1) },
                    Tpl(r) => { state[r] *= 3; Forward(1) },
                    Inc(r) => { state[r] += 1; Forward(1) },
                    Jmp(o) => o,
                    Jie(r, o) => { if state[r] % 2 == 0 { o } else { Forward(1) }},
                    Jio(r, o) => { if state[r] == 1 { o } else { Forward(1) }}
                };

                state.cur = match (i, offset) {
                    (i, Forward(d)) if i + d >= program.len() => None,
                    (i, Forward(d)) => Some(i + d),
                    (i, Backward(d)) if i < d => None,
                    (i, Backward(d)) => Some(i - d)
                }
            }
        }
    }
}

mod parse {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, digit1, newline};
    use nom::combinator::{map, map_res, value};
    use nom::IResult;
    use nom::multi::separated_list1;
    use nom::sequence::{delimited, pair, preceded};
    use crate::{Instruction, Offset, Register};
    use crate::Instruction::*;
    use crate::Offset::*;

    fn register(input: &str) -> IResult<&str, Register> {
        alt((
            value(Register::A, tag("a")),
            value(Register::B, tag("b"))
        ))(input)
    }

    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn offset(input: &str) -> IResult<&str, Offset> {
        map(
            pair(
                alt((
                    value(true, char('+')),
                    value(false, char('-'))
                )),
                number
            ),
            |(is_pos, num)| { if is_pos { Forward(num) } else { Backward(num) } }
        )(input)
    }


    fn instruction(input: &str) -> IResult<&str, Instruction> {
        alt((
            map(preceded(tag("hlf "), register), Hlf),
            map(preceded(tag("tpl "), register), Tpl),
            map(preceded(tag("inc "), register), Inc),
            map(preceded(tag("jmp "), offset), Jmp),
            map(
                pair(
                    delimited(tag("jie "), register, tag(", ")),
                    offset
                ),
                |(r, o)| Jie(r, o)
            ),
            map(
                pair(
                    delimited(tag("jio "), register, tag(", ")),
                    offset
                ),
                |(r, o)| Jio(r, o)
            )
        ))(input)
    }

    pub fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
        separated_list1(newline, instruction)(input)
    }
}

fn part1(input: &str) -> usize {
    let instructions = parse::instructions(input).unwrap().1;
    run(&instructions, (0, 0, Some(0)))
}

fn part2(input: &str) -> usize {
    let instructions = parse::instructions(input).unwrap().1;
    run(&instructions, (1, 0, Some(0)))
}

build_main!("day23.txt", "Part 1" => part1, "Part 2" => part2);