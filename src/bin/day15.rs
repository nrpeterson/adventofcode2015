use std::cmp::max;
use std::ops::{Add, Mul};
use adventofcode2015::build_main;

#[derive(Debug, Copy, Clone)]
struct Ingredient {
    tsps: isize,
    cals: isize,
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize
}

impl Ingredient {
    fn score(&self) -> isize {
        [self.capacity, self.durability, self.flavor, self.texture].iter()
            .map(|&x| max(x, 0))
            .product()
    }
}

impl Default for Ingredient {
    fn default() -> Self {
        Ingredient { tsps: 0, cals: 0, capacity: 0, durability: 0, flavor: 0, texture: 0 }
    }
}

impl Mul<isize> for Ingredient {
    type Output = Ingredient;

    fn mul(self, rhs: isize) -> Self::Output {
        Ingredient {
            tsps: self.tsps * rhs,
            cals: self.cals * rhs,
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs
        }
    }
}

impl Add for Ingredient {
    type Output = Ingredient;

    fn add(self, rhs: Self) -> Self::Output {
        Ingredient {
            tsps: self.tsps + rhs.tsps,
            cals: self.cals + rhs.cals,
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture
        }
    }
}

mod parse {
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, char, digit1, newline};
    use nom::combinator::{map, map_res, opt};
    use nom::IResult;
    use nom::multi::separated_list1;
    use nom::sequence::{delimited, pair, preceded, terminated, tuple};
    use crate::Ingredient;

    fn number(input: &str) -> IResult<&str, isize> {
        map(
            pair(
                map(opt(char('-')), |s| if s.is_none() { 1isize } else { -1isize }),
                map_res(digit1, |s: &str| s.parse::<isize>())
            ),
            |(signum, val)| signum * val
        )(input)
    }

    fn ingredient(input: &str) -> IResult<&str, Ingredient> {
        map(
            tuple((
                terminated(alpha1, tag(": ")),
                delimited(tag("capacity "), number, tag(", ")),
                delimited(tag("durability "), number, tag(", ")),
                delimited(tag("flavor "), number, tag(", ")),
                delimited(tag("texture "), number, tag(", ")),
                preceded(tag("calories "), number)
            )),
            |(_, capacity, durability, flavor, texture, cals)| {
                Ingredient { tsps: 1, cals, capacity, durability, flavor, texture }
            }
        )(input)
    }

    pub fn input(input: &str) -> IResult<&str, Vec<Ingredient>> {
        separated_list1(newline, ingredient)(input)
    }
}

struct Choice {
    i: usize,
    cur: Ingredient
}

fn part1(input: &str) -> isize {
    let ingredients = parse::input(input).unwrap().1;
    let n = ingredients.len();

    let mut choices = Vec::new();
    choices.push(Choice { i: 0, cur: Ingredient::default() });
    let mut best = 0;

    while let Some(&Choice { i, cur }) = choices.last() {
        if i == n - 2 {
            let rem = 100 - cur.tsps;
            best = max(best, (ingredients[i+1] * rem + cur).score());
            while let Some(&Choice { cur: Ingredient { tsps: 100, ..}, ..}) = choices.last() {
                choices.pop();
            }
            if let Some(last) = choices.last_mut() {
                last.cur = last.cur + ingredients[last.i];

                if last.i == 0 && last.cur.tsps > 100 {
                    break
                }
            }
        }
        else {
            choices.push(Choice { i: i + 1, cur });
        }
    }

    best
}

fn part2(input: &str) -> isize {
    let ingredients = parse::input(input).unwrap().1;
    let n = ingredients.len();

    let mut choices = Vec::new();
    choices.push(Choice { i: 0, cur: Ingredient::default() });
    let mut best = 0;

    while let Some(&Choice { i, cur }) = choices.last() {
        if i == n - 2 {
            let rem = 100 - cur.tsps;
            let overall = cur + ingredients[i + 1] * rem;

            if overall.cals == 500 {
                best = max(best, (ingredients[i+1] * rem + cur).score());
            }
            while let Some(&Choice { i, cur }) = choices.last() {
                if cur.tsps == 100 || cur.cals + ingredients[i].cals > 500 {
                    choices.pop();
                }
                else {
                    break
                }
            }

            if let Some(last) = choices.last_mut() {
                last.cur = last.cur + ingredients[last.i];
            }
            else {
                break
            }
        }
        else {
            choices.push(Choice { i: i + 1, cur });
        }
    }

    best
}


build_main!("day15.txt", "Part 1" => part1, "Part 2" => part2);