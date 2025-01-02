use std::collections::HashMap;
use adventofcode2015::build_main;

fn parse_input(input: &str) -> Vec<usize> {
    input.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn count_ways(sizes: &[usize], target: usize) -> Vec<usize> {
    let mut cur = vec![HashMap::from([(0, 1)])];

    for &size in sizes {
        let mut next_cur = cur.clone();
        next_cur.push(HashMap::new());

        for (num_containers, combs) in cur.iter().enumerate() {
            for (&total, &count) in combs.iter() {
                let entry = next_cur[num_containers + 1]
                    .entry(total + size)
                    .or_insert(0);

                *entry += count;
            }
        }

        cur = next_cur;
    }

    cur.iter().map(|c| c.get(&target).cloned().unwrap_or(0)).collect()
}

fn part1(input: &str) -> usize {
    let sizes = parse_input(input);
    let counts = count_ways(&sizes, 150);
    counts.into_iter().sum()
}

fn part2(input: &str) -> usize {
    let sizes = parse_input(input);
    let counts = count_ways(&sizes, 150);

    counts.into_iter().find(|&n| n > 0).unwrap()
}

build_main!("day17.txt", "Part 1" => part1, "Part 2" => part2);