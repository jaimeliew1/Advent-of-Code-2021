use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Copy, Clone)]
struct VentLine {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}
fn build_ventline(input: Vec<i32>) -> VentLine {
    VentLine {
        x1: input[0],
        y1: input[1],
        x2: input[2],
        y2: input[3],
    }
}
fn parse_input(filename: &str) -> Vec<VentLine> {
    let line_parser = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let contents = fs::read_to_string(filename).expect("can't find file");
    let mut data: Vec<VentLine> = Vec::new();
    for line in contents.lines() {
        let a: VentLine = build_ventline(
            line_parser
                .captures(line)
                .unwrap()
                .iter()
                .skip(1)
                .map(|cap| *&cap.unwrap().as_str().parse::<i32>().unwrap())
                .collect(),
        );

        data.push(a);
    }
    data
}

fn count_overlap(data: &Vec<VentLine>) -> u32 {
    let mut count = HashMap::new();
    for vent in data {
        let dx: i32 = (vent.x2 - vent.x1).signum();
        let dy: i32 = (vent.y2 - vent.y1).signum();
        let (mut x, mut y) = (vent.x1, vent.y1);
        while (x, y) != (vent.x2 + dx, vent.y2 + dy) {
            *count.entry((x, y)).or_insert(0) += 1;
            x += dx;
            y += dy;
        }
    }
    count.values().filter(|&&x| x > 1).count() as u32
}
fn part1(data: &Vec<VentLine>) -> u32 {
    let new_data: Vec<_> = data
        .iter()
        .filter(|&v| (v.x1 == v.x2) || (v.y1 == v.y2))
        .map(|&v| v)
        .collect();
    count_overlap(&new_data)
}

fn part2(data: &Vec<VentLine>) -> u32 {
    count_overlap(data)
}

fn main() {
    let now = std::time::Instant::now();

    let data = parse_input("input/day05.txt");

    let ans_part1 = part1(&data);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&data);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
