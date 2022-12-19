use core::panic;
use rayon::prelude::*;
use regex::Regex;
use std::{fs, time::Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Blueprint {
    ore_bot_cost: u32,
    clay_bot_cost: u32,
    obsidian_bot_cost_ore: u32,
    obsidian_bot_cost_clay: u32,
    geode_bot_cost_ore: u32,
    geode_bot_cost_obsidian: u32,
}

fn parse_input(path: &str) -> Vec<Blueprint> {
    let mut blueprints = vec![];

    let re = Regex::new(r"^Blueprint \d+: Each ore robot costs (?P<ore_bot_cost>\d+) ore. Each clay robot costs (?P<clay_bot_cost>\d+) ore. Each obsidian robot costs (?P<obsidian_bot_ore_cost>\d+) ore and (?P<obsidian_bot_clay_cost>\d+) clay. Each geode robot costs (?P<geode_bot_ore_cost>\d+) ore and (?P<geode_bot_obsidian_cost>\d+) obsidian.$").unwrap();
    for line in fs::read_to_string(path).unwrap().trim_end().split("\n") {
        let vals = re.captures(line).unwrap();
        let blueprint = Blueprint {
            ore_bot_cost: vals["ore_bot_cost"].parse().unwrap(),
            clay_bot_cost: vals["clay_bot_cost"].parse().unwrap(),
            obsidian_bot_cost_ore: vals["obsidian_bot_ore_cost"].parse().unwrap(),
            obsidian_bot_cost_clay: vals["obsidian_bot_clay_cost"].parse().unwrap(),
            geode_bot_cost_ore: vals["geode_bot_ore_cost"].parse().unwrap(),
            geode_bot_cost_obsidian: vals["geode_bot_obsidian_cost"].parse().unwrap(),
        };

        blueprints.push(blueprint);
    }

    blueprints
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct State {
    minute: u32,
    // Resources
    ore: u32,
    clay: u32,
    obsidian: u32,
    // Bots
    ore_bots: u32,
    clay_bots: u32,
    obsidian_bots: u32,
    geode_bots: u32,
    // Target
    geodes: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    BuildGeodeBot,
    BuildObsidianBot,
    BuildClayBot,
    BuildOreBot,
}

use Action::*;

impl State {
    fn advance(&mut self, blueprint: &Blueprint, action: Option<Action>) {
        // commit resources for building a new bot
        if let Some(action) = action {
            match action {
                BuildOreBot => self.ore -= blueprint.ore_bot_cost,
                BuildClayBot => self.ore -= blueprint.clay_bot_cost,
                BuildObsidianBot => {
                    self.ore -= blueprint.obsidian_bot_cost_ore;
                    self.clay -= blueprint.obsidian_bot_cost_clay;
                }
                BuildGeodeBot => {
                    self.ore -= blueprint.geode_bot_cost_ore;
                    self.obsidian -= blueprint.geode_bot_cost_obsidian;
                }
            }
        }

        // generate resources and target
        self.ore += self.ore_bots;
        self.clay += self.clay_bots;
        self.obsidian += self.obsidian_bots;
        self.geodes += self.geode_bots;

        // add the newly built bot
        if let Some(action) = action {
            match action {
                BuildGeodeBot => self.geode_bots += 1,
                BuildObsidianBot => self.obsidian_bots += 1,
                BuildClayBot => self.clay_bots += 1,
                BuildOreBot => self.ore_bots += 1,
            }
        }

        // advance time
        self.minute += 1;
    }

    // we want to always prefer to build the later bots => thus we order them before the lower level bots
    //
    // we also prune based on time, i.e. we won't build a geode robot on minute 24, since it won't produce
    // any open geodes, obsidian robot on the minute 23, because even though it will produce obsidian,
    // there won't be enough time to produce new geode bots => more open geodes, etc.
    //
    // additionally, we prune based on the number of robots we already have: if it's more than the resource
    // requirement for the next robort type, we don't produce more of those.
    //
    // we also cap the number of ore robots at 4
    fn possible_actions(&self, blueprint: &Blueprint, max_minutes: u32) -> Vec<Action> {
        let mut actions = vec![];

        if self.ore >= blueprint.geode_bot_cost_ore
            && self.obsidian >= blueprint.geode_bot_cost_obsidian
            && self.minute < max_minutes - 1
        {
            actions.push(BuildGeodeBot);
        }

        if self.ore >= blueprint.obsidian_bot_cost_ore
            && self.clay >= blueprint.obsidian_bot_cost_clay
            && self.minute < max_minutes - 3
            && self.obsidian_bots < blueprint.geode_bot_cost_obsidian
        {
            actions.push(BuildObsidianBot);
        }

        if self.ore >= blueprint.clay_bot_cost
            && self.minute < max_minutes - 5
            && self.clay_bots < blueprint.obsidian_bot_cost_clay
        {
            actions.push(BuildClayBot);
        }

        if self.ore >= blueprint.ore_bot_cost
            && self.minute < max_minutes - 7
            && self.ore_bots < blueprint.clay_bot_cost + blueprint.obsidian_bot_cost_ore + blueprint.geode_bot_cost_ore
            && self.ore_bots < 4
        {
            actions.push(BuildOreBot);
        }

        actions
    }
}

fn evaluate(blueprint: Blueprint, max_minutes: u32) -> u32 {
    let mut best_open_geodes = 0;

    let mut start_state = State::default();
    start_state.ore_bots = 1;

    let mut states = start_state
        .possible_actions(&blueprint, max_minutes)
        .into_iter()
        .map(|action| (start_state.clone(), Some(action)))
        .collect::<Vec<_>>();
    states.push((start_state.clone(), None));

    while let Some((mut state, action)) = states.pop() {
        state.advance(&blueprint, action);

        if state.minute == max_minutes {
            if state.geodes > best_open_geodes {
                best_open_geodes = state.geodes;
            }
        } else {
            // prune based on the possible achieavable score compared to the best so far
            let remaining_time = max_minutes - state.minute;
            if remaining_time <= 8 {
                // each of those is previous number + self, i.e. 3 => 6 implies 4 => 4 + 6 = 10.
                let possible_to_open_additional_geodes_with_new_bots = match remaining_time {
                    1 => 1,
                    2 => 3,
                    3 => 6,
                    4 => 10,
                    5 => 15,
                    6 => 21,
                    7 => 28,
                    8 => 36,
                    _ => panic!("cannot calculate for {remaining_time}"),
                };
                if state.geodes + remaining_time * state.geode_bots + possible_to_open_additional_geodes_with_new_bots
                    < best_open_geodes
                {
                    continue;
                }
            }

            match state.possible_actions(&blueprint, max_minutes)[..] {
                [best_bot_to_build, second_best_bot_to_build, ..] => {
                    states.push((state.clone(), None));
                    states.push((state.clone(), Some(second_best_bot_to_build)));
                    states.push((state.clone(), Some(best_bot_to_build)));
                }
                [best_bot_to_build, ..] => {
                    states.push((state.clone(), None));
                    states.push((state.clone(), Some(best_bot_to_build)));
                }
                [] => states.push((state.clone(), None)),
            }
        }
    }

    best_open_geodes
}

fn p1(input: Vec<Blueprint>) -> u32 {
    input
        .par_iter()
        .map(|&blueprint| evaluate(blueprint, 24))
        .enumerate()
        .map(|(id, x)| (id + 1) as u32 * x as u32)
        .sum()
}

fn p2(input: Vec<Blueprint>) -> u32 {
    input.par_iter().take(3).map(|&blueprint| evaluate(blueprint, 32)).map(|x| x as u32).product()
}

// p1 ans: 1703 [1384 ms]
// p2 ans: 5301 [50612 ms]
fn main() {
    let input = parse_input("../inputs/d19");
    let timer = Instant::now();
    let p1_ans = p1(input);
    let elapsed = timer.elapsed().as_millis();
    println!("p1 ans: {p1_ans} [{elapsed} ms]");
    assert_eq!(p1_ans, 1703);

    let input = parse_input("../inputs/d19");
    let timer = Instant::now();
    let p2_ans = p2(input);
    let elapsed = timer.elapsed().as_millis();
    println!("p2 ans: {p2_ans} [{elapsed} ms]");
    assert_eq!(p2_ans, 5301);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_test() {
        let test_input = parse_input("../inputs/d19_test");
        assert_eq!(test_input.len(), 2);
        assert_eq!(
            test_input[0],
            Blueprint {
                ore_bot_cost: 4,
                clay_bot_cost: 2,
                obsidian_bot_cost_ore: 3,
                obsidian_bot_cost_clay: 14,
                geode_bot_cost_ore: 2,
                geode_bot_cost_obsidian: 7,
            }
        );

        let test_input = parse_input("../inputs/d19");
        assert_eq!(test_input.len(), 30);
    }

    #[test]
    fn state_test() {
        let test_input = parse_input("../inputs/d19_test");
        let blueprint = test_input[0];
        let mut state = State::default();
        state.ore_bots = 1;

        assert_eq!(state.possible_actions(&blueprint, 24), vec![]);
        state.advance(&blueprint, None);
        assert_eq!(
            state,
            State {
                minute: 1,
                ore: 1,
                clay: 0,
                obsidian: 0,
                ore_bots: 1,
                clay_bots: 0,
                obsidian_bots: 0,
                geode_bots: 0,
                geodes: 0
            }
        );

        assert_eq!(state.possible_actions(&blueprint, 24), vec![]);
        state.advance(&blueprint, None);
        assert_eq!(
            state,
            State {
                minute: 2,
                ore: 2,
                clay: 0,
                obsidian: 0,
                ore_bots: 1,
                clay_bots: 0,
                obsidian_bots: 0,
                geode_bots: 0,
                geodes: 0
            }
        );

        assert_eq!(state.possible_actions(&blueprint, 24), vec![BuildClayBot]);
        state.advance(&blueprint, Some(BuildClayBot));
        assert_eq!(
            state,
            State {
                minute: 3,
                ore: 1,
                clay: 0,
                obsidian: 0,
                ore_bots: 1,
                clay_bots: 1,
                obsidian_bots: 0,
                geode_bots: 0,
                geodes: 0
            }
        );

        assert_eq!(state.possible_actions(&blueprint, 24), vec![]);
        state.advance(&blueprint, None);
        assert_eq!(
            state,
            State {
                minute: 4,
                ore: 2,
                clay: 1,
                obsidian: 0,
                ore_bots: 1,
                clay_bots: 1,
                obsidian_bots: 0,
                geode_bots: 0,
                geodes: 0
            }
        );

        let mut state = State {
            minute: 10,
            ore: 4,
            clay: 15,
            obsidian: 0,
            ore_bots: 1,
            clay_bots: 3,
            obsidian_bots: 0,
            geode_bots: 0,
            geodes: 0,
        };

        assert_eq!(state.possible_actions(&blueprint, 24), vec![BuildObsidianBot, BuildClayBot, BuildOreBot]);
        state.advance(&blueprint, Some(BuildObsidianBot));
        assert_eq!(
            state,
            State {
                minute: 11,
                ore: 2,
                clay: 4,
                obsidian: 0,
                ore_bots: 1,
                clay_bots: 3,
                obsidian_bots: 1,
                geode_bots: 0,
                geodes: 0
            }
        );

        assert_eq!(state.possible_actions(&blueprint, 24), vec![BuildClayBot]);
        state.advance(&blueprint, Some(BuildClayBot));
        assert_eq!(
            state,
            State {
                minute: 12,
                ore: 1,
                clay: 7,
                obsidian: 1,
                ore_bots: 1,
                clay_bots: 4,
                obsidian_bots: 1,
                geode_bots: 0,
                geodes: 0
            }
        );

        let mut state = State {
            minute: 17,
            ore: 3,
            clay: 13,
            obsidian: 8,
            ore_bots: 1,
            clay_bots: 4,
            obsidian_bots: 2,
            geode_bots: 0,
            geodes: 0,
        };

        assert_eq!(state.possible_actions(&blueprint, 24), vec![BuildGeodeBot, BuildClayBot]);
        state.advance(&blueprint, Some(BuildGeodeBot));
        assert_eq!(
            state,
            State {
                minute: 18,
                ore: 2,
                clay: 17,
                obsidian: 3,
                ore_bots: 1,
                clay_bots: 4,
                obsidian_bots: 2,
                geode_bots: 1,
                geodes: 0,
            }
        );

        assert_eq!(state.possible_actions(&blueprint, 24), vec![BuildClayBot]);
        state.advance(&blueprint, None);
        assert_eq!(
            state,
            State {
                minute: 19,
                ore: 3,
                clay: 21,
                obsidian: 5,
                ore_bots: 1,
                clay_bots: 4,
                obsidian_bots: 2,
                geode_bots: 1,
                geodes: 1,
            }
        );
    }

    #[test]
    fn p1_test() {
        let test_input = parse_input("../inputs/d19_test");

        let p1_test_b1 = evaluate(test_input[0], 24);
        assert_eq!(p1_test_b1, 9);
        let p1_test_b2 = evaluate(test_input[1], 24);
        assert_eq!(p1_test_b2, 12);

        let p1_test_ans = p1(test_input);
        assert_eq!(p1_test_ans, 33);
    }

    #[test]
    fn p2_test() {
        let test_input = parse_input("../inputs/d19_test");

        let p2_test_ans = p2(test_input);
        assert_eq!(p2_test_ans, 3472);
    }
}
