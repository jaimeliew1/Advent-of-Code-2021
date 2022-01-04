use itertools::Itertools;
use std::fs;

fn parse_num(s: &str) -> Vec<(u32, i32)> {
    let mut depth = -1;
    let mut out: Vec<(u32, i32)> = Vec::new();
    for c in s.chars() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' => (),
            _ => out.push((c.to_digit(10).unwrap(), depth)),
        }
    }
    out
}

fn parse_input(filename: &str) -> Vec<Vec<(u32, i32)>> {
    let contents = fs::read_to_string(filename).expect("can't find file");
    contents.lines().map(|l| parse_num(l)).collect()
}

fn add(x: &Vec<(u32, i32)>, y: &Vec<(u32, i32)>) -> Vec<(u32, i32)> {
    let mut out: Vec<(u32, i32)> = x.iter().map(|&(v, d)| (v, d + 1)).collect();
    out.extend(y.iter().map(|&(v, d)| (v, d + 1)));
    out
}

fn reduce(x: &Vec<(u32, i32)>) -> Vec<(u32, i32)> {
    let mut out = x.clone();
    while !out.iter().all(|&(v, d)| (v <= 9) & (d < 4)) {
        out = explode_all(&out);
        out = split_one(&out);
    }
    out
}

fn explode_all(x: &Vec<(u32, i32)>) -> Vec<(u32, i32)> {
    let mut out = x.clone();
    while let Some((pos, _)) = out
        .iter()
        .tuple_windows()
        .find_position(|(&(_, d1), &(_, d2))| (d1 == 4) & (d2 == 4))
    {
        if pos != 0 {
            out[pos - 1].0 += out[pos].0;
        }
        if pos < out.len() - 2 {
            out[pos + 2].0 += out[pos + 1].0;
        }
        out.remove(pos);
        out[pos] = (0, 3);
    }

    out
}

fn split_one(x: &Vec<(u32, i32)>) -> Vec<(u32, i32)> {
    let mut out = x.clone();
    if let Some((pos, (v, d))) = x.iter().find_position(|(v, _)| *v >= 10) {
        out.insert(pos + 1, (v / 2 + v % 2, d + 1));
        out[pos] = (v / 2, d + 1);
    }
    out
}

fn magnitude(x: &Vec<(u32, i32)>) -> u32 {
    let mut out = x.clone();
    for depth in [3, 2, 1, 0] {
        while let Some((pos, _)) = out
            .iter()
            .tuple_windows()
            .find_position(|(&(_, d1), &(_, d2))| (d1 == depth) & (d2 == depth))
        {
            out[pos] = (3 * out[pos].0 + 2 * out[pos + 1].0, depth - 1);
            out.remove(pos + 1);
        }
    }
    out[0].0
}

fn part1(data: &Vec<Vec<(u32, i32)>>) -> u32 {
    let mut sum = data[0].clone();
    for number in data.iter().skip(1) {
        sum = reduce(&add(&sum, number));
    }
    magnitude(&sum)
}

fn part2(data: &Vec<Vec<(u32, i32)>>) -> u32 {
    data.iter()
        .cartesian_product(data)
        .map(|(x, y)| magnitude(&reduce(&add(x, y))))
        .max()
        .unwrap()
}

fn main() {
    let now = std::time::Instant::now();

    let data = parse_input("input/day18.txt");

    let ans_part1 = part1(&data);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&data);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
