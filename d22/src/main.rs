use std::{collections::HashSet, fs};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Row {
    start_col: usize,
    end_col: usize,
    // column numbers only
    walls: HashSet<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Move { tiles: i64 },
    TurnL,
    TurnR,
}

#[derive(Debug, Clone, Default)]
struct Board {
    rows: Vec<Row>,
    path: Vec<Step>,
}

impl Board {
    fn walk(&self) -> (usize, usize, Facing) {
        let mut curr_row = 0;
        let mut curr_col = self.rows[0].start_col;
        let mut curr_facing = Facing::Right;

        for &step in &self.path {
            match step {
                Step::Move { tiles } => match curr_facing {
                    Facing::Right => {
                        for _ in 0..tiles {
                            let mut new_col = curr_col + 1;
                            if new_col >= self.rows[curr_row].end_col {
                                new_col = self.rows[curr_row].start_col;
                            }

                            if !self.rows[curr_row].walls.contains(&new_col) {
                                curr_col = new_col
                            }
                        }
                    }
                    Facing::Left => {
                        for _ in 0..tiles {
                            let mut new_col = curr_col.saturating_sub(1);
                            if curr_col == 0 || new_col < self.rows[curr_row].start_col {
                                new_col = self.rows[curr_row].end_col - 1;
                            }

                            if !self.rows[curr_row].walls.contains(&new_col) {
                                curr_col = new_col
                            }
                        }
                    }
                    Facing::Down => {
                        for _ in 0..tiles {
                            let mut new_row = curr_row + 1;
                            if new_row >= self.rows.len()
                                || curr_col < self.rows[new_row].start_col
                                || curr_col >= self.rows[new_row].end_col
                            {
                                new_row = self
                                    .rows
                                    .iter()
                                    .position(|row| curr_col >= row.start_col && curr_col < row.end_col)
                                    .unwrap();
                            }

                            if !self.rows[new_row].walls.contains(&curr_col) {
                                curr_row = new_row
                            }
                        }
                    }
                    Facing::Up => {
                        for _ in 0..tiles {
                            let mut new_row = curr_row.saturating_sub(1);
                            if curr_row == 0
                                || curr_col < self.rows[new_row].start_col
                                || curr_col >= self.rows[new_row].end_col
                            {
                                new_row = self
                                    .rows
                                    .iter()
                                    .rev()
                                    .position(|row| curr_col >= row.start_col && curr_col < row.end_col)
                                    .unwrap();
                            }

                            if !self.rows[new_row].walls.contains(&curr_col) {
                                curr_row = new_row
                            }
                        }
                    }
                },
                Step::TurnL => curr_facing = curr_facing.turn_l(),
                Step::TurnR => curr_facing = curr_facing.turn_r(),
            }
        }

        (curr_row, curr_col, curr_facing)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

use Facing::*;

impl Facing {
    fn turn_r(self) -> Facing {
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }

    fn turn_l(self) -> Facing {
        match self {
            Right => Up,
            Down => Right,
            Left => Down,
            Up => Left,
        }
    }
}

impl Into<usize> for Facing {
    fn into(self) -> usize {
        match self {
            Right => 0,
            Down => 1,
            Left => 2,
            Up => 3,
        }
    }
}

fn add_path_digit(board: &mut Board, digits: &mut Vec<char>) {
    if !digits.is_empty() {
        let tiles = String::from_iter(digits.iter()).parse().unwrap();
        digits.clear();
        board.path.push(Step::Move { tiles });
    }
}

fn parse_input(path: &str) -> Board {
    let mut board = Board::default();
    let mut is_path_line = false;
    for line in fs::read_to_string(path.to_string()).unwrap().trim_end().split("\n") {
        let line = line.trim_end();
        if line == "" {
            is_path_line = true;
        }

        if is_path_line {
            let mut digits = vec![];
            for ch in line.chars() {
                match ch {
                    'L' => {
                        add_path_digit(&mut board, &mut digits);
                        board.path.push(Step::TurnL);
                    }
                    'R' => {
                        add_path_digit(&mut board, &mut digits);
                        board.path.push(Step::TurnR);
                    }
                    ch if ch.is_digit(10) => digits.push(ch),
                    _ => panic!("unexpected ch {ch}."),
                }
            }
            add_path_digit(&mut board, &mut digits);
        } else {
            let mut start_col = 0;
            let mut end_col = 0;
            let mut walls = HashSet::new();

            for ch in line.chars() {
                match ch {
                    ' ' => start_col += 1,
                    '.' => end_col += 1,
                    '#' => {
                        walls.insert(end_col + start_col);
                        end_col += 1;
                    }
                    _ => panic!("unknown ch: {ch}."),
                }
            }

            end_col += start_col;
            board.rows.push(Row { start_col, end_col, walls })
        }
    }
    board
}

fn p1(board: &Board) -> usize {
    let (row, col, facing) = board.walk();
    (row + 1) * 1000 + (col + 1) * 4 + (facing as usize)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_test() {
        let test_input = parse_input("../inputs/d22_test");
        assert_eq!(p1(&test_input), 6032);

        let input = parse_input("../inputs/d22");
        // TODO: too high!
        assert_eq!(p1(&input), 151016);
        // let input = parse_input("../inputs/d22");
    }
}
