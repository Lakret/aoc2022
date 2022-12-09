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

fn is_adjacent(head_pos: (i32, i32), tail_pos: (i32, i32)) -> bool {
    (head_pos == tail_pos)
        || (((head_pos.0 - tail_pos.0).abs() <= 1) && ((head_pos.1 - tail_pos.1).abs() <= 1))
}

fn move_in_direction(pos: (i32, i32), direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Right => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Up => (pos.0, pos.1 + 1),
        Direction::Down => (pos.0, pos.1 - 1),
    }
}

fn move_tail(tail_pos: (i32, i32), head_pos: (i32, i32)) -> (i32, i32) {
    let possible_new_x = if (tail_pos.0 - head_pos.0).abs() >= 1 {
        if head_pos.0 > tail_pos.0 {
            tail_pos.0 + 1
        } else {
            tail_pos.0 - 1
        }
    } else {
        tail_pos.0
    };

    let possible_new_y = if (tail_pos.1 - head_pos.1).abs() >= 1 {
        if head_pos.1 > tail_pos.1 {
            tail_pos.1 + 1
        } else {
            tail_pos.1 - 1
        }
    } else {
        tail_pos.1
    };

    if possible_new_x != tail_pos.0 && possible_new_y != tail_pos.1 {
        (possible_new_x, possible_new_y)
    } else if is_adjacent(head_pos, (possible_new_x, tail_pos.1)) {
        (possible_new_x, tail_pos.1)
    } else if is_adjacent(head_pos, (tail_pos.0, possible_new_y)) {
        (tail_pos.0, possible_new_y)
    } else {
        (possible_new_x, possible_new_y)
    }
}

fn p1(instructions: &Vec<Instruction>) -> usize {
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    tail_visited.insert(tail_pos);

    for &Instruction { direction, steps } in instructions {
        for _ in 0..steps {
            head_pos = move_in_direction(head_pos, direction);
            if !is_adjacent(head_pos, tail_pos) {
                tail_pos = move_tail(tail_pos, head_pos);
                tail_visited.insert(tail_pos);
            }
        }
    }

    tail_visited.len()
}

fn p2(instructions: &Vec<Instruction>) -> usize {
    let mut positions = vec![(0, 0); 10];
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    tail_visited.insert(positions[9]);

    for &Instruction { direction, steps } in instructions {
        for _ in 0..steps {
            positions[0] = move_in_direction(positions[0], direction);

            for knot_idx in 1..10 {
                if !is_adjacent(positions[knot_idx - 1], positions[knot_idx]) {
                    positions[knot_idx] = move_tail(positions[knot_idx], positions[knot_idx - 1]);
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
        assert_eq!(
            test_instructions[0],
            Instruction {
                steps: 4,
                direction: Direction::Right
            }
        );

        let instructions = parse_input("../inputs/d09");
        assert_eq!(instructions.len(), 2000);
    }

    #[test]
    fn test_is_adjacent() {
        assert_eq!(is_adjacent((2, 1), (1, 1)), true);
        assert_eq!(is_adjacent((2, 1), (2, 1)), true);
        assert_eq!(is_adjacent((1, 2), (2, 1)), true);
        assert_eq!(is_adjacent((2, 1), (1, 1)), true);

        assert_eq!(is_adjacent((2, 1), (1, 3)), false);
    }

    #[test]
    fn test_move_in_direction() {
        assert_eq!(move_in_direction((0, 0), Direction::Up), (0, 1));
        assert_eq!(move_in_direction((0, 0), Direction::Down), (0, -1));
        assert_eq!(move_in_direction((0, 0), Direction::Left), (-1, 0));
        assert_eq!(move_in_direction((0, 0), Direction::Right), (1, 0));
    }

    #[test]
    fn test_move_tail() {
        assert_eq!(move_tail((1, 1), (2, 3)), (2, 2));
        assert_eq!(move_tail((1, 1), (3, 2)), (2, 2));

        assert_eq!(move_tail((1, 1), (3, 1)), (2, 1));
        assert_eq!(move_tail((1, 3), (1, 1)), (1, 2));

        assert_eq!(move_tail((1, 0), (3, 0)), (2, 0));
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
