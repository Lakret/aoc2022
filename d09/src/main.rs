use std::collections::HashSet;
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Instruction {
    direction: Direction,
    steps: u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coords {
    x: i32,
    y: i32,
}

impl Default for Coords {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Coords {
    fn is_adjacent(self, another: Coords) -> bool {
        (self == another) || (((self.x - another.x).abs() <= 1) && ((self.y - another.y).abs() <= 1))
    }

    fn move_in_direction(self, direction: Direction) -> Coords {
        let (x, y) = match direction {
            Direction::Right => (self.x + 1, self.y),
            Direction::Left => (self.x - 1, self.y),
            Direction::Up => (self.x, self.y + 1),
            Direction::Down => (self.x, self.y - 1),
        };

        Coords { x, y }
    }

    fn follow(self, target: Coords) -> Coords {
        let possible_new_x = if (self.x - target.x).abs() >= 1 {
            if target.x > self.x {
                self.x + 1
            } else {
                self.x - 1
            }
        } else {
            self.x
        };

        let possible_new_y = if (self.y - target.y).abs() >= 1 {
            if target.y > self.y {
                self.y + 1
            } else {
                self.y - 1
            }
        } else {
            self.y
        };

        if possible_new_x != self.x && possible_new_y != self.y {
            Coords { x: possible_new_x, y: possible_new_y }
        } else if target.is_adjacent(Coords { x: possible_new_x, y: self.y }) {
            Coords { x: possible_new_x, y: self.y }
        } else if target.is_adjacent(Coords { x: self.x, y: possible_new_y }) {
            Coords { x: self.x, y: possible_new_y }
        } else {
            Coords { x: possible_new_x, y: possible_new_y }
        }
    }
}

fn parse_input(path: &str) -> Vec<Instruction> {
    let mut directions = vec![];
    for line in fs::read_to_string(path).unwrap().trim_end().split("\n") {
        let items = line.split_ascii_whitespace().collect::<Vec<_>>();
        let direction = match items[0] {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("incorrect input"),
        };
        let steps: u8 = items[1].parse().unwrap();
        directions.push(Instruction { direction, steps });
    }
    directions
}

fn p1(instructions: &Vec<Instruction>) -> usize {
    let mut head_pos = Coords::default();
    let mut tail_pos = Coords::default();
    let mut tail_visited: HashSet<Coords> = HashSet::new();
    tail_visited.insert(tail_pos);

    for &Instruction { direction, steps } in instructions {
        for _ in 0..steps {
            head_pos = head_pos.move_in_direction(direction);
            if !head_pos.is_adjacent(tail_pos) {
                tail_pos = tail_pos.follow(head_pos);
                tail_visited.insert(tail_pos);
            }
        }
    }

    tail_visited.len()
}

fn p2(instructions: &Vec<Instruction>) -> usize {
    let mut positions = vec![Coords::default(); 10];
    let mut tail_visited: HashSet<Coords> = HashSet::new();
    tail_visited.insert(positions[9]);

    for &Instruction { direction, steps } in instructions {
        for _ in 0..steps {
            positions[0] = positions[0].move_in_direction(direction);

            for knot_idx in 1..10 {
                if !positions[knot_idx - 1].is_adjacent(positions[knot_idx]) {
                    positions[knot_idx] = positions[knot_idx].follow(positions[knot_idx - 1]);
                    if knot_idx == 9 {
                        tail_visited.insert(positions[knot_idx]);
                    }
                }
            }
        }
    }

    tail_visited.len()
}

fn main() {
    let instructions = parse_input("../inputs/d09");
    let p1_ans = p1(&instructions);
    println!("P1: {p1_ans}.");

    let p2_ans = p2(&instructions);
    println!("P2: {p2_ans}.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let test_instructions = parse_input("../inputs/d09_test");
        assert_eq!(test_instructions.len(), 8);
        assert_eq!(test_instructions[0], Instruction { steps: 4, direction: Direction::Right });

        let instructions = parse_input("../inputs/d09");
        assert_eq!(instructions.len(), 2000);
    }

    #[test]
    fn test_is_adjacent() {
        assert_eq!(Coords { x: 2, y: 1 }.is_adjacent(Coords { x: 1, y: 1 }), true);
        assert_eq!(Coords { x: 2, y: 1 }.is_adjacent(Coords { x: 2, y: 1 }), true);
        assert_eq!(Coords { x: 1, y: 2 }.is_adjacent(Coords { x: 2, y: 1 }), true);
        assert_eq!(Coords { x: 2, y: 1 }.is_adjacent(Coords { x: 1, y: 1 }), true);

        assert_eq!(Coords { x: 2, y: 1 }.is_adjacent(Coords { x: 1, y: 3 }), false);
    }

    #[test]
    fn test_move_in_direction() {
        let zero = Coords::default();
        assert_eq!(zero.move_in_direction(Direction::Up), Coords { x: 0, y: 1 });
        assert_eq!(zero.move_in_direction(Direction::Down), Coords { x: 0, y: -1 });
        assert_eq!(zero.move_in_direction(Direction::Left), Coords { x: -1, y: 0 });
        assert_eq!(zero.move_in_direction(Direction::Right), Coords { x: 1, y: 0 });
    }

    #[test]
    fn test_move_tail() {
        assert_eq!(Coords { x: 1, y: 1 }.follow(Coords { x: 2, y: 3 }), Coords { x: 2, y: 2 });
        assert_eq!(Coords { x: 1, y: 1 }.follow(Coords { x: 3, y: 2 }), Coords { x: 2, y: 2 });

        assert_eq!(Coords { x: 1, y: 1 }.follow(Coords { x: 3, y: 1 }), Coords { x: 2, y: 1 });
        assert_eq!(Coords { x: 1, y: 3 }.follow(Coords { x: 1, y: 1 }), Coords { x: 1, y: 2 });

        assert_eq!(Coords { x: 1, y: 0 }.follow(Coords { x: 3, y: 0 }), Coords { x: 2, y: 0 });
    }

    #[test]
    fn test_p1() {
        let test_instructions = parse_input("../inputs/d09_test");
        assert_eq!(p1(&test_instructions), 13);

        let instructions = parse_input("../inputs/d09");
        assert_eq!(p1(&instructions), 6026);
    }

    #[test]
    fn test_p2() {
        let test_instructions = parse_input("../inputs/d09_test");
        assert_eq!(p2(&test_instructions), 1);

        let test_instructions2 = parse_input("../inputs/d09_test2");
        assert_eq!(p2(&test_instructions2), 36);

        let instructions = parse_input("../inputs/d09");
        assert_eq!(p2(&instructions), 2273);
    }
}
