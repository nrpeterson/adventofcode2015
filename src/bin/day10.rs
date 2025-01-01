use itertools::Itertools;
use adventofcode2015::build_main;

fn look_and_say(ds: Vec<u8>) -> Vec<u8> {
    ds.iter().dedup_with_count().flat_map(|(x, y)| [x as u8, *y]).collect()
}

fn parse_input(input: &str) -> Vec<u8> {
    input.trim().chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn part1(input: &str) -> usize {
    let mut digits = parse_input(input);

    for _ in 0..40 {
        digits = look_and_say(digits);
    }

    digits.len()
}

fn part2(input: &str) -> usize {
        let mut digits = parse_input(input);

        for _ in 0..50 {
            digits = look_and_say(digits);
        }

        digits.len()
}

build_main!("day10.txt", "Part 1" => part1, "Part 2" => part2);