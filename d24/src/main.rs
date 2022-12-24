use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    fs,
    time::Instant,
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

fn get_neighbours(valley: &Valley, coords: Coords) -> Vec<Coords> {
    // coords is the "neighbour" which corresponds to waiting in a location
    let mut neighbours = vec![coords];

    // up
    if coords.row > 1 {
        neighbours.push(Coords { row: coords.row - 1, col: coords.col });
    }
    // up to the start location
    if coords.row == 1 && coords.col == 0 {
        neighbours.push(Coords { row: 0, col: 0 });
    }

    // down
    if coords.row < valley.target.row - 1 {
        neighbours.push(Coords { row: coords.row + 1, col: coords.col });
    }
    // down to target
    if coords.row < valley.target.row && coords.col == valley.target.col {
        neighbours.push(Coords { row: coords.row + 1, col: coords.col });
    }

    // left
    if coords.col >= 1 {
        neighbours.push(Coords { row: coords.row, col: coords.col - 1 });
    }

    // right
    if coords.col < valley.target.col && coords.row != 0 {
        neighbours.push(Coords { row: coords.row, col: coords.col + 1 });
    }

    neighbours
}

// manhattan distance is always admissable here
fn heuristic(start: Coords, target: Coords) -> usize {
    ((target.row as i64 - start.row as i64).abs() + (target.col as i64 - start.col as i64).abs()) as usize
}

/// A* implementation with the following tweaks compared to the Wikipedia pseudocode:
/// - `fScore` and `openSet` tracking are combined into `discovered` BinaryHeap
/// - `gScore` is called `known_path_scores`
/// - we track the current minute together with the location + we use both as a key in `known_path_scores`
/// - we always allow waiting in the current location if blizzard doesn't move there
/// - blizzard positions are lazily computed when needed and cached; due to the continuity of the paths,
/// we can always rely on the blizzard locations for the previous day to be cached
fn find_path(
    valley: &Valley,
    start: Coords,
    target: Coords,
    start_time: usize,
    precomputed_blizzards_at_times: Option<HashMap<usize, Blizzards>>,
) -> (usize, HashMap<usize, Blizzards>) {
    let mut blizzards_at_times = HashMap::new();
    match precomputed_blizzards_at_times {
        None => {
            blizzards_at_times.insert(0, valley.blizzards.clone());
        }
        Some(precomputed_blizzards_at_times) => {
            blizzards_at_times.insert(start_time, precomputed_blizzards_at_times[&start_time].clone());
        }
    }

    let mut discovered = BinaryHeap::new();
    discovered.push(Reverse(ScoredCoords { coords: start, score: heuristic(start, target), minute: start_time }));

    let mut known_path_scores = HashMap::new();
    known_path_scores.insert((start, start_time), start_time);

    while !discovered.is_empty() {
        let Reverse(ScoredCoords { coords, minute, .. }) = discovered.pop().unwrap();
        if coords == target {
            return (known_path_scores[&(coords, minute)], blizzards_at_times);
        }

        let current_known_path_score = known_path_scores[&(coords, minute)];

        // lazily compute blizzards at a given time
        let blizzards = match blizzards_at_times.get(&(minute + 1)) {
            Some(blizzards) => blizzards,
            None => {
                let blizzards = advance(&blizzards_at_times[&(minute)], valley.target.row - 1, valley.target.col);
                blizzards_at_times.insert(minute + 1, blizzards);
                &blizzards_at_times[&(minute + 1)]
            }
        };

        for &new_coords in get_neighbours(valley, coords)
            .iter()
            .filter(|new_coords| !blizzards.contains_key(&new_coords) || blizzards[&new_coords].is_empty())
        {
            // any move or waiting will cost 1 minute
            let new_path_score = current_known_path_score + 1;
            // normal A* condition + waiting is always allowed as long as the location is not affected by the blizzard
            if new_coords == coords
                || new_path_score < *known_path_scores.get(&(new_coords, minute + 1)).unwrap_or(&usize::MAX)
            {
                known_path_scores.insert((new_coords, minute + 1), new_path_score);

                discovered.push(Reverse(ScoredCoords {
                    coords: new_coords,
                    score: new_path_score + heuristic(new_coords, target),
                    minute: new_path_score,
                }));
            }
        }
    }

    panic!("didn't find anything!")
}

fn p2(valley: &Valley, p1_answer: usize, p1_blizzards_at_times: HashMap<usize, Blizzards>) -> usize {
    let (back_path_minutes, blizzards_at_back_path) =
        find_path(&valley, valley.target, valley.start, p1_answer, Some(p1_blizzards_at_times));
    let (p2_ans, _) = find_path(&valley, valley.start, valley.target, back_path_minutes, Some(blizzards_at_back_path));
    p2_ans
}

fn main() {
    let valley = parse_input("../inputs/d24");

    let timer = Instant::now();
    let (p1_ans, p1_blizzards_at_times) = find_path(&valley, valley.start, valley.target, 0, None);
    let elapsed = timer.elapsed();
    println!("\np1 ans = {p1_ans} [{elapsed:?}]");

    let timer = Instant::now();
    let p2_ans = p2(&valley, p1_ans, p1_blizzards_at_times);
    let elapsed = timer.elapsed();
    println!("p2 ans = {p2_ans} [{elapsed:?}]");
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

        let (p1_ans_test, blizzards_at_p1_test) = find_path(&test_input, test_input.start, test_input.target, 0, None);
        assert_eq!(p1_ans_test, 18);

        assert_eq!(p2(&test_input, p1_ans_test, blizzards_at_p1_test), 54);

        let input = parse_input("../inputs/d24");
        let (p1_ans, blizzards_at_p1) = find_path(&input, input.start, input.target, 0, None);
        assert_eq!(p1_ans, 281);

        // 743 is too low
        assert_ne!(p2(&input, p1_ans, blizzards_at_p1), 743);
    }
}
