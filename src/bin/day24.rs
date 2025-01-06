use std::collections::VecDeque;
use itertools::Itertools;
use adventofcode2015::build_main;

#[derive(Copy, Clone, Eq, PartialEq)]
struct BitSet {
    bits: usize,
    cap: usize
}

impl BitSet {
    fn new(cap: usize) -> BitSet {
        BitSet { bits: 0, cap }
    }

    fn insert(&mut self, x: usize) -> bool {
        let result = self.bits & (1 << x) > 0;
        self.bits |= 1 << x;
        result
    }

    fn iter(&self) -> BitSetIter {
        BitSetIter { bits: self.bits, i: 0 }
    }

    fn complement(&self) -> BitSet {
        let mask = (1 << self.cap) - 1;
        let negated = !self.bits;
        let new_bits = negated & mask;
        BitSet { bits: new_bits, cap: self.cap }
    }
}

struct BitSetIter {
    bits: usize,
    i: usize
}

impl Iterator for BitSetIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.bits > 0 && self.bits & 1 == 0 {
            self.bits >>= 1;
            self.i += 1;
        }

        if self.bits == 0 {
            None
        }
        else {
            let result = Some(self.i);
            self.bits ^= 1;
            result
        }
    }
}

fn parse_input(input: &str) -> Vec<usize> {
    input.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn can_split_in_n(nums: &[usize], n: usize) -> bool {
    if n == 1 {
        return nums.len() > 0
    }

    let s = nums.iter().sum::<usize>();
    if s % n != 0 {
        return false;
    }

    let target = s / n;

    let mut stack = vec![(BitSet::new(nums.len()), 0, target)];

    while let Some((elems, cur_i, cur_target)) = stack.pop() {
        if cur_target == 0 {
            let remaining = elems.complement().iter().collect_vec();
            if can_split_in_n(&remaining, n - 1) {
                return true
            }
            else {
                continue
            }
        }

        if cur_i == nums.len() {
            continue;
        }

        stack.push((elems, cur_i + 1, cur_target));

        if cur_target >= nums[cur_i] {
            let mut new_elems = elems;
            new_elems.insert(cur_i);
            stack.push((new_elems, cur_i + 1, cur_target - nums[cur_i]));
        }

    }

    false
}

struct StackItem { elems: BitSet, count: usize, sum: usize, product: u128, cur_i: usize }

fn solve(numbers: &[usize], n: usize) -> u128 {
    let total = numbers.iter().sum::<usize>();

    assert_eq!(total % n, 0);

    let target = total / n;

    let mut best_prod = u128::MAX;
    let mut best_len = usize::MAX;

    let mut queue = VecDeque::new();

    queue.push_back(
        StackItem {
            elems: BitSet::new(numbers.len()),
            count: 0,
            sum: 0,
            product: 1,
            cur_i: 0
        }
    );

    while let Some(StackItem { elems, count, sum, product, cur_i }) = queue.pop_front() {
        if count > best_len {
            continue;
        }

        if sum == target {
            let remaining = elems.complement().iter().map(|i| numbers[i]).collect_vec();
            if can_split_in_n(&remaining, n - 1) {
                if count < best_len {
                    best_len = count;
                    best_prod = product;
                }
                else {
                    if product < best_prod {
                        best_prod = product;
                    }
                }
            }
        }
        else {
            if cur_i == numbers.len() {
                continue;
            }

            queue.push_back(
                StackItem {
                    elems,
                    count,
                    sum,
                    product,
                    cur_i: cur_i + 1
                }
            );

            if sum + numbers[cur_i] <= target {
                let mut new_elems = elems;
                new_elems.insert(cur_i);

                queue.push_back(
                    StackItem {
                        elems: new_elems,
                        count: count + 1,
                        sum: sum + numbers[cur_i],
                        product: product * (numbers[cur_i] as u128),
                        cur_i: cur_i + 1
                    }
                );
            }
        }
    }

    best_prod
}

fn part1(input: &str) -> u128 {
    let numbers = parse_input(input);
    solve(&numbers, 3)
}

fn part2(input: &str) -> u128 {
    let numbers = parse_input(input);
    solve(&numbers, 4)
}

build_main!("day24.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let numbers = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        assert_eq!(solve(&numbers, 3), 99);
    }

    #[test]
    fn test_part2() {
        let numbers = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        assert_eq!(solve(&numbers, 4), 44);
    }
}