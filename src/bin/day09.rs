use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::fs;

lazy_static! {
    static ref NEIGHB: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
}

fn parse_input(filename: &str) -> Vec<Vec<u32>> {
    let contents = fs::read_to_string(filename).expect("can't find file");
    let mut data: Vec<Vec<u32>> = contents
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    data.iter_mut().for_each(|row| {
        row.insert(0, 99);
        row.push(99)
    });
    data.insert(0, vec![99; data[0].len()]);
    data.push(vec![99; data[0].len()]);
    data
}

fn get_low_points(data: &Vec<Vec<u32>>) -> Vec<(i32, i32)> {
    let mut low_points: Vec<(i32, i32)> = Vec::new();
    for i in 1..101 {
        for j in 1..101 {
            let neighbors: Vec<u32> = NEIGHB
                .iter()
                .map(|(dx, dy)| data[((i as i32) + dx) as usize][((j as i32) + dy) as usize])
                .collect();
            if neighbors.iter().all(|&n| n > data[i][j]) {
                low_points.push((i as i32, j as i32));
            }
        }
    }
    low_points
}
fn part1(data: &Vec<Vec<u32>>) -> u32 {
    get_low_points(&data)
        .iter()
        .map(|&(x, y)| data[x as usize][y as usize] + 1)
        .sum()
}

fn part2(data: &Vec<Vec<u32>>) -> u32 {
    let low_points = get_low_points(&data);

    let mut sizes: Vec<u32> = Vec::new();
    for (low_x, low_y) in low_points {
        let mut basin_points: HashSet<(i32, i32)> = vec![(low_x, low_y)].into_iter().collect();
        let mut last_size = 999;
        while basin_points.len() != last_size {
            last_size = basin_points.len();
            for (x, y) in basin_points.clone() {
                for (dx, dy) in NEIGHB.iter() {
                    if data[(x + dx) as usize][(y + dy) as usize] < 9 {
                        basin_points.insert((x + dx, y + dy));
                    }
                }
            }
        }
        sizes.push(basin_points.len() as u32);
    }
    sizes.iter().sorted().rev().take(3).product()
}

fn main() {
    let now = std::time::Instant::now();

    let data = parse_input("input/day09.txt");

    let ans_part1 = part1(&data);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&data);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
