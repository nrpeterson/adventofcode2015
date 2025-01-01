use md5;
use adventofcode2015::build_main;

fn part1(input: &str) -> usize {
    let mut i = 0;

    loop {
        let s = format!("{input}{i}");
        let d = md5::compute(&s);

        if format!("{d:x}").chars().take(5).all(|ch| ch == '0') {
            return i
        }

        i += 1;
    }
}

fn part2(input: &str) -> usize {
    let mut i = 0;

    loop {
        let s = format!("{input}{i}");
        let d = md5::compute(&s);

        if format!("{d:x}").chars().take(6).all(|ch| ch == '0') {
            return i
        }

        i += 1;
    }
}

build_main!("day04.txt", "Part 1" => part1, "Part 2" => part2);