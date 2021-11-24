use std::fs;

fn part1() -> u32 {
    0
}

fn part2() -> u32 {
    0
}

fn main() {
    let now = std::time::Instant::now();

    let contents = fs::read_to_string("input/day00.txt").expect("can't find file");

    let ans_part1 = part1();
    println!("part1: {}", ans_part1);

    let ans_part2 = part2();
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
