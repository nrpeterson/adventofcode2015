use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use adventofcode2015::build_main;

struct Graph {
    num_nodes: usize,
    dists: Vec<Vec<usize>>
}

impl Graph {
    fn from_dists(dist_list: Vec<(&str, &str, usize)>) -> Graph {
        let city_names: Vec<&str> = dist_list.iter()
            .flat_map(|&(a, b, _)| vec![a, b])
            .unique()
            .collect();

        let city_ids: HashMap<&str, usize> = city_names.iter().enumerate()
            .map(|(i, &name)| (name, i))
            .collect();
        
        let num_nodes = city_names.len();

        let mut dists = vec![vec![0; num_nodes]; num_nodes];
        for (a, b, dist) in dist_list.into_iter() {
            let i_a = city_ids[&a];
            let i_b = city_ids[&b];
            dists[i_a][i_b] = dist;
            dists[i_b][i_a] = dist;
        }

        Graph { num_nodes, dists }
    }
}

fn parse_input(input: &str) -> Graph {
    let dists: Vec<(&str, &str, usize)> = input.lines()
        .map(|line| line.split(" ").collect_vec())
        .map(|parts| (parts[0], parts[2], parts[4].parse::<usize>().unwrap()))
        .collect();

    Graph::from_dists(dists)
}



fn part1(input: &str) -> usize {
    let graph = parse_input(input);

    let mut best = usize::MAX;
    let mut queue: VecDeque<(Vec<usize>, usize)> = (0..graph.num_nodes)
        .map(|i| (vec![i], 0))
        .collect();

    while let Some((route, cost)) = queue.pop_front() {
        if cost >= best {
            continue;
        }
        if route.len() == graph.num_nodes {
            best = min(best, cost);
        }
        else {
            for next in 0..graph.num_nodes {
                if !route.contains(&next) {
                    let mut next_route = route.clone();
                    next_route.push(next);
                    let next_cost = cost + graph.dists[*route.last().unwrap()][next];
                    if next_cost < best {
                        queue.push_back((next_route, next_cost));
                    }
                }
            }
        }
    }

    best
}

fn part2(input: &str) -> usize {
    let graph = parse_input(input);

    let mut best = 0;
    let mut queue: VecDeque<(Vec<usize>, usize)> = (0..graph.num_nodes)
        .map(|i| (vec![i], 0))
        .collect();

    while let Some((route, cost)) = queue.pop_front() {
        if route.len() == graph.num_nodes {
            best = max(best, cost);
        }
        else {
            for next in 0..graph.num_nodes {
                if !route.contains(&next) {
                    let mut next_route = route.clone();
                    next_route.push(next);
                    let next_cost = cost + graph.dists[*route.last().unwrap()][next];
                    queue.push_back((next_route, next_cost));
                }
            }
        }
    }

    best
}

build_main!("day09.txt", "Part 1" => part1, "Part 2" => part2);