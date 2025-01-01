use std::ops::{Index, IndexMut};
use itertools::Itertools;
use adventofcode2015::build_main;

fn is_nice1(input: &str) -> bool {
    let mut vowels = 0;
    let mut dups = 0;

    if input.starts_with(&['a', 'e', 'i', 'o', 'u']) {
        vowels += 1;
    }

    for (prev, cur) in input.chars().tuple_windows() {
        if ['a', 'e', 'i', 'o', 'u'].contains(&cur) {
            vowels += 1;
        }

        if prev == cur {
            dups += 1;
        }

        if [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')].contains(&(prev, cur)) {
            return false;
        }
    }

    vowels >= 3 && dups >= 1
}

#[derive(Copy, Clone)]
struct CharMap<T> {
    data: [T; 26]
}

impl<T> Index<char> for CharMap<T> {
    type Output = T;

    fn index(&self, index: char) -> &Self::Output {
        &self.data[(index as usize) - ('a' as usize)]
    }
}

impl<T> IndexMut<char> for CharMap<T> {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        &mut self.data[(index as usize) - ('a' as usize)]
    }
}

impl<T: Default + Copy> Default for CharMap<T> {
    fn default() -> Self {
        let data = [Default::default(); 26];
        Self { data }
    }
}

fn is_nice2(input: &str) -> bool {
    let contains_triple = input.chars()
        .tuple_windows()
        .any(|(a, _, c)| a == c);

    if !contains_triple {
        return false
    }

    let mut first_seen: CharMap<CharMap<Option<usize>>> = Default::default();

    for (i, (a, b)) in input.chars().tuple_windows().enumerate() {
        match first_seen[a][b] {
            Some(last) => {
                if i > last + 1 {
                    return true
                }
            },
            None => {
                first_seen[a][b] = Some(i);
            }
        }
    }

    false
}

fn part1(input: &str) -> usize {
    input.lines()
        .filter(|&line| is_nice1(line))
        .count()
}

fn part2(input: &str) -> usize {
    input.lines()
        .filter(|&line| is_nice2(line))
        .count()
}


build_main!("day05.txt", "Part 1" => part1, "Part 2" => part2);