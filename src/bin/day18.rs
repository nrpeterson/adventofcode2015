use std::collections::{HashMap, HashSet};
use itertools::iproduct;
use adventofcode2015::build_main;

struct GameOfLife {
    rows: isize,
    cols: isize,
    on: HashSet<(isize, isize)>
}

impl GameOfLife {
    fn update(&mut self) {
        let mut on_neighbors = HashMap::new();

        // Assure that all on vertices are present
        self.on.iter().for_each(|&p| { on_neighbors.insert(p, 0); });

        iproduct!(self.on.iter(), -1..=1, -1..=1)
            .filter(|&(_, di, dj)| (di, dj) != (0, 0))
            .map(|(&(i, j), di, dj)| (i + di, j + dj))
            .filter(|&(s, t)| s >= 0 && s < self.rows && t >= 0 && t < self.cols)
            .for_each(|p| {
                let entry = on_neighbors.entry(p).or_insert(0);
                *entry += 1;
            });

        for (p, n) in on_neighbors {
            if self.on.contains(&p) && n != 2 && n != 3 {
                self.on.remove(&p);
            }
            else if !self.on.contains(&p) && n == 3 {
                self.on.insert(p);
            }
        }
    }
}

fn parse_board(input: &str) -> GameOfLife {
    let on = input.lines().enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(j, _)| (i as isize, j as isize))
        })
        .collect();

    let rows = input.lines().count() as isize;
    let cols = input.lines().next().unwrap().len() as isize;

    GameOfLife { rows, cols, on }
}

fn part1(input: &str) -> usize {
    let mut game = parse_board(input);
    (0..100).for_each(|_| game.update());
    game.on.len()
}

fn part2(input: &str) -> usize {
    let mut game = parse_board(input);

    fn corners_on(g: &mut GameOfLife) {
        g.on.insert((0, 0));
        g.on.insert((0, g.cols - 1));
        g.on.insert((g.rows - 1, 0));
        g.on.insert((g.rows - 1, g.cols - 1));
    }

    corners_on(&mut game);

    for _ in 0..100 {
        game.update();
        corners_on(&mut game);
    }

    game.on.len()

}

build_main!("day18.txt", "Part 1" => part1, "Part 2" => part2);