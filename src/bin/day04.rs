use itertools::Itertools;
use std::fs;
#[derive(Debug, Clone, Copy)]
struct BingoCard([[u64; 5]; 5]);

impl BingoCard {
    fn from_string(input: &str) -> BingoCard {
        let mut arr: [[u64; 5]; 5] = [[0; 5]; 5];
        for (arr_row, str_row) in arr.iter_mut().zip(input.split("\n")) {
            for (arr_elem, str_elem) in arr_row.iter_mut().zip(str_row.split_whitespace()) {
                *arr_elem = str_elem.parse().unwrap();
            }
        }
        BingoCard(arr)
    }

    fn get_pos(self, val: &u64) -> Option<(usize, usize)> {
        let (x, y): (usize, usize);
        match self.0.iter().position(|row| row.contains(val)) {
            Some(pos) => y = pos,
            None => return None,
        }
        x = self.0[y].iter().position(|x| x == val).unwrap();
        Some((x, y))
    }

    fn solved(self, draws: &Vec<(usize, usize)>) -> Option<u64> {
        let solved_col: Vec<usize> = (0..5)
            .into_iter()
            .filter(|col| draws.iter().filter(|(_, y)| *y == *col as usize).count() == 5)
            .collect();

        let solved_row: Vec<usize> = (0..5)
            .into_iter()
            .filter(|row| draws.iter().filter(|(x, _)| *x == *row as usize).count() == 5)
            .collect();

        if (solved_row.len() == 1) || (solved_col.len() == 1) {
            let mut sum_unmarked: u64 = 0;
            for (i, row) in self.0.iter().enumerate() {
                for (j, elem) in row.iter().enumerate() {
                    if !draws.contains(&(j, i)) {
                        sum_unmarked += elem;
                    }
                }
            }
            return Some(sum_unmarked);
        }

        None
    }
}

fn parse_input(filename: &str) -> (Vec<u64>, Vec<BingoCard>) {
    let contents = fs::read_to_string(filename).expect("can't find file");
    let data: Vec<&str> = contents.split("\n\n").collect();
    let ans1: Vec<u64> = data[0].split(",").map(|s| s.parse().unwrap()).collect();
    let ans2: Vec<BingoCard> = data[1..]
        .iter()
        .map(|&s| BingoCard::from_string(s))
        .collect();
    (ans1, ans2)
}

fn solve(draws: &Vec<u64>, card: &BingoCard) -> (usize, u64) {
    let mut list: Vec<(usize, usize)> = Vec::new();

    for (i, draw) in draws.iter().enumerate() {
        if let Some((x, y)) = card.get_pos(draw) {
            list.push((x, y));
        }
        if let Some(solution) = card.solved(&list) {
            return (i, solution * draw);
        }
    }
    unreachable!()
}

fn main() {
    let now = std::time::Instant::now();

    let (draws, cards) = parse_input("input/day04.txt");
    let scores: Vec<(usize, u64)> = cards
        .iter()
        .map(|c| solve(&draws, c))
        .sorted_by_key(|(niter, _score)| *niter)
        .collect();

    let ans_part1 = scores[0].1;
    println!("part1: {}", ans_part1);

    let ans_part2 = scores.last().unwrap().1;
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
