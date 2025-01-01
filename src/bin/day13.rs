use std::collections::HashMap;
use itertools::Itertools;
use adventofcode2015::build_main;

struct Graph {
    num_nodes: usize,
    dists: Vec<Vec<isize>>
}

impl Graph {
    fn from_input(input: Vec<(&str, &str, isize)>) -> Graph {
        let nodes: Vec<&str> = input.iter()
            .flat_map(|&(a, b, _)| [a, b])
            .unique()
            .collect();

        let node_ids: HashMap<&str, usize> = nodes.iter().enumerate()
            .map(|(i, &node)| (node, i))
            .collect();

        let num_nodes = nodes.len();
        let mut dists = vec![vec![0; num_nodes]; num_nodes];

        input.into_iter().for_each(|(a, b, net)| {
            let i_a = node_ids[&a];
            let i_b = node_ids[&b];
            dists[i_a][i_b] += net;
            dists[i_b][i_a] += net;
        });

        Graph { num_nodes, dists }
    }
}

mod parse {
    use nom::IResult;
    use crate::Graph;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, char, digit1, newline};
    use nom::combinator::{map, map_res, value};
    use nom::multi::separated_list1;
    use nom::sequence::{delimited, terminated, tuple};

    fn line(input: &str) -> IResult<&str, (&str, &str, isize)> {
        map(
            tuple((
                terminated(alpha1, tag(" would ")),
                alt((
                    value(1isize, tag("gain ")),
                    value(-1isize, tag("lose "))
                )),
                map_res(digit1, |s: &str| s.parse::<isize>()),
                delimited(tag(" happiness units by sitting next to "), alpha1, char('.'))
            )),
            |(a, signum, val, b)| (a, b, signum * val)
        )(input)
    }

    pub fn graph(input: &str) -> IResult<&str, Graph> {
        map(
            separated_list1(newline, line),
            |lines| Graph::from_input(lines)
        )(input)
    }
}

fn part1(input: &str) -> isize {
    let graph = parse::graph(input).unwrap().1;

    (1..graph.num_nodes).permutations(graph.num_nodes - 1)
        .map(|p| {
            let zero_costs = graph.dists[0][p[0]] + graph.dists[0][*p.last().unwrap()];
            p.into_iter().tuple_windows()
                .map(|(a, b)| graph.dists[a][b])
                .sum::<isize>() + zero_costs
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> isize {
    let graph = parse::graph(input).unwrap().1;

    (0..graph.num_nodes).permutations(graph.num_nodes)
        .map(|p| {
            p.into_iter().tuple_windows()
                .map(|(a, b)| graph.dists[a][b])
                .sum::<isize>()
        })
        .max()
        .unwrap()
}

build_main!("day13.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 330);
    }
}