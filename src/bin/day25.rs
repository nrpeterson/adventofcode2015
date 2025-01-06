use nom::bytes::complete::is_not;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;
use nom::sequence::{preceded, separated_pair};
use adventofcode2015::build_main;

fn parse_input(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(
        preceded(is_not("0123456789"), map_res(digit1, |s: &str| s.parse::<usize>())),
        is_not("0123456789"),
        map_res(digit1, |s: &str| s.parse::<usize>())
    )(input)
}

struct CodePosIter {
    i: usize,
    j: usize
}

impl CodePosIter {
    fn new() -> CodePosIter {
        CodePosIter { i: 0, j: 0 }
    }
}

impl Iterator for CodePosIter {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            self.i = self.i + self.j + 1;
            self.j = 0;
        }
        else {
            self.i -= 1;
            self.j += 1;
        }

        Some((self.i, self.j))
    }
}

fn part1(input: &str) -> usize {
    let (r, c) = parse_input(input).unwrap().1;
    let target = (r - 1, c - 1);
    let mut cur = 20151125u128;
    for coord in CodePosIter::new() {
        cur = (cur * 252533) % 33554393;
        if coord == target {
            return cur as usize;
        }
    }

    unreachable!()
}

build_main!("day25.txt", "Part 1" => part1);