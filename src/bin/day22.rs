use std::cmp::min;
use std::collections::{HashSet, VecDeque};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{map, map_res};
use nom::IResult;
use nom::sequence::{preceded, separated_pair};
use adventofcode2015::build_main;

#[derive(Debug, Copy, Clone)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge
}
use Spell::*;
use crate::GameState::{BossWins, PlayerWins};

impl Spell {
    fn mana_cost(&self) -> usize {
        match self {
            MagicMissile => 53,
            Drain => 73,
            Shield => 113,
            Poison => 173,
            Recharge => 229
        }
    }
}

const SPELLS: [Spell; 5] = [MagicMissile, Drain, Shield, Poison, Recharge];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Stats {
    player_hp: usize,
    player_mana: usize,
    mana_spent: usize,
    boss_hp: usize,
    boss_damage: usize,
    poison_turns: usize,
    recharge_turns: usize,
    shield_turns: usize
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Live(Stats),
    PlayerWins(usize),
    BossWins(usize)
}
use GameState::*;

impl GameState {
    fn next_state(&self, spell: Spell, hard_mode: bool) -> Option<GameState> {
        match *self {
            PlayerWins(_) => None,
            BossWins(_) => None,
            Live(stats) => {
                let mut result = stats.clone();

                if hard_mode {
                    if result.player_hp == 1 {
                        return Some(BossWins(result.mana_spent));
                    }
                    result.player_hp -= 1;
                }

                if result.poison_turns > 0 {
                    if result.boss_hp <= 3 {
                        return Some(PlayerWins(result.mana_spent));
                    }
                    result.boss_hp -= 3;
                    result.poison_turns -= 1;
                }

                if result.recharge_turns > 0 {
                    result.player_mana += 101;
                    result.recharge_turns -= 1;
                }

                if result.shield_turns > 0 {
                    result.shield_turns -= 1;
                }

                let mana_cost = spell.mana_cost();

                if result.player_mana < mana_cost {
                    return None
                }

                result.player_mana -= mana_cost;
                result.mana_spent += mana_cost;

                match spell {
                    MagicMissile => {
                        result.boss_hp = if result.boss_hp < 4 { 0 } else { result.boss_hp - 4 };
                    },
                    Drain => {
                        result.boss_hp = if result.boss_hp < 2 { 0 } else { result.boss_hp - 2 };
                        result.player_hp += 2;
                    },
                    Shield => {
                        if result.shield_turns > 0 {
                            return None
                        }
                        result.shield_turns = 6;
                    },
                    Poison => {
                        if result.poison_turns > 0 {
                            return None
                        }
                        result.poison_turns = 6;
                    }
                    Recharge => {
                        if result.recharge_turns > 0 {
                            return None
                        }
                        result.recharge_turns = 5;
                    }
                }

                if result.boss_hp == 0 {
                    return Some(PlayerWins(result.mana_spent));
                }

                if result.poison_turns > 0 {
                    if result.boss_hp <= 3 {
                        return Some(PlayerWins(result.mana_spent));
                    }
                    result.boss_hp -= 3;
                    result.poison_turns -= 1;
                }

                if result.recharge_turns > 0 {
                    result.player_mana += 101;
                    result.recharge_turns -= 1;
                }

                let player_armor = if result.shield_turns > 0 {
                    result.shield_turns -= 1;
                    7
                } else { 0 };

                let damage = if stats.boss_damage > player_armor {
                    stats.boss_damage - player_armor
                } else { 1 };

                if result.player_hp <= damage {
                    return Some(BossWins(result.mana_spent));
                }

                result.player_hp -= damage;

                Some(Live(result))
            }
        }
    }

    fn next_states(&self, hard_mode: bool) -> Vec<GameState> {
        SPELLS.iter()
            .filter_map(|&spell| self.next_state(spell, hard_mode))
            .collect()
    }
}

fn parse_input(input: &str) -> IResult<&str, Stats> {
    map(
        separated_pair(
            preceded(tag("Hit Points: "), map_res(digit1, |s: &str| s.parse::<usize>())),
            newline,
            preceded(tag("Damage: "), map_res(digit1, |s: &str| s.parse::<usize>()))
        ),
        |(boss_hp, boss_damage)| Stats {
            player_hp: 50,
            player_mana: 500,
            mana_spent: 0,
            boss_hp,
            boss_damage,
            poison_turns: 0,
            recharge_turns: 0,
            shield_turns: 0
        }
    )(input)
}

fn min_mana_to_win(initial_state: GameState, hard_mode: bool) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut best = usize::MAX;

    queue.push_back(initial_state);
    seen.insert(initial_state);

    while let Some(state) = queue.pop_front() {
        match state {
            PlayerWins(mana_spent) => best = min(best, mana_spent),
            BossWins(_) => { continue; },
            Live(stats) => {
                if stats.mana_spent >= best {
                    continue;
                }

                for next_state in state.next_states(hard_mode) {
                    if seen.insert(next_state) {
                        queue.push_back(next_state);
                    }
                }
            }
        }
    }

    best
}

fn part1(input: &str) -> usize {
    let initial_stats = parse_input(input).unwrap().1;
    let initial_state = Live(initial_stats);
    min_mana_to_win(initial_state, false)
}

fn part2(input: &str) -> usize {
    let initial_stats = parse_input(input).unwrap().1;
    let initial_state = Live(initial_stats);
    min_mana_to_win(initial_state, true)
}

build_main!("day22.txt", "Part 1" => part1, "Part 2" => part2);