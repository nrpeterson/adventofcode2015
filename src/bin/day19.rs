use std::collections::{HashMap, HashSet};
use std::ops::Index;
use itertools::{chain, Itertools};
use adventofcode2015::build_main;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Rule<'a> {
    from: &'a str,
    to: Vec<&'a str>
}

struct Grammar<'a> {
    start_rule: Rule<'a>,
    rules: HashMap<&'a str, Vec<Rule<'a>>>,
    nonterminals: HashSet<&'a str>
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct EarleyItem<'a> {
    rule: &'a Rule<'a>,
    rule_item: usize,
    start: usize
}

#[derive(Copy, Clone, Debug)]
struct BackPointer {
    from: (usize, usize),
    by: Option<(usize, usize)>
}

#[derive(Clone, Debug)]
struct StateSet<'a>(Vec<(EarleyItem<'a>, Vec<BackPointer>)>);

impl<'a> StateSet<'a> {
    fn new() -> StateSet<'a> { StateSet(Vec::new()) }

    fn insert(&mut self, item: EarleyItem<'a>, pred: Option<BackPointer>) {
        for (item_i, preds_i) in self.0.iter_mut() {
            if *item_i == item {
                pred.into_iter().for_each(|p| preds_i.push(p));
                return
            }
        }

        let preds = pred.into_iter().collect();
        self.0.push((item, preds));
    }

    fn len(&self) -> usize { self.0.len() }

    fn iter(&self) -> impl Iterator<Item=&(EarleyItem<'a>, Vec<BackPointer>)> {
        self.0.iter()
    }
}

impl<'a> Index<usize> for StateSet<'a> {
    type Output = (EarleyItem<'a>, Vec<BackPointer>);
    fn index(&self, idx: usize) -> &(EarleyItem<'a>, Vec<BackPointer>) { &self.0[idx] }
}

impl<'a> Grammar<'a> {
    fn new(start_rule: Rule<'a>, other_rules: Vec<Rule<'a>>) -> Grammar<'a> {
        let mut rules = HashMap::new();
        for rule in other_rules {
            rules.entry(rule.from).or_insert_with(Vec::new).push(rule);
        }

        let nonterminals = rules.keys().cloned().collect();

        Grammar { start_rule, rules, nonterminals }
    }

    fn build_table(&'a self, target: &[&'a str]) -> Vec<StateSet<'a>> {
        let mut table = vec![StateSet::new(); target.len() + 1];
        table[0].insert(EarleyItem { rule: &self.start_rule, rule_item: 0, start: 0 }, None);

        for k in 0..=target.len() {
            let mut i = 0;
            while i < table[k].len() {
                let EarleyItem { rule, rule_item, start } = table[k][i].0;

                if rule_item == rule.to.len() {
                    // Completion
                    let mut j = 0;
                    while j < table[start].len() {
                        let EarleyItem { rule: r, rule_item: r_item, start: s } = table[start][j].0;

                        if r_item < r.to.len() && r.to[r_item] == rule.from {
                            table[k].insert(
                                EarleyItem { rule: r, rule_item: r_item + 1, start: s },
                                Some(BackPointer { from: (start, j), by: Some((k, i)) })
                            );
                        }

                        j += 1;
                    }
                }
                else {
                    // Scan.  Note we want to allow non-terminals in search string, so we don't restrict
                    // scanning to terminals.
                    if k < target.len() && rule.to[rule_item] == target[k] {
                        table[k+1].insert(
                            EarleyItem { rule, rule_item: rule_item + 1, start },
                            Some(BackPointer { from: (k, i), by: None })
                        );
                    }

                    if self.nonterminals.contains(rule.to[rule_item]) {
                        // Predict.
                        for r in self.rules[&rule.to[rule_item]].iter() {
                            if r.from == rule.to[rule_item] {
                                table[k].insert(
                                    EarleyItem { rule: r, rule_item: 0, start: k },
                                    None
                                );
                            }
                        }
                    }
                }

                i += 1;
            }
        }

        table
    }

    fn min_parse_tree(&self, target: &[&'a str]) -> usize {
        let table = self.build_table(target);
        let mut cache: Vec<Vec<Option<usize>>> = table.iter().map(
            |s| s.iter().map(|_| None).collect()
        ).collect();


        let k0 = target.len();
        let i0 = table[k0].iter().enumerate()
            .find(|(_, (item, _))| {
                *item.rule == self.start_rule && item.rule_item == self.start_rule.to.len()
            })
            .map(|(i, _)| i)
            .expect("No solutions found");

        let mut stack = vec![(k0, i0)];

        while let Some(&(k, i)) = stack.last() {
            if cache[k][i].is_none() {
                let (_, backpointers) = &table[k][i];
                if backpointers.is_empty() {
                    cache[k][i] = Some(1);
                    stack.pop();
                }
                else {
                    let missing_deps: Vec<(usize, usize)> = backpointers.iter()
                        .flat_map(|bp| chain!([bp.from], bp.by))
                        .unique()
                        .filter(|&pos| cache[pos.0][pos.1].is_none())
                        .collect();

                    if missing_deps.is_empty() {
                        let result = backpointers.iter()
                            .map(|&BackPointer { from, by }| {
                                let child = by.and_then(|(s, t)| cache[s][t])
                                    .unwrap_or(0);

                                cache[from.0][from.1].unwrap() + child
                            })
                            .min()
                            .unwrap();

                        cache[k][i] = Some(result);
                        stack.pop();
                    }
                    else {
                        stack.extend(missing_deps);
                    }
                }
            }
            else {
                stack.pop();
            }
        }

        cache[k0][i0].unwrap()
    }
}

mod parse {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{multispace1, newline, one_of, char as character};
    use nom::combinator::{map, recognize};
    use nom::IResult;
    use nom::multi::{many0, many1, separated_list1};
    use nom::sequence::{pair, separated_pair};
    use crate::{Grammar, Rule};

    fn upper_char(input: &str) -> IResult<&str, char> {
        one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(input)
    }

    fn lower_char(input: &str) -> IResult<&str, char> {
        one_of("abcdefghijklmnopqrstuvwxyz")(input)
    }

    fn symbol(input: &str) -> IResult<&str, &str> {
        alt((
            recognize(pair(upper_char, many0(lower_char))),
            recognize(character('e'))
        ))(input)
    }

    fn rule(input: &str) -> IResult<&str, Rule> {
        map(
            separated_pair(
                symbol,
                tag(" => "),
                many1(symbol)
            ),
            |(from, to)| Rule { from: from, to: to }
        )(input)
    }

    fn grammar(input: &str) -> IResult<&str, Grammar> {
        map(
            separated_list1(newline, rule),
            |rules| {
                let start_rule = Rule { from: "S0", to: vec!["e"] };
                Grammar::new(start_rule, rules)
            }
        )(input)
    }

    pub fn input(input: &str) -> IResult<&str, (Grammar, Vec<&str>)> {
        separated_pair(grammar, multispace1, many1(symbol))(input)
    }
}

fn part1(input: &str) -> usize {
    let (grammar, target) = parse::input(input).unwrap().1;

    let mut seen = HashSet::new();

    for (_, rules) in grammar.rules {
        for rule in rules {
            for i in 0..target.len() {
                if target[i] == rule.from {
                    let mut new: Vec<&str> = Vec::new();
                    new.extend(&target[..i]);
                    new.extend(&rule.to);
                    new.extend(&target[i+1..]);
                    seen.insert(new);
                }
            }
        }
    }
    seen.len()
}

fn part2(input: &str) -> usize {
    let (grammar, target) = parse::input(input).unwrap().1;

    grammar.min_parse_tree(&target) - 1
}


build_main!("day19.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "e => H
e => O
H => HO
H => OH
O => HH

HOHOHO";

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 6);
    }
}