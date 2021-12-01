use std::fs;

fn part1(inp_vec: &Vec<i32>) -> u32 {
    inp_vec
        .windows(2)
        .map(|x| x[1] - x[0])
        .filter(|&x| x > 0)
        .count() as u32
}

fn part2(inp_vec: &Vec<i32>) -> u32 {
    inp_vec
        .windows(3)
        .map(|x| x[0] + x[1] + x[2])
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|x| x[1] - x[0])
        .filter(|&x| x > 0)
        .count() as u32
}

fn main() {
    let now = std::time::Instant::now();

    let contents = fs::read_to_string("input/day01.txt").expect("can't find file");
    let inp_vec: Vec<i32> = contents.lines().map(|s| s.parse().unwrap()).collect();

    let ans_part1 = part1(&inp_vec);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&inp_vec);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
