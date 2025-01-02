use std::cmp::min;
use adventofcode2015::build_main;

#[derive(Debug)]
struct Reindeer {
    speed: usize,
    stamina: usize,
    rest_period: usize
}

impl Reindeer {
    fn distance(&self, t: usize) -> usize {
        let full_periods = t / (self.rest_period + self.stamina);
        let remaining_secs = min(self.stamina, t % (self.rest_period + self.stamina));

        full_periods * self.speed * self.stamina + remaining_secs * self.speed
    }
}

enum ReindeerStatus {
    Resting(usize),
    Flying(usize)
}
use ReindeerStatus::*;

struct Trajectory {
    reindeer: Reindeer,
    status: ReindeerStatus,
    position: usize
}

impl Trajectory {
    fn of(reindeer: Reindeer) -> Trajectory {
        let status = Flying(reindeer.stamina);
        let position = 0;
        Trajectory { reindeer, status, position }
    }
}

impl Iterator for Trajectory {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.status {
            Resting(1) => { self.status = Flying(self.reindeer.stamina); },
            Resting(n) => { self.status = Resting(n - 1); },
            Flying(1) => {
                self.status = Resting(self.reindeer.rest_period);
                self.position += self.reindeer.speed;
            },
            Flying(n) => {
                self.status = Flying(n - 1);
                self.position += self.reindeer.speed;
            }
        }

        Some(self.position)
    }
}

struct Race {
    trajectories: Vec<Trajectory>,
    points: Vec<usize>
}

impl Race {
    fn new(reindeer: Vec<Reindeer>) -> Race {
        let trajectories: Vec<Trajectory> = reindeer.into_iter()
            .map(|r| Trajectory::of(r))
            .collect();

        let points = vec![0; trajectories.len()];
        Race { trajectories, points }
    }
}

impl Iterator for Race {
    type Item = (Vec<usize>, Vec<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let positions: Vec<usize> = self.trajectories.iter_mut()
            .map(|t| t.next().unwrap())
            .collect();

        let best = *positions.iter().max().unwrap();
        for (i, &pos) in positions.iter().enumerate() {
            if pos == best {
                self.points[i] += 1;
            }
        }

        Some((positions, self.points.clone()))
    }
}

mod parse {
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, digit1, newline};
    use nom::combinator::{map, map_res};
    use nom::IResult;
    use nom::multi::separated_list1;
    use nom::sequence::{delimited, preceded, tuple};
    use super::Reindeer;

    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn reindeer(input: &str) -> IResult<&str, Reindeer> {
        map(
            tuple((
                preceded(tuple((alpha1, tag(" can fly "))), number),
                preceded(tag(" km/s for "), number),
                delimited(tag(" seconds, but then must rest for "), number, tag(" seconds."))
            )),
            |(speed, stamina, rest_period)| {
                Reindeer { speed, stamina, rest_period }
            }
        )(input)
    }

    pub fn input(input: &str) -> IResult<&str, Vec<Reindeer>> {
        separated_list1(newline, reindeer)(input)
    }
}

fn part1(input: &str) -> usize {
    let reindeer = parse::input(input).unwrap().1;
    reindeer.iter().map(|r| r.distance(2503)).max().unwrap()
}

fn part2(input: &str) -> usize {
    let reindeer = parse::input(input).unwrap().1;
    let mut race = Race::new(reindeer);

    let (_, scores) = race.nth(2503).unwrap();

    *scores.iter().max().unwrap()
}

build_main!("day14.txt", "Part 1" => part1, "Part 2" => part2);