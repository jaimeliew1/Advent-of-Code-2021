use std::fs;

static NDIG: usize = 12;

fn int_vec_to_binary(input: &Vec<u32>) -> u32 {
    input
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| x * 2_u32.pow(i as u32))
        .fold(0, |acc, x| acc + x)
}

fn parse_input(filename: &str) -> Vec<Vec<u32>> {
    let contents = fs::read_to_string(filename).expect("can't find file");
    let data: Vec<Vec<u32>> = contents
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    data
}

fn part1(data: &Vec<Vec<u32>>) -> u32 {
    let mut count = vec![0; NDIG];
    for i in 0..NDIG {
        count[i] = data.iter().fold(0, |acc, x| acc + x[i]);
    }
    let gamma: u32 = count
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| match *x {
            x if x > 500 => 2_u32.pow(i as u32),
            _ => 0,
        })
        .fold(0, |acc, x| acc + x);

    let epsilon: u32 = 2_u32.pow(12) - 1 - gamma;

    gamma * epsilon
}
fn part2(in_data: &Vec<Vec<u32>>) -> u32 {
    let mut data = in_data.clone();
    for i in 0..NDIG {
        let most_common: u32 = match data.iter().fold(0, |acc, x| acc + x[i]) {
            x if x * 2 >= data.len() as u32 => 1,
            _ => 0,
        };
        data.retain(|x| x[i] == most_common);
        if data.len() == 1 {
            break;
        }
    }
    let ox_gen = int_vec_to_binary(&data[0]);

    let mut data = in_data.clone();
    for i in 0..NDIG {
        let least_common: u32 = match data.iter().fold(0, |acc, x| acc + x[i]) {
            x if x * 2 < data.len() as u32 => 1,
            _ => 0,
        };

        data.retain(|x| x[i] == least_common);
        if data.len() == 1 {
            break;
        }
    }
    let scrub_rate = int_vec_to_binary(&data[0]);

    ox_gen * scrub_rate
}

fn main() {
    let now = std::time::Instant::now();

    let data = parse_input("input/day03.txt");

    let ans_part1 = part1(&data);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&data);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
