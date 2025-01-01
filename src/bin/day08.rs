use adventofcode2015::build_main;

fn count_diff(input: &str) -> usize {
    let mut result = 0;
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == '"' {
            result += 1;
        }
        else if c == '\\' {
            let next = chars.next().unwrap();
            if next == 'x' {
                result += 3;
                chars.next();
                chars.next();
            }
            else {
                result += 1;
            }
        }
        else {
            continue
        }
    }

    result
}

fn part1(input: &str) -> usize {
    input.lines().map(count_diff).sum()
}

fn part2(input: &str) -> usize {
    input.lines()
        .map(|line| line.chars().filter(|&c| c == '\\' || c == '"').count() + 2)
        .sum()
}

build_main!("day08.txt", "Part 1" => part1, "Part 2" => part2);