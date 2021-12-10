use std::fs;

fn parse_input(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename).expect("can't find file");
    let data: Vec<i32> = contents.split(",").map(|s| s.parse().unwrap()).collect();
    data
}

fn part1(data: &Vec<i32>) -> i32 {
    let mut data = data.clone();
    data.sort();
    let mid = data.len() / 2;
    let median: i32 = match data.len() % 2 {
        0 => (data[mid - 1] + data[mid]) / 2,
        1 => data[mid],
        _ => unreachable!(),
    };
    data.iter().fold(0, |acc, x| acc + (x - median).abs())
}

fn part2(data: &Vec<i32>) -> i32 {
    let mean = data.iter().sum::<i32>() / (data.len() as i32);
    data.iter()
        .map(|x| (x - mean).abs())
        .map(|x| x * (x + 1) / 2)
        .sum()
}

fn main() {
    let now = std::time::Instant::now();

    let data = parse_input("input/day07.txt");

    let ans_part1 = part1(&data);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&data);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
