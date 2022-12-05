use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Crates<const N: usize> {
    pub state: [Vec<char>; N],
}

impl<const N: usize> Crates<N> {
    pub fn get_top_crates(&self) -> String {
        let mut ans = String::new();
        for cr in &self.state {
            ans.push(*cr.last().unwrap());
        }
        ans
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    from: usize,
    to: usize,
    num: usize,
}

fn p1<const N: usize>(crates: &Crates<N>, moves: &Vec<Move>) -> String {
    let mut crates = crates.clone();
    for mv in moves {
        for _ in 0..mv.num {
            let x = crates.state[mv.from - 1].pop().unwrap();
            crates.state[mv.to - 1].push(x);
        }
    }

    crates.get_top_crates()
}

fn p2<const N: usize>(crates: &Crates<N>, moves: &Vec<Move>) -> String {
    let mut crates = crates.clone();
    for mv in moves {
        let source_crate = &mut crates.state[mv.from - 1];
        let start_idx = source_crate.len() - mv.num;
        let mut elems = source_crate.drain(start_idx..).collect::<Vec<_>>();

        crates.state[mv.to - 1].append(&mut elems);
    }

    crates.get_top_crates()
}

fn read_input<const N: usize>(path: &str) -> (Crates<N>, Vec<Move>) {
    let content = fs::read_to_string(path).unwrap();
    let mut lines = content.trim_end().split("\n");
    let mut line = lines.next().unwrap();

    const EMPTY: Vec<char> = vec![];
    let mut crates = Crates { state: [EMPTY; N] };
    while !line.starts_with(" 1") {
        let chars = line.chars().collect::<Vec<_>>();
        for crate_id in 0..N {
            if let Some(&ch) = chars.get(crate_id * 4 + 1) {
                if ch != ' ' {
                    crates.state[crate_id].push(ch);
                }
            }
        }

        line = lines.next().unwrap();
    }

    for crate_id in 0..N {
        crates.state[crate_id].reverse();
    }

    lines.next();
    let mut moves = vec![];
    for line in lines {
        let mut it = line.split_ascii_whitespace();
        it.next();
        let num: usize = it.next().unwrap().parse().unwrap();
        it.next();
        let from: usize = it.next().unwrap().parse().unwrap();
        it.next();
        let to: usize = it.next().unwrap().parse().unwrap();

        moves.push(Move { from, to, num });
    }

    (crates, moves)
}

const INPUT_PATH: &'static str = "../inputs/d05";

fn main() {
    let (crates, moves) = read_input::<9>(INPUT_PATH);
    let p1_ans = p1(&crates, &moves);
    println!("P1: {p1_ans}.");

    let p2_ans = p2(&crates, &moves);
    println!("P2: {p2_ans}.");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PATH: &'static str = "../inputs/d05_test";

    #[test]
    fn test_input_parsing_test() {
        let (crates, moves) = read_input::<3>(TEST_PATH);

        assert_eq!(crates.state[0], vec!['Z', 'N']);
        assert_eq!(crates.state[1], vec!['M', 'C', 'D']);
        assert_eq!(crates.state[2], vec!['P']);

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
        let (crates, moves) = read_input::<3>(TEST_PATH);
        let test_ans = p1(&crates, &moves);
        assert_eq!(test_ans, "CMZ");

        let (crates, moves) = read_input::<9>(INPUT_PATH);
        let ans = p1(&crates, &moves);
        assert_eq!(ans, "BWNCQRMDB");
    }

    #[test]
    fn test_p2() {
        let (crates, moves) = read_input::<3>(TEST_PATH);
        let test_ans = p2(&crates, &moves);
        assert_eq!(test_ans, "MCD");

        let (crates, moves) = read_input::<9>(INPUT_PATH);
        let ans = p2(&crates, &moves);
        assert_eq!(ans, "NHWZCBNBF");
    }
}
