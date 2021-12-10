use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref BRACKETS: HashMap<char, char> = vec![('[', ']'), ('(', ')'), ('{', '}'), ('<', '>')]
        .into_iter()
        .collect();
    static ref SCORES1: HashMap<char, u64> = vec![(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
        .into_iter()
        .collect();
    static ref SCORES2: HashMap<char, u64> = vec![(')', 1), (']', 2), ('}', 3), ('>', 4)]
        .into_iter()
        .collect();
}
fn parse_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("can't find file");
    let data: Vec<String> = contents.lines().map(|l| l.to_string()).collect();
    data
}

enum LineState<'a> {
    Corrupt(char),
    Incomplete(Vec<&'a char>),
}

fn process_bracket_string(input: &String) -> LineState {
    let mut expected = Vec::new();
    for c in input.chars() {
        match BRACKETS.get(&c) {
            Some(closing) => expected.push(closing),
            None if c == **expected.last().unwrap_or(&&'?') => {
                expected.pop().unwrap();
            }
            _ => {
                return LineState::Corrupt(c);
            }
        };
    }
    return LineState::Incomplete(expected.into_iter().rev().collect());
}

fn part1(data: &Vec<String>) -> u64 {
    let mut score: u64 = 0;
    for line in data {
        if let LineState::Corrupt(c) = process_bracket_string(line) {
            score += SCORES1.get(&c).unwrap()
        }
    }
    score
}

fn part2(data: &Vec<String>) -> u64 {
    let mut scores: Vec<u64> = Vec::new();
    for line in data {
        if let LineState::Incomplete(expected) = process_bracket_string(line) {
            scores.push(
                expected
                    .iter()
                    .fold(0, |acc, &x| acc * 5 + SCORES2.get(x).unwrap()),
            )
        }
    }
    scores.sort();
    let mid = scores.len() / 2;
    scores[mid]
}

fn main() {
    let now = std::time::Instant::now();

    let data = parse_input("input/day10.txt");

    let ans_part1 = part1(&data);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&data);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
