use std::collections::{HashMap, HashSet};
use adventofcode2015::build_main;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Rule<'a> {
    from: &'a str,
    to: Vec<&'a str>
}

struct Grammar<'a> {
    pub rules: Vec<Rule<'a>>,
    pub start_rule_id: usize,
    pub rule_ids_by_from: HashMap<&'a str, Vec<usize>>
}

impl<'a> Grammar<'a> {
    fn new(start_rule_id: usize, rules: Vec<Rule<'a>>) -> Grammar<'a> {
        let mut rule_ids_by_from: HashMap<&'a str, Vec<usize>> = HashMap::new();
        for (id, rule) in rules.iter().enumerate() {
            let from = rule.from;
            rule_ids_by_from.entry(from).or_default().push(id);
        }

        Grammar { start_rule_id, rules , rule_ids_by_from }
    }
}

mod earley {
    use std::ops::Index;
    use itertools::{chain, Itertools};
    use crate::Grammar;

    #[derive(Copy, Clone)]
    struct BackPointer {
        prev_item: (usize, usize),
        advanced_by: Option<(usize, usize)>
    }

    #[derive(Debug, Eq, PartialEq, Clone, Copy)]
    struct EarleyItem {
        rule_id: usize,
        rule_state: usize,
        parse_start: usize
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

        fn build(grammar: &Grammar, target: &[&str]) -> ParseTable {
            let mut table: ParseTable = ParseTable::with_rows(target.len() + 1);

            table.insert(
                EarleyItem { rule_id: grammar.start_rule_id, rule_state: 0, parse_start: 0 },
                0,
                None
            );

            for cur_parse_end in 0..=target.len() {
                let mut cur_item_index = 0;
                while cur_item_index < table[cur_parse_end].len() {
                    let cur_item = table[cur_parse_end][cur_item_index].0;
                    let cur_rule = &grammar.rules[cur_item.rule_id];

                    if cur_item.rule_state == cur_rule.to.len() {
                        // We've completed this item. If any items were waiting for this, we can
                        // advance them one step.
                        let mut j = 0;
                        while j < table[cur_item.parse_start].len() {
                            let next_item = &table[cur_item.parse_start][j].0;
                            let next_rule = &grammar.rules[next_item.rule_id];
                            let next_is_complete = next_item.rule_state == next_rule.to.len();

                            if !next_is_complete &&
                                next_rule.to[next_item.rule_state] == cur_rule.from {
                                let new_item = EarleyItem {
                                    rule_id: next_item.rule_id,
                                    rule_state: next_item.rule_state + 1,
                                    parse_start: next_item.parse_start
                                };

                                table.insert(
                                    new_item,
                                    cur_parse_end,
                                    Some(BackPointer {
                                        prev_item: (cur_item.parse_start, j),
                                        advanced_by: Some((cur_parse_end, cur_item_index))
                                    })
                                );
                            }

                            j += 1;
                        }
                    }
                    else {
                        let cur_item = &table[cur_parse_end][cur_item_index].0;
                        let cur_rule = &grammar.rules[cur_item.rule_id];
                        let cur_rule_symb = &cur_rule.to[cur_item.rule_state];

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
                                    rule_id: cur_item.rule_id,
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
                        if let Some(rule_ids) = grammar.rule_ids_by_from.get(cur_rule_symb) {
                            for &rule_id in rule_ids {
                                table.insert(
                                    EarleyItem {
                                        rule_id,
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
                .find(|(_, (item, _))| {
                    let rule = &grammar.rules[item.rule_id];
                    item.rule_id == grammar.start_rule_id && item.rule_state == rule.to.len()
                })
                .map(|(i, _)| (k, i));

            table.completed_start_rule = completed_start;

            table
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

    pub fn min_rules(grammar: &Grammar, target: &[&str]) -> Option<usize> {
        let table = ParseTable::build(grammar, target);
        let mut cache: Vec<Vec<Option<usize>>> = table.iter().map(
            |s| s.iter().map(|_| None).collect()
        ).collect();

        let (k0, i0) = table.completed_start_rule?;

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

        Some(cache[k0][i0].unwrap())
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
            |mut rules| {
                let start_rule = Rule { from: "S0", to: vec!["e"] };
                rules.push(start_rule);
                Grammar::new(rules.len() - 1, rules)
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

    for rule in grammar.rules {

        for i in 0..target.len() {
            if target[i] == rule.from {
                let mut new: Vec<&str> = Vec::new();
                new.extend(target[..i].iter());
                new.extend(rule.to.iter());
                new.extend(target[i+1..].iter());
                seen.insert(new);
            }
        }
    }
    seen.len()
}

fn part2(input: &str) -> usize {
    let (grammar, target) = parse::input(input).unwrap().1;

    earley::min_rules(&grammar, &target).unwrap() - 1
}


build_main!("day19.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "e => H
e => O
H => HO
H => OH
O => HH

HOH";

    const TEST_INPUT2: &str = "e => H
e => O
H => HO
H => OH
O => HH

HOHOHO";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT1), 4);
        assert_eq!(part1(TEST_INPUT2), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT1), 3);
        assert_eq!(part2(TEST_INPUT2), 6);
    }
}