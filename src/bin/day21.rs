use std::ops::Add;
use itertools::{iproduct, Itertools};
use adventofcode2015::build_main;

#[derive(Copy, Clone, Debug)]
struct Stats {
    hp: usize,
    damage: usize,
    armor: usize,
    cost: usize
}

impl Stats {
    fn beats(&self, enemy: &Stats) -> bool {
        let my_damage = if self.damage > enemy.armor { self.damage - enemy.armor } else { 1 };
        let enemy_damage = if enemy.damage > self.armor { enemy.damage - self.armor } else { 1 };

        enemy.hp.div_ceil(my_damage) <= self.hp.div_ceil(enemy_damage)
    }
}

impl Add for Stats {
    type Output = Stats;
    fn add(self, rhs: Self) -> Self::Output {
        Stats {
            hp: self.hp + rhs.hp,
            damage: self.damage + rhs.damage,
            armor: self.armor + rhs.armor,
            cost: self.cost + rhs.cost
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Stats { hp: 0, damage: 0, armor: 0, cost: 0 }
    }
}

fn choices(from: &[Stats], sizes: impl Iterator<Item=usize>) -> Vec<Stats> {
    let mut result = Vec::new();
    for size in sizes {
        for combs in from.iter().cloned().combinations(size) {
            result.push(combs.into_iter().reduce(|x, y| x + y).unwrap_or_default());
        }
    }

    result
}

mod parse {
    use nom::bytes::complete::tag;
    use nom::character::complete::{digit1, newline};
    use nom::combinator::{map, map_res};
    use nom::IResult;
    use nom::sequence::{delimited, preceded, tuple};
    use crate::Stats;

    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    pub fn input(input: &str) -> IResult<&str, Stats> {
        map(
            tuple((
                delimited(tag("Hit Points: "), number, newline),
                delimited(tag("Damage: "), number, newline),
                preceded(tag("Armor: "), number)
            )),
            |(hp, damage, armor)| Stats { hp, damage, armor, cost: 0 }
        )(input)
    }
}

const WEAPONS: [Stats; 5] = [
    Stats { hp: 0, damage: 4, armor: 0, cost: 8 },
    Stats { hp: 0, damage: 5, armor: 0, cost: 10 },
    Stats { hp: 0, damage: 6, armor: 0, cost: 25 },
    Stats { hp: 0, damage: 7, armor: 0, cost: 40 },
    Stats { hp: 0, damage: 8, armor: 0, cost: 74 }
];

const ARMOR: [Stats; 5] = [
    Stats { hp: 0, damage: 0, armor: 1, cost: 13 },
    Stats { hp: 0, damage: 0, armor: 2, cost: 31 },
    Stats { hp: 0, damage: 0, armor: 3, cost: 53 },
    Stats { hp: 0, damage: 0, armor: 4, cost: 75 },
    Stats { hp: 0, damage: 0, armor: 5, cost: 102 }
];

const RINGS: [Stats; 6] = [
    Stats { hp: 0, damage: 1, armor: 0, cost: 25 },
    Stats { hp: 0, damage: 2, armor: 0, cost: 50 },
    Stats { hp: 0, damage: 3, armor: 0, cost: 100 },
    Stats { hp: 0, damage: 0, armor: 1, cost: 20 },
    Stats { hp: 0, damage: 0, armor: 2, cost: 40 },
    Stats { hp: 0, damage: 0, armor: 3, cost: 80 },
];

fn part1(input: &str) -> usize {
    let boss = parse::input(input).unwrap().1;

    let armor_opts: Vec<Stats> = choices(&ARMOR, 0..=1);
    let ring_opts = choices(&RINGS, 0..=2);

    iproduct!(WEAPONS.iter(), armor_opts.iter(), ring_opts.iter())
        .map(|(w, a, r)| {
            let mut total = w.clone() + a.clone() + r.clone();
            total.hp = 100;
            total
        })
        .filter(|s| s.beats(&boss))
        .map(|s| s.cost)
        .min()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let boss = parse::input(input).unwrap().1;

    let armor_opts: Vec<Stats> = choices(&ARMOR, 0..=1);
    let ring_opts = choices(&RINGS, 0..=2);

    iproduct!(WEAPONS.iter(), armor_opts.iter(), ring_opts.iter())
        .map(|(w, a, r)| {
            let mut total = w.clone() + a.clone() + r.clone();
            total.hp = 100;
            total
        })
        .filter(|s| !s.beats(&boss))
        .map(|s| s.cost)
        .max()
        .unwrap()
}

build_main!("day21.txt", "Part 1" => part1, "Part 2" => part2);