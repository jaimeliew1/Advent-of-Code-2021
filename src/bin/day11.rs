use lazy_static::lazy_static;
use std::fs;

lazy_static! {
    static ref NEIGHB: Vec<(i32, i32)> = vec![
        (-1, 0),
        (1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, -1),
        (-1, 1),
    ];
}

fn parse_input(filename: &str) -> Vec<Vec<u32>> {
    let contents = fs::read_to_string(filename).expect("can't find file");
    let mut data: Vec<Vec<u32>> = contents
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    data.iter_mut().for_each(|row| {
        row.insert(0, 0);
        row.push(0)
    });
    data.insert(0, vec![0; data[0].len()]);
    data.push(vec![0; data[0].len()]);
    data
}

fn get_new_flashes(
    data: &Vec<Vec<u32>>,
    flashes: &Vec<(usize, usize)>,
) -> Option<Vec<(usize, usize)>> {
    let mut new_flashes = Vec::new();
    for i in 1..11 {
        for j in 1..11 {
            if data[i][j] > 9 && !flashes.contains(&(i, j)) {
                new_flashes.push((i, j));
            }
        }
    }

    if !new_flashes.is_empty() {
        return Some(new_flashes);
    }
    None
}
fn part1(data: &Vec<Vec<u32>>) -> u32 {
    let mut data = data.clone();
    let mut count = 0;
    for _ in 0..100 {
        let (dat, n_flashes) = step(&data);
        data = dat;
        count += n_flashes;
    }
    count
}

fn step(data_in: &Vec<Vec<u32>>) -> (Vec<Vec<u32>>, u32) {
    let mut data = data_in.clone();

    // add 1 to all elements.
    data.iter_mut().flatten().for_each(|c| *c += 1);

    // iterate until no more new flashes.
    let mut flashes = Vec::new();
    while let Some(newflashes) = get_new_flashes(&data, &flashes) {
        for (x0, y0) in &newflashes {
            for (dx, dy) in NEIGHB.iter() {
                let (x, y) = ((*x0 as i32 + *dx) as usize, (*y0 as i32 + *dy) as usize);
                if !flashes.contains(&(x, y)) {
                    data[x][y] += 1;
                }
            }
        }
        flashes.extend(newflashes);
    }
    // set flashed elements to zero.
    flashes.iter().for_each(|&(x, y)| data[x][y] = 0 );

    (data, flashes.len() as u32)
}

fn part2(data: &Vec<Vec<u32>>) -> u32 {
    let (mut count, mut iters) = (0, 0);
    let mut data = data.clone();
    while count != 100 {
        let (dat, n_flashes) = step(&data);
        data = dat;

        count = n_flashes;
        iters += 1;
    }

    iters
}

fn main() {
    let now = std::time::Instant::now();

    let data = parse_input("input/day11.txt");

    let ans_part1 = part1(&data);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&data);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
