use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_input(filename: &str) -> HashMap<String, HashSet<String>> {
    let contents = fs::read_to_string(filename).expect("can't find file");
    let data: Vec<(String, String)> = contents
        .lines()
        .map(|s| s.split_once("-").unwrap())
        .map(|(l, r)| (l.to_string(), r.to_string()))
        .collect();

    let mut segments = HashMap::new();
    for (left, right) in data {
        segments
            .entry(left.to_string())
            .or_insert(HashSet::from([right.to_string()]))
            .insert(right.to_string());
        segments
            .entry(right.to_string())
            .or_insert(HashSet::from([left.to_string()]))
            .insert(left.to_string());
    }
    segments
}

fn simulate(path: &Vec<String>, segments: &HashMap<String, HashSet<String>>) -> u32 {
    if path.last().unwrap() == "end" {
        return 1;
    }
    let mut npaths = 0;
    for segment in segments.get(path.last().unwrap()).unwrap() {
        if segment.chars().next().unwrap().is_uppercase() || !path.contains(segment) {
            let mut path_temp = path.clone();
            path_temp.push(segment.to_string());
            npaths += simulate(&path_temp, &segments);
        }
    }
    npaths
}

fn simulate2(path: &Vec<String>, segments: &HashMap<String, HashSet<String>>) -> u32 {
    let double_visit = path.iter().any(|x| {
        path.iter()
            .filter(|&y| x == y && !x.chars().next().unwrap().is_uppercase())
            .count()
            >= 2
    });

    if path.last().unwrap() == "end" {
        return 1;
    }

    let mut npaths = 0;
    for segment in segments.get(path.last().unwrap()).unwrap() {
        if *segment == "start" {
            continue;
        }
        if segment.chars().next().unwrap().is_uppercase()
            || !path.contains(segment)
            || !double_visit
        {
            let mut path_temp = path.clone();
            path_temp.push(segment.to_string());
            npaths += simulate2(&path_temp, &segments);
        }
    }
    npaths
}
fn part1(segments: &HashMap<String, HashSet<String>>) -> u32 {
    let npaths = simulate(&vec!["start".to_string()], &segments);
    npaths
}
fn part2(segments: &HashMap<String, HashSet<String>>) -> u32 {
    let npaths = simulate2(&vec!["start".to_string()], &segments);
    npaths
}

fn main() {
    let now = std::time::Instant::now();

    let data = parse_input("input/day12.txt");

    let ans_part1 = part1(&data);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&data);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
