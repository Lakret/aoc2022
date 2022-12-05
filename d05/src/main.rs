use std::fs;

#[derive(Debug, Clone)]
struct Crates<const N: usize> {
    pub state: [Vec<char>; N],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    from: usize,
    to: usize,
    num: usize,
}

fn parse_moves(raw_moves: String) -> Vec<Move> {
    let mut moves = vec![];

    for line in raw_moves.trim_end().split("\n") {
        let mut it = line.split_ascii_whitespace();
        it.next();
        let num: usize = it.next().unwrap().parse().unwrap();
        it.next();
        let from: usize = it.next().unwrap().parse().unwrap();
        it.next();
        let to: usize = it.next().unwrap().parse().unwrap();

        moves.push(Move { from, to, num });
    }

    moves
}

fn p1<const N: usize>(crates: &Crates<N>, moves: &Vec<Move>) -> String {
    let mut crates = crates.clone();
    for mv in moves {
        for _ in 0..mv.num {
            let x = crates.state[mv.from - 1].pop().unwrap();
            crates.state[mv.to - 1].push(x);
        }
    }

    let mut ans = String::new();
    for cr in &crates.state {
        ans.push(*cr.last().unwrap());
    }
    ans
}

fn p2<const N: usize>(crates: &Crates<N>, moves: &Vec<Move>) -> String {
    let mut crates = crates.clone();
    for mv in moves {
        let source_crate = &mut crates.state[mv.from - 1];
        let start_idx = source_crate.len() - mv.num;
        let mut elems = source_crate.drain(start_idx..).collect::<Vec<_>>();

        crates.state[mv.to - 1].append(&mut elems);
    }

    let mut ans = String::new();
    for cr in &crates.state {
        ans.push(*cr.last().unwrap());
    }
    ans
}

fn get_task_input() -> (Crates<9>, Vec<Move>) {
    let crates = Crates {
        state: [
            vec!['B', 'Q', 'C'],
            vec!['R', 'Q', 'W', 'Z'],
            vec!['B', 'M', 'R', 'L', 'V'],
            vec!['C', 'Z', 'H', 'V', 'T', 'W'],
            vec!['D', 'Z', 'H', 'B', 'N', 'V', 'G'],
            vec!['H', 'N', 'P', 'C', 'J', 'F', 'V', 'Q'],
            vec!['D', 'G', 'T', 'R', 'W', 'Z', 'S'],
            vec!['C', 'G', 'M', 'N', 'B', 'W', 'Z', 'P'],
            vec!['N', 'J', 'B', 'M', 'W', 'Q', 'F', 'P'],
        ],
    };

    let moves = fs::read_to_string("../inputs/d05_edited").unwrap();

    let moves = parse_moves(moves);
    (crates, moves)
}

fn main() {
    let (crates, moves) = get_task_input();
    let p1_ans = p1(&crates, &moves);
    println!("P1: {p1_ans}.");

    let p2_ans = p2(&crates, &moves);
    println!("P2: {p2_ans}.");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> (Crates<3>, Vec<Move>) {
        let crates = Crates {
            state: [vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
        };

        let moves = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            .to_string();

        let moves = parse_moves(moves);
        (crates, moves)
    }

    #[test]
    fn test_input_parsing_test() {
        let (_crates, moves) = get_test_input();

        assert_eq!(moves.len(), 4);
        assert_eq!(
            &moves[0],
            &Move {
                num: 1,
                from: 2,
                to: 1
            }
        );
        assert_eq!(
            &moves[1],
            &Move {
                num: 3,
                from: 1,
                to: 3
            }
        );
    }

    #[test]
    fn test_p1() {
        let (crates, moves) = get_test_input();
        let test_ans = p1(&crates, &moves);
        assert_eq!(test_ans, "CMZ");

        let (crates, moves) = get_task_input();
        let ans = p1(&crates, &moves);
        assert_eq!(ans, "BWNCQRMDB");
    }

    #[test]
    fn test_p2() {
        let (crates, moves) = get_test_input();
        let test_ans = p2(&crates, &moves);
        assert_eq!(test_ans, "MCD");

        let (crates, moves) = get_task_input();
        let ans = p2(&crates, &moves);
        assert_eq!(ans, "NHWZCBNBF");
    }
}
