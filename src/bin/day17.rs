use regex::Regex;
use std::fs;

fn parse_input(filename: &str) -> (i64, i64, i64, i64) {
    let re = Regex::new(r"target area: x=(-*\d+)..(-*\d+), y=(-*\d+)..(-*\d+)").unwrap();
    let contents = fs::read_to_string(filename).expect("can't find file");
    let caps = re.captures(&contents).unwrap();
    (
        caps[1].parse().unwrap(),
        caps[2].parse().unwrap(),
        caps[3].parse().unwrap(),
        caps[4].parse().unwrap(),
    )
}

fn part1((_xmin, _xmax, ymin, _ymax): &(i64, i64, i64, i64)) -> i64 {
    ymin.abs() * (ymin.abs() - 1) / 2
}

fn part2((xmin, xmax, ymin, ymax): &(i64, i64, i64, i64)) -> usize {
    let mut count = 0;
    for uvel in 0..*xmax + 1 {
        for vvel in *ymin..(-*ymin + 1) {
            let (mut u, mut v) = (uvel, vvel);
            let (mut x, mut y) = (0, 0);
            while y >= *ymin {
                if x >= *xmin && x <= *xmax && y >= *ymin && y <= *ymax {
                    count += 1;
                    break;
                }
                x += u;
                y += v;
                v -= 1;
                u = match u {
                    u if u == 0 => 0,
                    u => u - 1,
                };
            }
        }
    }
    count
}

fn main() {
    let now = std::time::Instant::now();

    let data = parse_input("input/day17.txt");

    let ans_part1 = part1(&data);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&data);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
