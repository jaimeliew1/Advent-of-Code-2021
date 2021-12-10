use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref SCORE2DIGIT: HashMap<u32, u32> = vec![
        (42, 0),
        (17, 1),
        (34, 2),
        (39, 3),
        (30, 4),
        (37, 5),
        (41, 6),
        (25, 7),
        (49, 8),
        (45, 9),
    ]
    .into_iter()
    .collect();
}

fn parse_input(filename: &str) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let contents = fs::read_to_string(filename).expect("can't find file");
    let mut data_in: Vec<Vec<String>> = Vec::new();
    let mut data_out: Vec<Vec<String>> = Vec::new();
    for line in contents.lines() {
        let (input, output) = line.split_once("|").unwrap();
        data_in.push(
            input
                .split_whitespace()
                .map(|s| s.chars().sorted().collect::<String>())
                .collect(),
        );
        data_out.push(
            output
                .split_whitespace()
                .map(|s| s.chars().sorted().collect::<String>())
                .collect(),
        );
    }
    (data_in, data_out)
}

fn part1((_, data_out): &(Vec<Vec<String>>, Vec<Vec<String>>)) -> u32 {
    data_out.iter().fold(0, |acc, list| {
        acc + list
            .iter()
            .filter(|&s| vec![2, 3, 4, 7].contains(&s.len()))
            .count()
    }) as u32
}


fn part2((data_in, data_out): &(Vec<Vec<String>>, Vec<Vec<String>>)) -> u32 {
    let mut solution: u32 = 0;
    for (din, dout) in data_in.iter().zip(data_out) {
        let lookup = identify(din);
        solution += dout
            .iter()
            .rev()
            .map(|s| lookup.get(s).unwrap())
            .enumerate()
            .fold(0, |acc, (i, x)| acc + x * 10_u32.pow(i as u32));
    }
    solution
}

fn identify(input: &Vec<String>) -> HashMap<String, u32> {
    let mut frequency = HashMap::new();
    for digit in input {
        for segment in digit.chars() {
            *frequency.entry(segment).or_insert(0) += 1;
        }
    }
    let scores: Vec<u32> = input
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| frequency.get(&c).unwrap())
                .sum()
        })
        .collect();

    input
        .iter()
        .zip(scores)
        .map(|(st, sc)| (st.to_string(), *SCORE2DIGIT.get(&sc).unwrap()))
        .collect()
}

fn main() {
    let now = std::time::Instant::now();

    let data = parse_input("input/day08.txt");

    let ans_part1 = part1(&data);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&data);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
