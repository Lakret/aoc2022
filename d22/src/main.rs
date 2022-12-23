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

// --------------------------
// ********* Part 2 *********
// --------------------------

/// left, right, up, and down show transiations from the current face borders.
/// `rows` and `cols` map relative face coordinates to the oritinal net coordinates
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

/// Transiations are expressed in relative coordinates of the source & destination cube faces.
/// `swap` is `true` when we need to swap cols & rows coordinates, i.e. (2, 3) becomes (3, 2)
/// `inv_row` and `inv_col` are applied after `swap`.
/// if both `inv_row` and `inv_col` are `false`, coordinates are left intact.
/// if `inv_row` is `true` the row will be inverted, i.e. it will be set to (MAX_ROW - row_id - 1).
/// `inv_col` does the same for the column.
#[derive(Debug, Clone, Copy)]
struct Transition {
    face_id: usize,
    facing: Facing,
    swap: bool,
    inv_row: bool,
    inv_col: bool,
}

/// returns (new_face_id, new_facing, new_face_row_id, new_face_col_id)
fn move_to_face(
    faces: &[Face; 6],
    size: usize,
    from_face_id: usize,
    facing: Facing,
    row_id: usize,
    col_id: usize,
) -> (usize, Facing, usize, usize) {
    let transition = match facing {
        Left => faces[from_face_id].left,
        Right => faces[from_face_id].right,
        Down => faces[from_face_id].down,
        Up => faces[from_face_id].up,
    };

    let mut row_id = row_id;
    let mut col_id = col_id;

    if transition.swap {
        std::mem::swap(&mut row_id, &mut col_id);
    }

    if transition.inv_row {
        row_id = size - row_id - 1;
    }

    if transition.inv_col {
        col_id = size - col_id - 1;
    }

    (transition.face_id, transition.facing, row_id, col_id)
}

/// Returns (face_id, face_row_id, face_col_id).
fn board_coords_to_face_coords(faces: &[Face; 6], row_id: usize, col_id: usize) -> (usize, usize, usize) {
    let face_id = faces.iter().position(|face| face.rows.contains(&row_id) && face.cols.contains(&col_id)).unwrap();
    let face_row_id = row_id - faces[face_id].rows.start;
    let face_col_id = col_id - faces[face_id].cols.start;
    (face_id, face_row_id, face_col_id)
}

/// Returns a vector, where index is the face_id, and values are (face_row_id, face_col_id) tuples.
fn get_walls_on_cube(board: &Board, faces: &[Face; 6]) -> Vec<HashSet<(usize, usize)>> {
    let mut walls = vec![HashSet::new(); 6];

    for (row_id, row) in board.rows.iter().enumerate() {
        for &col_id in row.walls.iter() {
            let (face_id, face_row_id, face_col_id) = board_coords_to_face_coords(faces, row_id, col_id);
            walls[face_id].insert((face_row_id, face_col_id));
        }
    }

    walls
}

fn walk_on_cube(board: &Board, faces: &[Face; 6], size: usize) -> (usize, usize, Facing) {
    let cube_walls = get_walls_on_cube(board, faces);

    let mut curr_face = 0;
    let mut curr_row = 0;
    let mut curr_col = 0;
    let mut curr_facing = Right;

    // let mut trace = HashMap::new();

    for &step in &board.path {
        match step {
            Step::Move { tiles } => {
                if curr_row >= size || curr_col >= size {
                    panic!("outside the field at #{curr_face}, #{curr_row}, #{curr_col}, #{curr_facing:#?}.")
                }

                if cube_walls[curr_face].contains(&(curr_row, curr_col)) {
                    panic!("ran into a wall at #{curr_face}, #{curr_row}, #{curr_col}, #{curr_facing:#?}.")
                }

                for _ in 0..tiles {
                    let mut new_face = curr_face;
                    let mut new_row = curr_row;
                    let mut new_col = curr_col;
                    let mut new_facing = curr_facing;

                    match curr_facing {
                        Right => {
                            if curr_col + 1 >= size {
                                (new_face, new_facing, new_row, new_col) =
                                    move_to_face(faces, size, curr_face, curr_facing, curr_row, curr_col);
                            } else {
                                new_col += 1;
                            }
                        }
                        Left => {
                            if curr_col == 0 {
                                (new_face, new_facing, new_row, new_col) =
                                    move_to_face(faces, size, curr_face, curr_facing, curr_row, curr_col);
                            } else {
                                new_col -= 1;
                            }
                        }
                        Down => {
                            if curr_row + 1 >= size {
                                (new_face, new_facing, new_row, new_col) =
                                    move_to_face(faces, size, curr_face, curr_facing, curr_row, curr_col);
                            } else {
                                new_row += 1;
                            }
                        }
                        Up => {
                            if curr_row == 0 {
                                (new_face, new_facing, new_row, new_col) =
                                    move_to_face(faces, size, curr_face, curr_facing, curr_row, curr_col);
                            } else {
                                new_row -= 1;
                            }
                        }
                    }

                    if !cube_walls[new_face].contains(&(new_row, new_col)) {
                        curr_face = new_face;
                        curr_row = new_row;
                        curr_col = new_col;
                        curr_facing = new_facing;
                        // trace.insert((curr_row, curr_col), curr_facing);
                    }
                }
            }
            Step::TurnL => {
                curr_facing = curr_facing.turn_l();
                // trace.insert((curr_row, curr_col), curr_facing);
            }
            Step::TurnR => {
                curr_facing = curr_facing.turn_r();
                // trace.insert((curr_row, curr_col), curr_facing);
            }
        }
    }

    // print_trace(board, trace);

    (faces[curr_face].rows.start + curr_row, faces[curr_face].cols.start + curr_col, curr_facing)
}

lazy_static! {
    static ref TEST_FACES: [Face; 6] = [
        Face {
            id: 0,
            rows: 0..4,
            cols: 8..12,
            left: Transition { face_id: 2, facing: Down, swap: true, inv_row: false, inv_col: false },
            right: Transition { face_id: 5, facing: Left, swap: false, inv_row: true, inv_col: false },
            up: Transition { face_id: 1, facing: Down, swap: false, inv_row: false, inv_col: true },
            down: Transition { face_id: 3, facing: Down, swap: false, inv_row: true, inv_col: false },
        },
        Face {
            id: 1,
            rows: 4..8,
            cols: 0..4,
            left: Transition { face_id: 5, facing: Up, swap: true, inv_row: true, inv_col: true },
            right: Transition { face_id: 2, facing: Right, swap: false, inv_row: false, inv_col: true },
            up: Transition { face_id: 0, facing: Down, swap: false, inv_row: false, inv_col: true },
            down: Transition { face_id: 4, facing: Up, swap: false, inv_row: false, inv_col: true }
        },
        Face {
            id: 2,
            rows: 4..8,
            cols: 4..8,
            left: Transition { face_id: 1, facing: Left, swap: false, inv_row: false, inv_col: true },
            right: Transition { face_id: 3, facing: Right, swap: false, inv_row: false, inv_col: true },
            up: Transition { face_id: 0, facing: Right, swap: true, inv_row: false, inv_col: false },
            down: Transition { face_id: 4, facing: Right, swap: true, inv_row: true, inv_col: true }
        },
        Face {
            id: 3,
            rows: 4..8,
            cols: 8..12,
            left: Transition { face_id: 2, facing: Left, swap: false, inv_row: false, inv_col: true },
            right: Transition { face_id: 5, facing: Down, swap: true, inv_row: true, inv_col: true },
            up: Transition { face_id: 0, facing: Up, swap: false, inv_row: true, inv_col: false },
            down: Transition { face_id: 4, facing: Down, swap: false, inv_row: true, inv_col: false }
        },
        Face {
            id: 4,
            rows: 8..12,
            cols: 8..12,
            left: Transition { face_id: 2, facing: Up, swap: true, inv_row: true, inv_col: true },
            right: Transition { face_id: 5, facing: Right, swap: false, inv_row: false, inv_col: true },
            up: Transition { face_id: 3, facing: Up, swap: false, inv_row: true, inv_col: false },
            down: Transition { face_id: 1, facing: Up, swap: false, inv_row: false, inv_col: true }
        },
        Face {
            id: 5,
            rows: 8..12,
            cols: 12..16,
            left: Transition { face_id: 4, facing: Left, swap: false, inv_row: false, inv_col: true },
            right: Transition { face_id: 0, facing: Left, swap: false, inv_row: true, inv_col: false },
            up: Transition { face_id: 3, facing: Right, swap: true, inv_row: true, inv_col: true },
            down: Transition { face_id: 1, facing: Right, swap: true, inv_row: true, inv_col: true }
        }
    ];
}

fn p2(board: &Board, faces: &[Face; 6], size: usize) -> usize {
    let (row, col, facing) = walk_on_cube(board, faces, size);
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

    #[test]
    fn move_to_face_test() {
        let size = 4;

        assert_eq!(move_to_face(&TEST_FACES, size, 0, Left, 1, 0), (2, Down, 0, 1));
        assert_eq!(move_to_face(&TEST_FACES, size, 0, Right, 1, 3), (5, Left, 2, 3));
        assert_eq!(move_to_face(&TEST_FACES, size, 0, Up, 0, 1), (1, Down, 0, 2));
        assert_eq!(move_to_face(&TEST_FACES, size, 0, Down, 3, 1), (3, Down, 0, 1));

        assert_eq!(move_to_face(&TEST_FACES, size, 1, Left, 1, 0), (5, Up, 3, 2));
        assert_eq!(move_to_face(&TEST_FACES, size, 1, Right, 1, 3), (2, Right, 1, 0));
        assert_eq!(move_to_face(&TEST_FACES, size, 1, Up, 0, 1), (0, Down, 0, 2));
        assert_eq!(move_to_face(&TEST_FACES, size, 1, Down, 3, 1), (4, Up, 3, 2));

        assert_eq!(move_to_face(&TEST_FACES, size, 2, Left, 1, 0), (1, Left, 1, 3));
        assert_eq!(move_to_face(&TEST_FACES, size, 2, Right, 1, 3), (3, Right, 1, 0));
        assert_eq!(move_to_face(&TEST_FACES, size, 2, Up, 0, 1), (0, Right, 1, 0));
        assert_eq!(move_to_face(&TEST_FACES, size, 2, Down, 3, 1), (4, Right, 2, 0));

        assert_eq!(move_to_face(&TEST_FACES, size, 3, Left, 1, 0), (2, Left, 1, 3));
        assert_eq!(move_to_face(&TEST_FACES, size, 3, Right, 1, 3), (5, Down, 0, 2));
        assert_eq!(move_to_face(&TEST_FACES, size, 3, Up, 0, 1), (0, Up, 3, 1));
        assert_eq!(move_to_face(&TEST_FACES, size, 3, Down, 3, 1), (4, Down, 0, 1));

        assert_eq!(move_to_face(&TEST_FACES, size, 4, Left, 1, 0), (2, Up, 3, 2));
        assert_eq!(move_to_face(&TEST_FACES, size, 4, Right, 1, 3), (5, Right, 1, 0));
        assert_eq!(move_to_face(&TEST_FACES, size, 4, Up, 0, 1), (3, Up, 3, 1));
        assert_eq!(move_to_face(&TEST_FACES, size, 4, Down, 3, 1), (1, Up, 3, 2));

        assert_eq!(move_to_face(&TEST_FACES, size, 5, Left, 1, 0), (4, Left, 1, 3));
        assert_eq!(move_to_face(&TEST_FACES, size, 5, Right, 1, 3), (0, Left, 2, 3));
        assert_eq!(move_to_face(&TEST_FACES, size, 5, Up, 0, 1), (3, Right, 2, 3));
        assert_eq!(move_to_face(&TEST_FACES, size, 5, Down, 3, 1), (1, Right, 2, 0));
    }

    #[test]
    fn board_coords_to_face_coords_test() {
        let test_input = parse_input("../inputs/d22_test");

        assert_eq!(board_coords_to_face_coords(&TEST_FACES, 0, test_input.rows[0].start_col), (0, 0, 0));
        assert_eq!(board_coords_to_face_coords(&TEST_FACES, 0, test_input.rows[0].start_col + 1), (0, 0, 1));
        assert_eq!(board_coords_to_face_coords(&TEST_FACES, 1, test_input.rows[0].start_col), (0, 1, 0));
        assert_eq!(board_coords_to_face_coords(&TEST_FACES, 5, 7), (2, 1, 3));
    }

    #[test]
    fn get_walls_on_cube_test() {
        let test_input = parse_input("../inputs/d22_test");

        let cube_walls = get_walls_on_cube(&test_input, &TEST_FACES);
        assert_eq!(cube_walls.len(), 6);

        let second_face_walls = &cube_walls[2];
        assert_eq!(second_face_walls.len(), 1);
        assert!(second_face_walls.contains(&(2, 3)));
    }

    #[test]
    fn p2_test() {
        let test_input = parse_input("../inputs/d22_test");
        assert_eq!(p2(&test_input, &TEST_FACES, 4), 5031);

        // let input = parse_input("../inputs/d22");
        // assert_eq!(p2(&input), 89224);
    }
}
