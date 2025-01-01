use std::collections::HashSet;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::value;
use nom::IResult;
use nom::multi::many1;
use adventofcode2015::build_main;

#[derive(Copy, Clone)]
enum Move { Up, Left, Down, Right }
use Move::*;

impl Move {
    fn apply(&self, (i, j): (isize, isize)) -> (isize, isize) {
        match self {
            Up => (i - 1, j),
            Down => (i + 1, j),
            Left => (i, j - 1),
            Right => (i, j + 1)
        }
    }
}

struct State {
    santa: (isize, isize),
    robot: (isize, isize),
    seen: HashSet<(isize, isize)>
}

impl State {
    fn new() -> State {
        let santa = (0, 0);
        let robot = (0, 0);
        let seen = HashSet::from([santa]);
        State { santa, robot, seen}
    }

    fn move_santa(&mut self, mv: Move) {
        self.santa = mv.apply(self.santa);
        self.seen.insert(self.santa);
    }

    fn move_robot(&mut self, mv: Move) {
        self.robot = mv.apply(self.robot);
        self.seen.insert(self.robot);
    }
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    many1(
        alt((
            value(Down, char('v')),
            value(Up, char('^')),
            value(Left, char('<')),
            value(Right, char('>'))
        ))
    )(input)
}

fn part1(input: &str) -> usize {
    let moves = parse_moves(input).expect("parse moves").1;

    moves.into_iter()
        .fold(State::new(), |mut acc, cur| {
            acc.move_santa(cur);
            acc
        })
        .seen.len()
}

fn part2(input: &str) -> usize {
    let moves = parse_moves(input).expect("parse moves").1;

    moves.into_iter().enumerate()
        .fold(State::new(), |mut acc, (i, cur)| {
            if i % 2 == 0 { acc.move_santa(cur) } else { acc.move_robot(cur) };
            acc
        })
        .seen.len()
}

build_main!("day03.txt", "Part 1" => part1, "Part 2" => part2);