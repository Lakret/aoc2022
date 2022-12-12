use std::collections::VecDeque;
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
enum Op {
    #[default]
    Plus,
    Mul,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Operation {
    op: Op,
    arg1: Option<u64>,
    arg2: Option<u64>,
}

impl Operation {
    fn eval(&self, old: u64) -> u64 {
        let arg1 = self.arg1.unwrap_or(old);
        let arg2 = self.arg2.unwrap_or(old);
        match self.op {
            Op::Plus => arg1 + arg2,
            Op::Mul => arg1 * arg2,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Monkey {
    inventory: VecDeque<u64>,
    test_divisible_by: u64,
    true_throw_destination: usize,
    false_throw_destination: usize,
    op: Operation,
}

#[derive(Debug)]
pub struct State {
    monkeys: Vec<Monkey>,
    stats: Vec<u64>,
}

impl State {
    pub fn new(monkeys: Vec<Monkey>) -> State {
        let stats = vec![0; monkeys.len()];
        State { monkeys, stats }
    }

    pub fn inventory(&self, monkey_id: usize) -> &VecDeque<u64> {
        &self.monkeys[monkey_id].inventory
    }

    pub fn round(&mut self, divide_by_three: bool) {
        for monkey_id in 0..self.monkeys.len() {
            self.inspect_all(monkey_id, divide_by_three);
        }
    }

    pub fn monkey_business(&self) -> u64 {
        let mut stats_ascending = self.stats.clone();
        stats_ascending.sort();
        stats_ascending[stats_ascending.len() - 1] * stats_ascending[stats_ascending.len() - 2]
    }

    fn inspect_all(&mut self, monkey_id: usize, divide_by_three: bool) {
        loop {
            if !(self.inspect(monkey_id, divide_by_three)) {
                break;
            }
        }
    }

    fn inspect(&mut self, monkey_id: usize, divide_by_three: bool) -> bool {
        let lcm = self.lcm();

        let monkey = &mut self.monkeys[monkey_id];
        if let Some(worry_level) = monkey.inventory.pop_front() {
            let worry_level = monkey.op.eval(worry_level);
            let worry_level =
                if divide_by_three { (worry_level as f64 / 3.0).floor() as u64 } else { worry_level % lcm };

            let throw_to = if (&worry_level % monkey.test_divisible_by) == 0 {
                monkey.true_throw_destination
            } else {
                monkey.false_throw_destination
            };
            self.monkeys[throw_to].inventory.push_back(worry_level);

            self.stats[monkey_id] += 1;
            true
        } else {
            false
        }
    }

    fn lcm(&self) -> u64 {
        self.monkeys.iter().map(|m| m.test_divisible_by).product()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExpectedToken {
    MonkeyId,
    InventoryList,
    Operation,
    TestCriterion,
    TrueThrowDestination,
    FalseThrowDestination,
}

fn parse_input(path: &str) -> State {
    use ExpectedToken::*;

    let mut state = Vec::new();
    let mut expected_token = MonkeyId;
    let mut curr = Monkey::default();

    for line in fs::read_to_string(path).unwrap().trim_end().split("\n") {
        let line = line.trim();

        match expected_token {
            MonkeyId if line.is_empty() => (),
            MonkeyId => {
                if !line.starts_with("Monkey ") {
                    panic!("expected monkey id line, but got: {line}");
                }
                expected_token = InventoryList;
            }
            InventoryList => {
                curr.inventory = line
                    .strip_prefix("Starting items: ")
                    .unwrap()
                    .split(", ")
                    .map(|worry| worry.parse().unwrap())
                    .collect();
                expected_token = Operation;
            }
            Operation => {
                let line = line.strip_prefix("Operation: new = ").unwrap();
                curr.op = parse_operation(line);
                expected_token = TestCriterion;
            }
            TestCriterion => {
                curr.test_divisible_by = line.strip_prefix("Test: divisible by ").unwrap().parse().unwrap();
                expected_token = TrueThrowDestination;
            }
            TrueThrowDestination => {
                curr.true_throw_destination = line.strip_prefix("If true: throw to monkey ").unwrap().parse().unwrap();
                expected_token = FalseThrowDestination;
            }
            FalseThrowDestination => {
                curr.false_throw_destination =
                    line.strip_prefix("If false: throw to monkey ").unwrap().parse().unwrap();
                expected_token = MonkeyId;

                state.push(curr);
                curr = Monkey::default();
            }
        }
    }

    State::new(state)
}

fn parse_operation(line: &str) -> Operation {
    let mut arg1 = None;
    let mut arg2 = None;
    let mut op = Op::Plus;

    let mut second_arg = false;
    for token in line.split_ascii_whitespace() {
        match token {
            "old" if second_arg => arg2 = None,
            "old" => {
                arg1 = None;
                second_arg = true;
            }
            "+" => op = Op::Plus,
            "*" => op = Op::Mul,
            literal => {
                let arg = literal.parse().unwrap();
                if second_arg {
                    arg2 = Some(arg);
                } else {
                    arg1 = Some(arg);
                }
            }
        }
    }

    Operation { op, arg1, arg2 }
}

fn p1(state: &mut State) -> u64 {
    for _ in 0..20 {
        state.round(true);
    }

    state.monkey_business()
}

fn p2(state: &mut State) -> u64 {
    for _ in 0..10000 {
        state.round(false);
    }

    state.monkey_business()
}

fn main() {
    let mut state = parse_input("../inputs/d11");
    let p1_ans = p1(&mut state);
    println!("P1: {p1_ans}.");

    let mut state = parse_input("../inputs/d11");
    let p2_ans = p2(&mut state);
    println!("P2: {p2_ans}.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_test() {
        let State { monkeys, .. } = parse_input("../inputs/d11_test");
        assert_eq!(monkeys.len(), 4);
        assert_eq!(
            monkeys[0],
            Monkey {
                inventory: VecDeque::from([79, 98]),
                test_divisible_by: 23,
                true_throw_destination: 2,
                false_throw_destination: 3,
                op: Operation { op: Op::Mul, arg1: None, arg2: Some(19) },
            }
        );
        assert_eq!(
            monkeys[3],
            Monkey {
                inventory: VecDeque::from([74]),
                test_divisible_by: 17,
                true_throw_destination: 0,
                false_throw_destination: 1,
                op: Operation { op: Op::Plus, arg1: None, arg2: Some(3) },
            }
        );
        assert_eq!(monkeys[2].op, Operation { op: Op::Mul, arg1: None, arg2: None });

        let State { monkeys, .. } = parse_input("../inputs/d11");
        assert_eq!(monkeys.len(), 8);
        assert_eq!(
            monkeys[6],
            Monkey {
                inventory: VecDeque::from([99, 90, 84, 50]),
                test_divisible_by: 17,
                true_throw_destination: 7,
                false_throw_destination: 1,
                op: Operation { op: Op::Mul, arg1: None, arg2: None }
            }
        );
    }

    #[test]
    fn round_test() {
        let mut test_state = parse_input("../inputs/d11_test");

        test_state.round(true);
        assert_eq!(test_state.inventory(0), &[20, 23, 27, 26]);
        assert_eq!(test_state.inventory(1), &[2080, 25, 167, 207, 401, 1046]);
        assert_eq!(test_state.inventory(2), &[]);
        assert_eq!(test_state.inventory(3), &[]);

        for _ in 0..19 {
            test_state.round(true);
        }
        assert_eq!(test_state.inventory(0), &[10, 12, 14, 26, 34]);
        assert_eq!(test_state.inventory(1), &[245, 93, 53, 199, 115]);
        assert_eq!(test_state.inventory(2), &[]);
        assert_eq!(test_state.inventory(3), &[]);
    }

    #[test]
    fn p1_test() {
        let mut test_state = parse_input("../inputs/d11_test");
        assert_eq!(p1(&mut test_state), 10605);

        let mut test_state = parse_input("../inputs/d11");
        assert_eq!(p1(&mut test_state), 54752);
    }

    #[test]
    fn p2_test() {
        let mut test_state = parse_input("../inputs/d11_test");
        assert_eq!(p2(&mut test_state), 2713310158);

        let mut test_state = parse_input("../inputs/d11");
        assert_eq!(p2(&mut test_state), 13606755504);
    }
}
