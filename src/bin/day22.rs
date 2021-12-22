use itertools::Itertools;
use regex::Regex;
use std::fs;

fn parse_input(filename: &str) -> Vec<(bool, Vec<i64>)> {
    let re = Regex::new(r"(\w+) x=(-*\d+)..(-*\d+),y=(-*\d+)..(-*\d+),z=(-*\d+)..(-*\d+)").unwrap();
    let contents = fs::read_to_string(filename).expect("can't find file");

    let mut data: Vec<(bool, Vec<i64>)> = Vec::new();
    for line in contents.lines() {
        let caps = re.captures(&line).unwrap();
        let on = match &caps[1] {
            "on" => true,
            _ => false,
        };
        let coords: Vec<i64> = caps
            .iter()
            .skip(2)
            .map(|x| x.unwrap().as_str().parse().unwrap())
            .collect();
        data.push((on, coords));
    }
    data
}

fn count_activated_cubes(data: &Vec<(bool, Vec<i64>)>) -> u64 {
    let xs: Vec<i64> = data
        .iter()
        .map(|(_, coords)| vec![coords[0], coords[1] + 1])
        .flatten()
        .unique()
        .sorted()
        .collect();
    let ys: Vec<i64> = data
        .iter()
        .map(|(_, coords)| vec![coords[2], coords[3] + 1])
        .flatten()
        .unique()
        .sorted()
        .collect();
    let zs: Vec<i64> = data
        .iter()
        .map(|(_, coords)| vec![coords[4], coords[5] + 1])
        .flatten()
        .unique()
        .sorted()
        .collect();
    let mut count: u64 = 0;

    for (i, (x, xx)) in xs.iter().tuple_windows().enumerate() {
        // println!("{}/{}", i, xs.len());
        let candids: Vec<&(bool, Vec<i64>)> = data
            .iter()
            .filter(|(_, c)| (x >= &c[0]) & (x <= &c[1]))
            .collect();
        for (y, yy) in ys.iter().tuple_windows() {
            let candids2: Vec<&&(bool, Vec<i64>)> = candids
                .iter()
                .filter(|(_, c)| (y >= &c[2]) & (y <= &c[3]))
                .collect();
            for (z, zz) in zs.iter().tuple_windows() {
                let blah = candids2
                    .iter()
                    .rev()
                    .filter(|(_, c)| (z >= &c[4]) & (z <= &c[5]))
                    .next();
                count += match blah {
                    Some((true, _)) => ((xx - x) * (yy - y) * (zz - z)) as u64,
                    _ => 0,
                };
            }
        }
    }

    count
}

fn main() {
    let now = std::time::Instant::now();

    let data = parse_input("input/day22.txt");
    let startup_data: Vec<(bool, Vec<i64>)> = data
        .iter()
        .filter(|(_, coords)| coords.iter().all(|x| x.abs() <= 50))
        .map(|(on, coords)| (*on, coords.iter().map(|v| *v).collect()))
        .collect();

    let ans_part1 = count_activated_cubes(&startup_data);
    println!("part1: {}", ans_part1);

    println!("This will take a couple of minutes...");
    let ans_part2 = count_activated_cubes(&data);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
