use regex::Regex;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Ore(i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Clay(i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Obsidian(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Blueprint {
    id: i32,
    ore_bot_cost: Ore,
    clay_bot_cost: Ore,
    obsidian_bot_cost: (Ore, Clay),
    geode_bot_cost: (Ore, Obsidian),
}

// Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.

fn parse_input(path: &str) -> Vec<Blueprint> {
    let mut blueprints = vec![];

    let re = Regex::new(r"^Blueprint (?P<id>\d+): Each ore robot costs (?P<ore_bot_cost>\d+) ore. Each clay robot costs (?P<clay_bot_cost>\d+) ore. Each obsidian robot costs (?P<obsidian_bot_ore_cost>\d+) ore and (?P<obsidian_bot_clay_cost>\d+) clay. Each geode robot costs (?P<geode_bot_ore_cost>\d+) ore and (?P<geode_bot_obsidian_cost>\d+) obsidian.$").unwrap();
    for line in fs::read_to_string(path).unwrap().trim_end().split("\n") {
        let vals = re.captures(line).unwrap();
        let blueprint = Blueprint {
            id: vals["id"].parse().unwrap(),
            ore_bot_cost: Ore(vals["ore_bot_cost"].parse().unwrap()),
            clay_bot_cost: Ore(vals["clay_bot_cost"].parse().unwrap()),
            obsidian_bot_cost: (
                Ore(vals["obsidian_bot_ore_cost"].parse().unwrap()),
                Clay(vals["obsidian_bot_clay_cost"].parse().unwrap()),
            ),
            geode_bot_cost: (
                Ore(vals["geode_bot_ore_cost"].parse().unwrap()),
                Obsidian(vals["geode_bot_obsidian_cost"].parse().unwrap()),
            ),
        };

        blueprints.push(blueprint);
    }

    blueprints
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
                ore_bot_cost: Ore(4),
                clay_bot_cost: Ore(2),
                obsidian_bot_cost: (Ore(3), Clay(14)),
                geode_bot_cost: (Ore(2), Obsidian(7))
            }
        );
        // TODO:
    }
}
