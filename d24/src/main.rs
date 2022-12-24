use std::{
    arch::x86_64::_blcic_u32,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

type Blizzards = HashMap<Coords, Vec<Direction>>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Valley {
    blizzards: Blizzards,
    start: Coords,
    target: Coords,
}

use Direction::*;

fn parse_input(path: &str) -> Valley {
    let mut blizzards = HashMap::new();
    let mut max_row = 0;
    let mut max_col = 0;

    for (row, line) in fs::read_to_string(path).unwrap().trim_end().split("\n").enumerate() {
        max_row = row.max(max_row);

        let chars = line.trim_matches('#').chars();
        for (col, ch) in chars.enumerate() {
            max_col = col.max(max_col);

            if ch != '.' {
                let direction = match ch {
                    '>' => Right,
                    '<' => Left,
                    '^' => Up,
                    'v' => Down,
                    _ => panic!("unexpected char: {ch}"),
                };

                let coords = Coords { row, col };
                blizzards.insert(coords, vec![direction]);
            }
        }
    }

    Valley { blizzards, start: Coords { row: 0, col: 0 }, target: Coords { row: max_row, col: max_col } }
}

fn advance(blizzards: &Blizzards, max_row: usize, max_col: usize) -> Blizzards {
    let mut new_blizzards: Blizzards = HashMap::new();

    for (&Coords { row, col }, blizzards_at_coords) in blizzards {
        for &direction in blizzards_at_coords {
            let (new_row, new_col) = match direction {
                Right => {
                    let new_col = if col + 1 > max_col { 0 } else { col + 1 };
                    (row, new_col)
                }
                Left => {
                    let new_col = if col == 0 { max_col } else { col - 1 };
                    (row, new_col)
                }
                Up => {
                    let new_row = if row == 1 { max_row } else { row - 1 };
                    (new_row, col)
                }
                Down => {
                    let new_row = if row + 1 > max_row { 1 } else { row + 1 };
                    (new_row, col)
                }
            };

            new_blizzards.entry(Coords { row: new_row, col: new_col }).or_default().push(direction);
        }
    }

    new_blizzards
}

#[derive(Debug, Clone, Copy)]
struct ScoredCoords {
    coords: Coords,
    score: usize,
    minute: usize,
}

impl PartialEq for ScoredCoords {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for ScoredCoords {}

impl Ord for ScoredCoords {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for ScoredCoords {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const MOVES: [(i32, i32); 5] = [(0, 1), (0, -1), (-1, 0), (1, 0), (0, 0)];

// manhattan distance is always admissable here
fn heuristic(valley: &Valley, coords: Coords) -> usize {
    (valley.target.row.saturating_sub(coords.row)) + (valley.target.col.saturating_sub(coords.col))
}

fn find_path(valley: &Valley) -> usize {
    let mut blizzards_at_times = HashMap::new();
    blizzards_at_times.insert(0, valley.blizzards.clone());

    // openSet + current fScore used to order
    let mut discovered = BinaryHeap::new();
    discovered.push(Reverse(ScoredCoords { coords: valley.start, score: heuristic(valley, valley.start), minute: 0 }));

    // gScore
    let mut known_path_scores = HashMap::new();
    known_path_scores.insert(valley.start, 0);

    while !discovered.is_empty() {
        let Reverse(ScoredCoords { coords, minute, .. }) = discovered.pop().unwrap();
        dbg!((coords, minute));
        if coords == valley.target {
            return known_path_scores[&coords];
        }

        let current_known_path_score = known_path_scores[&coords];

        // lazily compute blizzards at a given time
        let blizzards = match blizzards_at_times.get(&(minute + 1)) {
            Some(blizzards) => blizzards,
            None => {
                let blizzards = advance(&blizzards_at_times[&(minute)], valley.target.row - 1, valley.target.col);
                blizzards_at_times.insert(minute, blizzards);
                &blizzards_at_times[&minute]
            }
        };

        for &(row_delta, col_delta) in &MOVES {
            dbg!((row_delta, col_delta));

            // discard coords outside the allowed path
            if ((coords.row == 1 || coords.row == 0) && row_delta == -1) || (coords.col == 0 && col_delta == -1) {
                continue;
            }

            let new_coords =
                Coords { row: (coords.row as i32 + row_delta) as usize, col: (coords.col as i32 + col_delta) as usize };

            dbg!(new_coords);

            // discard wrong coordinates + cannot run into a blizzard
            if new_coords.row > valley.target.row
                || new_coords.col > valley.target.col
                || (new_coords.row == 0 && new_coords.col != 0)
                || (new_coords.row == valley.target.row && new_coords.col != valley.target.col)
                || blizzards.contains_key(&new_coords) && !blizzards[&new_coords].is_empty()
            {
                continue;
            }

            dbg!(new_coords);

            // any move or waiting will cost 1 minute
            let new_path_score = current_known_path_score + 1;
            if new_path_score < *known_path_scores.get(&new_coords).unwrap_or(&usize::MAX) {
                known_path_scores.insert(new_coords, new_path_score);
                dbg!(("adding to known_path_scores", new_coords, new_path_score));

                discovered.push(Reverse(ScoredCoords {
                    coords: new_coords,
                    score: heuristic(valley, new_coords),
                    minute: new_path_score,
                }));
                dbg!(("adding to discovered", new_coords, heuristic(valley, new_coords), new_path_score));
            }
        }
    }

    panic!("didn't find anything!")
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_test() {
        let test_input = parse_input("../inputs/d24_test");

        assert_eq!(test_input.blizzards.keys().len(), 19);

        assert_eq!(test_input.blizzards[&Coords { row: 1, col: 0 }], vec![Right]);
        assert_eq!(test_input.blizzards[&Coords { row: 1, col: 5 }], vec![Left]);
        assert_eq!(test_input.blizzards[&Coords { row: 4, col: 1 }], vec![Up]);
        assert_eq!(test_input.blizzards[&Coords { row: 4, col: 2 }], vec![Down]);

        assert_eq!(test_input.start, Coords { row: 0, col: 0 });
        assert_eq!(test_input.target, Coords { row: 5, col: 5 });

        let input = parse_input("../inputs/d24");
        assert_eq!(input.blizzards.keys().len(), 2710);
        assert_eq!(input.start, Coords { row: 0, col: 0 });
        assert_eq!(input.target, Coords { row: 26, col: 119 });
    }

    #[test]
    fn advance_test() {
        let test_input = parse_input("../inputs/d24_test");

        let min1_blizzards = advance(&test_input.blizzards, test_input.target.row - 1, test_input.target.col);
        assert_eq!(min1_blizzards.keys().len(), 14);
        assert_eq!(min1_blizzards[&Coords { row: 1, col: 1 }], vec![Right]);
        assert_eq!(min1_blizzards[&Coords { row: 1, col: 2 }].len(), 3);

        let min2_blizzards = advance(&min1_blizzards, test_input.target.row - 1, test_input.target.col);
        assert_eq!(min2_blizzards.keys().len(), 14);

        let min3_blizzards = advance(&min2_blizzards, test_input.target.row - 1, test_input.target.col);
        assert_eq!(min3_blizzards.keys().len(), 14);
        assert_eq!(min3_blizzards[&Coords { row: 1, col: 1 }], vec![Up]);
        assert_eq!(min3_blizzards[&Coords { row: 3, col: 2 }].len(), 2);
        assert_eq!(min3_blizzards[&Coords { row: 4, col: 2 }], vec![Right]);
        assert_eq!(min3_blizzards[&Coords { row: 4, col: 3 }], vec![Left]);
    }

    #[test]
    fn find_path_test() {
        let test_input = parse_input("../inputs/d24_test");
        assert_eq!(find_path(&test_input), 0);
    }
}
