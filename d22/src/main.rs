use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::Range,
    time::Instant,
};

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

/// left, right, up, and down ids are assigned to the adjacent face ids at the corresponding directions
/// if the current face is "front" of the cube
#[derive(Debug, Clone)]
struct Face {
    id: usize,
    rows: Range<usize>,
    cols: Range<usize>,
    left: Transition,
    right: Transition,
    up: Transition,
    down: Transition,
}

/// `swap` is `true` when we need to swap cols & rows
#[derive(Debug, Clone, Copy)]
struct Transition {
    face_id: usize,
    facing: Facing,
    swap: bool,
    col_delta: i64,
    row_delta: i64,
}

lazy_static! {
    static ref TEST_FACES: [Face; 1] = [
        Face {
            id: 0,
            rows: 0..4,
            cols: 8..12,
            left: Transition { face_id: 2, facing: Down, swap: true, col_delta: 0, row_delta: 0 },
            right: Transition { face_id: 5, facing: Left, swap: false, col_delta: 4, row_delta: 4 },
            up: Transition { face_id: 1, facing: Down, swap: false, col_delta: -4, row_delta: 4 },
            down: Transition { face_id: 3, facing: Down, swap: false, col_delta: 0, row_delta: 0 },
        },
//         // Face { id: 1, rows: 4..8, cols: 0..4, left_id: 2, right_id: 5, up_id: 4, down_id: 0 },
//         // Face { id: 2, rows: 4..8, cols: 4..8, left_id: 3, right_id: 1, up_id: 4, down_id: 0 },
//         // Face { id: 3, rows: 4..8, cols: 8..12, left_id: 5, right_id: 2, up_id: 4, down_id: 0 },
//         // Face { id: 4, rows: 8..12, cols: 8..12, left_id: 2, right_id: 5, up_id: 3, down_id: 1 },
//         // Face { id: 5, rows: 8..12, cols: 12..16, left_id: 4, right_id: 0, up_id: 3, down_id: 2 }
    ];
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

        let mut trace = HashMap::new();

        for &step in &self.path {
            match step {
                Step::Move { tiles } => {
                    // TODO: dbg!((curr_row, curr_col, curr_facing, &step));
                    if curr_col < self.rows[curr_row].start_col || curr_col >= self.rows[curr_row].end_col {
                        panic!("outside the field at #{curr_row}, #{curr_col}, #{curr_facing:#?}.")
                    }

                    match curr_facing {
                        Facing::Right => {
                            for _ in 0..tiles {
                                let mut new_col = curr_col + 1;
                                if new_col >= self.rows[curr_row].end_col {
                                    new_col = self.rows[curr_row].start_col;
                                }

                                if !self.rows[curr_row].walls.contains(&new_col) {
                                    curr_col = new_col;

                                    trace.insert((curr_row, curr_col), curr_facing);
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
                                    curr_col = new_col;

                                    trace.insert((curr_row, curr_col), curr_facing);
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
                                    curr_row = new_row;

                                    trace.insert((curr_row, curr_col), curr_facing);
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
                                        .enumerate()
                                        .rev()
                                        .find(|(_row_idx, row)| curr_col >= row.start_col && curr_col < row.end_col)
                                        .unwrap()
                                        .0;
                                }

                                if !self.rows[new_row].walls.contains(&curr_col) {
                                    curr_row = new_row;

                                    trace.insert((curr_row, curr_col), curr_facing);
                                }
                            }
                        }
                    }
                }
                Step::TurnL => {
                    curr_facing = curr_facing.turn_l();

                    trace.insert((curr_row, curr_col), curr_facing);
                }
                Step::TurnR => {
                    curr_facing = curr_facing.turn_r();

                    trace.insert((curr_row, curr_col), curr_facing);
                }
            }
        }

        print_trace(self, trace);

        (curr_row, curr_col, curr_facing)
    }
}

fn print_trace(board: &Board, trace: HashMap<(usize, usize), Facing>) {
    for (row_idx, row) in board.rows.iter().enumerate() {
        for col_idx in 0..row.end_col {
            match trace.get(&(row_idx, col_idx)) {
                None => {
                    if row.walls.contains(&col_idx) {
                        print!("#");
                    } else {
                        if col_idx >= row.start_col {
                            print!(".");
                        } else {
                            print!(" ");
                        }
                    }
                }
                Some(facing) => {
                    // outside the field at #0, #30, #Right.
                    if row.walls.contains(&col_idx) {
                        panic!("standing in the wall at #{row_idx}, #{col_idx}, #{facing:#?}.")
                    }

                    if col_idx < row.start_col {
                        panic!("outside the field at #{row_idx}, #{col_idx}, #{facing:#?}.")
                    }

                    print!("P");
                }
            }
        }

        println!("");
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
    let input = parse_input("../inputs/d22");

    let timer = Instant::now();
    let p1_ans = p1(&input);
    let elapsed = timer.elapsed();
    println!("p1 ans = {p1_ans} [{elapsed:?}]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_test() {
        let test_input = parse_input("../inputs/d22_test");
        assert_eq!(p1(&test_input), 6032);

        let input = parse_input("../inputs/d22");
        assert_eq!(p1(&input), 89224);
    }
}
