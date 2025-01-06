use adventofcode2015::build_main;

fn part1(input: &str) -> usize {
    let target = input.parse::<usize>().unwrap() / 10;
    let mut best = target;
    let mut i = 1;
    let mut presents = vec![0; target + 1];

    while i < best {
        let mut ki = i;
        while ki <= best {
            presents[ki] += i;
            if presents[ki] >= target {
                best = ki;
            }

            ki += i;
        }

        i += 1;
    }

    best
}

fn part2(input: &str) -> usize {
    let target = input.parse::<usize>().unwrap();
    let mut best = target;
    let mut i = 1;
    let mut presents = vec![0; target + 1];

    while i < best {
        let mut k = 1;
        let mut ki = i;
        while ki <= best && k <= 50 {
            presents[ki] += 11 * i;
            if presents[ki] >= target {
                best = ki;
            }

            ki += i;
            k += 1;
        }

        i += 1;
    }

    best
}


build_main!("day20.txt", "Part 1" => part1, "Part 2" => part2);