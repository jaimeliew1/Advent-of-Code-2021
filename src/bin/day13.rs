use itertools::Itertools;
use std::fs;

fn parse_input(filename: &str) -> (Vec<(u32, u32)>, Vec<(String, u32)>) {
    let content = fs::read_to_string(filename).expect("can't find file");
    let (raw1, raw2) = content.split_once("\n\n").unwrap();

    let points: Vec<(u32, u32)> = raw1
        .lines()
        .map(|s| s.split_once(",").unwrap())
        .map(|(s1, s2)| (s1.parse().unwrap(), s2.parse().unwrap()))
        .collect();

    let folds: Vec<(String, u32)> = raw2
        .lines()
        .map(|s| s.rsplit_once(" ").unwrap())
        .map(|(_, s2)| s2.split_once("=").unwrap())
        .map(|(s1, s2)| (s1.to_string(), s2.parse().unwrap()))
        .collect();
    (points, folds)
}

fn fold_data(data: &Vec<(u32, u32)>, axis: &str, line: u32) -> Vec<(u32, u32)> {
    data.iter()
        .map(|(xx, yy)| match (*xx, *yy, axis) {
            (x, y, "x") if x > line => (2 * line - x, y),
            (x, y, "y") if y > line => (x, 2 * line - y),
            (x, y, _) => (x, y),
        })
        .collect()
}

fn print_points(data: &Vec<(u32, u32)>) {
    let xmax: usize = data.iter().map(|(x, _)| *x).max().unwrap() as usize;
    let ymax: usize = data.iter().map(|(_, y)| *y).max().unwrap() as usize;

    let mut out = vec![vec![' '; xmax + 1]; ymax + 1];
    for (x, y) in data {
        out[*y as usize][*x as usize] = '#';
    }
    for line in out {
        println!("{}", line.into_iter().collect::<String>());
    }
}

fn part1(data: &Vec<(u32, u32)>, folds: &Vec<(String, u32)>) -> u32 {
    let data = fold_data(&data, &folds[0].0, folds[0].1);
    data.iter().unique().count() as u32
}

fn part2(data: &Vec<(u32, u32)>, folds: &Vec<(String, u32)>) -> u32 {
    let out = folds.iter().fold(data.clone(), |acc, (axis, line)| {
        fold_data(&acc, axis, *line)
    });
    print_points(&out);
    0
}

fn main() {
    let now = std::time::Instant::now();

    let (points, folds) = parse_input("input/day13.txt");

    let ans_part1 = part1(&points, &folds);
    println!("part1: {}", ans_part1);

    let _ = part2(&points, &folds);
    println!("part2: {}", "ARHZPCUH");

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
