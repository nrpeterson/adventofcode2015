use itertools::{Itertools, MinMaxResult};
use adventofcode2015::build_main;

#[derive(Copy, Clone, Debug)]
struct Password {
    value: [u8; 8]
}

impl Password {
    fn to_string(&self) -> String {
        self.value.iter()
            .map(|&x| (x + ('a' as u8)) as char)
            .collect()
    }

    fn from_string(s: &str) -> Password {
        let value: [u8; 8] = s.chars()
            .map(|c| (c as u8) - ('a' as u8))
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        Password { value }
    }

    fn iter(self) -> PasswordIter {
        PasswordIter { cur: Some(self) }
    }
}

struct PasswordIter { cur: Option<Password> }

impl Iterator for PasswordIter {
    type Item = Password;

    fn next(&mut self) -> Option<Self::Item> {
        let pass = self.cur.as_mut()?;
        let result = Some(pass.clone());

        let mut j = 7;
        while j > 0 && pass.value[j] == 25 {
            pass.value[j] = 0;
            j -= 1;
        }

        if pass.value[j] == 25 {
            self.cur = None;
        }
        else {
            pass.value[j] += 1;
        }

        result
    }
}

fn is_valid(password: &Password) -> bool {
    let contains_trip = password.value.iter().tuple_windows()
        .any(|(&a, &b, &c)| c == b + 1 && b == a + 1);

    let bad = ['i', 'o', 'l'].map(|c| (c as u8) - ('a' as u8));
    let contains_bad = password.value.iter().any(|o| bad.contains(o));

    let pair_indices = password.value.iter().tuple_windows().enumerate()
        .filter(|(_, (&a, &b))| a == b)
        .map(|(i, _)| i)
        .minmax();

    let contains_pairs = match pair_indices {
        MinMaxResult::MinMax(a, b) if b > a + 1 => true,
        _ => false
    };

    contains_trip && !contains_bad && contains_pairs
}

fn part1(input: &str) -> String {
    let password = Password::from_string(input.trim());

    password.iter()
        .filter(is_valid)
        .map(|pass| pass.to_string())
        .next()
        .unwrap()
}

fn part2(input: &str) -> String {
    let password = Password::from_string(input.trim());

    password.iter()
        .filter(is_valid)
        .map(|pass| pass.to_string())
        .dropping(1)
        .next()
        .unwrap()
}

build_main!("day11.txt", "Part 1" => part1, "Part 2" => part2);