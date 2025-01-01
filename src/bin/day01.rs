use adventofcode2015::build_main;

fn part1(input: &str) -> isize {
    input.chars()
        .fold(0, |acc, cur| {
            match cur {
                '(' => acc + 1,
                ')' => acc - 1,
                _ => panic!("Bad character {cur}")
            }
        })
}

fn part2(input: &str) -> usize {
    let mut cur = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => cur += 1,
            ')' => cur -= 1,
            _ => panic!("Bad character {cur}")
        };
        if cur < 0 {
            return i + 1;
        }
    }

    panic!("No result found");
}

build_main!("day01.txt", "Part 1" => part1, "Part 2" => part2);