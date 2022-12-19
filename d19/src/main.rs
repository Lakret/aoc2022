use regex::Regex;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Blueprint {
    id: i32,
    ore_bot_cost: i32,
    clay_bot_cost: i32,
    obsidian_bot_cost_ore: i32,
    obsidian_bot_cost_clay: i32,
    geode_bot_cost_ore: i32,
    geode_bot_cost_obsidian: i32,
}

// Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.

fn parse_input(path: &str) -> Vec<Blueprint> {
    let mut blueprints = vec![];

    let re = Regex::new(r"^Blueprint (?P<id>\d+): Each ore robot costs (?P<ore_bot_cost>\d+) ore. Each clay robot costs (?P<clay_bot_cost>\d+) ore. Each obsidian robot costs (?P<obsidian_bot_ore_cost>\d+) ore and (?P<obsidian_bot_clay_cost>\d+) clay. Each geode robot costs (?P<geode_bot_ore_cost>\d+) ore and (?P<geode_bot_obsidian_cost>\d+) obsidian.$").unwrap();
    for line in fs::read_to_string(path).unwrap().trim_end().split("\n") {
        let vals = re.captures(line).unwrap();
        let blueprint = Blueprint {
            id: vals["id"].parse().unwrap(),
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
    minute: i32,
    // Resources
    ore: i32,
    clay: i32,
    obsidian: i32,
    // Bots
    ore_bots: i32,
    clay_bots: i32,
    obsidian_bots: i32,
    geode_bots: i32,
    // Target
    geodes: i32,
}

// we always open geods when we can
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    BuildGeodeBot,
    BuildObsidianBot,
    BuildClayBot,
    BuildOreBot,
}

// Each ore robot costs 4 ore.
// Each clay robot costs 2 ore.
// Each obsidian robot costs 3 ore and 14 clay.
// Each geode robot costs 2 ore and 7 obsidian.

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

    // always prefer to build the later bots
    fn possible_actions(&self, blueprint: &Blueprint) -> Vec<Action> {
        let mut actions = vec![];

        if self.ore >= blueprint.geode_bot_cost_ore && self.obsidian >= blueprint.geode_bot_cost_obsidian {
            actions.push(BuildGeodeBot);
        }

        if self.ore >= blueprint.obsidian_bot_cost_ore && self.clay >= blueprint.obsidian_bot_cost_clay {
            actions.push(BuildObsidianBot);
        }

        if self.ore >= blueprint.clay_bot_cost {
            actions.push(BuildClayBot);
        }

        if self.ore >= blueprint.ore_bot_cost {
            actions.push(BuildOreBot);
        }

        actions
    }
}

fn evaluate(blueprint: Blueprint) -> i32 {
    let max_minutes = 24;
    let mut start_state = State::default();
    start_state.ore_bots = 1;

    todo!()
}

fn main() {
    println!("Hello, world!");
}

// doesn't make sense to build a geode robot at the last minute
// only one robot can be built per minute

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
                id: 1,
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

        assert_eq!(state.possible_actions(&blueprint), vec![]);
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

        assert_eq!(state.possible_actions(&blueprint), vec![]);
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

        assert_eq!(state.possible_actions(&blueprint), vec![BuildClayBot]);
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

        assert_eq!(state.possible_actions(&blueprint), vec![]);
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

        assert_eq!(state.possible_actions(&blueprint), vec![BuildObsidianBot, BuildClayBot, BuildOreBot]);
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

        assert_eq!(state.possible_actions(&blueprint), vec![BuildClayBot]);
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

        assert_eq!(state.possible_actions(&blueprint), vec![BuildGeodeBot, BuildClayBot]);
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

        assert_eq!(state.possible_actions(&blueprint), vec![BuildClayBot]);
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
}
