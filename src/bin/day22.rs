use itertools::Itertools;
use regex::Regex;
use std::fs;

fn parse_input(filename: &str) -> Vec<(bool, Vec<i32>)> {
    let re = Regex::new(r"(\w+) x=(-*\d+)..(-*\d+),y=(-*\d+)..(-*\d+),z=(-*\d+)..(-*\d+)").unwrap();
    let contents = fs::read_to_string(filename).expect("can't find file");

    let mut data: Vec<(bool, Vec<i32>)> = Vec::new();
    for line in contents.lines() {
        let caps = re.captures(&line).unwrap();
        let on = match &caps[1] {
            "on" => true,
            _ => false,
        };
        let coords: Vec<i32> = caps
            .iter()
            .skip(2)
            .map(|x| x.unwrap().as_str().parse().unwrap())
            .collect();
        data.push((on, coords));
    }
    data
}

fn part1(data: &Vec<(bool, Vec<i32>)>) -> u32 {
    let data: Vec<(bool, Vec<usize>)> = data
        .iter()
        .filter(|(_, coords)| coords.iter().all(|x| x.abs() <= 50))
        .map(|(on, coords)| (*on, coords.iter().map(|v| (v + 50) as usize).collect()))
        .collect();

    let mut cubes = [[[false; 101]; 101]; 101];

    for (on, c) in data {
        for ((i, j), k) in (c[0]..c[1] + 1)
            .cartesian_product(c[2]..c[3] + 1)
            .cartesian_product(c[4]..c[5] + 1)
        {
            cubes[i][j][k] = on;
        }
    }
    cubes
        .iter()
        .flatten()
        .flatten()
        .fold(0, |acc, &val| acc + val as u32)
}

fn part2(data: &Vec<(bool, Vec<i32>)>) -> u32 {
    0
}

fn main() {
    let now = std::time::Instant::now();

    let data = parse_input("input/day22.txt");

    let ans_part1 = part1(&data);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&data);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
