use std::fs;

fn parse_input(filename: &str) -> Vec<(String, i32)> {
    let contents = fs::read_to_string(filename).expect("can't find file");

    contents
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(command, val)| (command.to_string(), val.parse().unwrap()))
        .collect()
}

fn part1(data: &Vec<(String, i32)>) -> u32 {
    let (x, y): (i32, i32) = data
        .iter()
        .map(|(command, val)| match command.as_str() {
            "forward" => (*val, 0),
            "up" => (0, -*val),
            "down" => (0, *val),
            _ => panic!(),
        })
        .fold((0, 0), |(x, y), (dx, dy)| (x + dx, y + dy));

    (x * y) as u32
}

fn part2(data: &Vec<(String, i32)>) -> u32 {
    let (mut x, mut y, mut aim): (i32, i32, i32) = (0, 0, 0);
    for (command, val) in data.iter() {
        match command.as_str() {
            "forward" => {
                x += val;
                y += aim * val;
            }
            "up" => aim -= val,
            "down" => aim += val,
            _ => (),
        }
    }
    (x * y) as u32
}

fn main() {
    let now = std::time::Instant::now();

    let data = parse_input("input/day02.txt");

    let ans_part1 = part1(&data);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&data);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
