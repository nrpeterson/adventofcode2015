use std::collections::{HashMap, HashSet};
use adventofcode2015::build_main;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Rule {
    from: String,
    to: Vec<String>
}

struct Grammar {
    start_rule: Rule,
    rules: HashMap<String, Vec<Rule>>
}

impl Grammar {
    fn new(start_rule: Rule, rules_list: Vec<Rule>) -> Grammar {
        let mut rules: HashMap<String, Vec<Rule>> = HashMap::new();
        rules_list.into_iter().for_each(|rule| {
            rules.entry(rule.from.clone())
                .or_default()
                .push(rule);
        });

        Grammar { start_rule, rules }
    }
}

mod earley {
    use std::ops::Index;
    use itertools::{chain, Itertools};
    use crate::{Grammar, Rule};

    #[derive(Copy, Clone)]
    struct BackPointer {
        prev_item: (usize, usize),
        advanced_by: Option<(usize, usize)>
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct EarleyItem {
        rule: Rule,
        rule_state: usize,
        parse_start: usize
    }

    impl EarleyItem {
        fn is_complete(&self) -> bool {
            self.rule_state == self.rule.to.len()
        }

        fn cur_symb(&self) -> Option<&String> {
            self.rule.to.get(self.rule_state)
        }
    }

    struct ParseTable {
        data: Vec<Vec<(EarleyItem, Vec<BackPointer>)>>,
        completed_start_rule: Option<(usize, usize)>
    }

    impl ParseTable {
        fn with_rows(n: usize) -> ParseTable {
            let data = vec![Vec::new(); n];
            let completed_start_rule = None;
            ParseTable { data, completed_start_rule }
        }

        fn insert(&mut self, item: EarleyItem, parse_end: usize, pred: Option<BackPointer>) {
            for (item_i, preds_i) in self.data[parse_end].iter_mut() {
                if *item_i == item {
                    pred.into_iter().for_each(|p| preds_i.push(p));
                    return
                }
            }

            let preds = pred.into_iter().collect();
            self.data[parse_end].push((item, preds));
        }

        fn iter(&self) -> impl Iterator<Item=&Vec<(EarleyItem, Vec<BackPointer>)>> {
            self.data.iter()
        }
    }

    impl Index<usize> for ParseTable {
        type Output = Vec<(EarleyItem, Vec<BackPointer>)>;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    fn build_parse_table<'a>(grammar: &Grammar, target: &[String]) -> ParseTable {
        let mut table: ParseTable = ParseTable::with_rows(target.len() + 1);

        table.insert(
            EarleyItem { rule: grammar.start_rule.clone(), rule_state: 0, parse_start: 0 },
            0,
            None
        );

        for cur_parse_end in 0..=target.len() {
            let mut cur_item_index = 0;
            while cur_item_index < table[cur_parse_end].len() {
                let cur_symb = table[cur_parse_end][cur_item_index].0
                    .cur_symb()
                    .map(|symb| symb.clone());


                if cur_symb.is_none() {
                    // We've completed this item. If any items were waiting for this, we can
                    // advance them one step.
                    let cur_item  = &table[cur_parse_end][cur_item_index].0;
                    let cur_parse_start = cur_item.parse_start;
                    let cur_rule_from = cur_item.rule.from.clone();

                    let mut j = 0;
                    while j < table[cur_parse_start].len() {
                        let next_item = &table[cur_parse_start][j].0;

                        if !next_item.is_complete() &&
                            next_item.rule.to[next_item.rule_state] == cur_rule_from {
                            let new_item = EarleyItem {
                                rule: next_item.rule.clone(),
                                rule_state: next_item.rule_state + 1,
                                parse_start: next_item.parse_start
                            };

                            table.insert(
                                new_item,
                                cur_parse_end,
                                Some(BackPointer {
                                    prev_item: (cur_parse_start, j),
                                    advanced_by: Some((cur_parse_end, cur_item_index))
                                })
                            );
                        }

                        j += 1;
                    }
                }
                else {
                    let cur_item = &table[cur_parse_end][cur_item_index].0;
                    let cur_rule_symb = cur_item.cur_symb().unwrap().clone();

                    // Scan: if the current symbol of target matches the current symbol of our
                    // current rule, we can "consume" it and advance this rule by one.
                    //
                    // Note that unlike the traditional Earley parser, I've allowed this to
                    // happen whether or not the current symbol is terminal.  That way, we can
                    // query against nonterminal symbols instead of having to modify the
                    // grammar.
                    if cur_parse_end < target.len() && *cur_rule_symb == target[cur_parse_end] {
                        table.insert(
                            EarleyItem {
                                rule: cur_item.rule.clone(),
                                rule_state: cur_item.rule_state + 1,
                                parse_start: cur_item.parse_start },
                            cur_parse_end + 1,
                            Some(BackPointer {
                                prev_item: (cur_parse_end, cur_item_index),
                                advanced_by: None }
                            )
                        );
                    }

                    // Predict: we may want to use another rule whose "from" is our current
                    // rule's current symbol, so insert them at this point to give it a try.
                    if grammar.rules.contains_key(&cur_rule_symb) {
                        for rule in grammar.rules[&cur_rule_symb].iter() {
                            table.insert(
                                EarleyItem {
                                    rule: rule.clone(),
                                    rule_state: 0,
                                    parse_start: cur_parse_end
                                },
                                cur_parse_end,
                                None
                            );
                        }
                    }
                }

                cur_item_index += 1;
            }
        }

        let k = target.len();
        let completed_start = table[k].iter().enumerate()
            .find(|(_, (item, _))| item.is_complete() && item.rule == grammar.start_rule)
            .map(|(i, _)| (k, i));

        table.completed_start_rule = completed_start;

        table
    }

    pub fn min_parse_tree(grammar: &Grammar, target: &[String]) -> usize {
        let table = build_parse_table(grammar, target);
        let mut cache: Vec<Vec<Option<usize>>> = table.iter().map(
            |s| s.iter().map(|_| None).collect()
        ).collect();

        let k0 = target.len();
        let i0 = table[k0].iter().enumerate()
            .find(|(_, (item, _))| {
                item.rule == grammar.start_rule && item.is_complete()
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
                        .flat_map(|bp| chain!([bp.prev_item], bp.advanced_by))
                        .unique()
                        .filter(|&pos| cache[pos.0][pos.1].is_none())
                        .collect();

                    if missing_deps.is_empty() {
                        let result = backpointers.iter()
                            .map(|&BackPointer { prev_item, advanced_by }| {
                                let child = advanced_by.and_then(|(s, t)| cache[s][t])
                                    .unwrap_or(0);

                                cache[prev_item.0][prev_item.1].unwrap() + child
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

    fn symbol(input: &str) -> IResult<&str, String> {
        map(
            alt((
                recognize(pair(upper_char, many0(lower_char))),
                recognize(character('e'))
            )),
            |s: &str| s.to_owned()
        )(input)
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
                let start_rule = Rule { from: "S0".to_owned(), to: vec!["e".to_owned()] };
                Grammar::new(start_rule, rules)
            }
        )(input)
    }

    pub fn input(input: &str) -> IResult<&str, (Grammar, Vec<String>)> {
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
                    let mut new: Vec<String> = Vec::new();
                    new.extend(target[..i].iter().cloned());
                    new.extend(rule.to.iter().cloned());
                    new.extend(target[i+1..].iter().cloned());
                    seen.insert(new);
                }
            }
        }
    }
    seen.len()
}

fn part2(input: &str) -> usize {
    let (grammar, target) = parse::input(input).unwrap().1;

    earley::min_parse_tree(&grammar, &target) - 1
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