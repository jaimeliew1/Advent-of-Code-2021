use itertools::Itertools;
use std::fs;

fn parse_input(filename: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(filename).expect("can't find file");
    contents.lines().map(|s| s.chars().collect()).collect()
}

fn iter(input: &Vec<Vec<char>>) -> Option<Vec<Vec<char>>> {
    let (N, M) = (input.len(), input[0].len());
    let mut temp = input.clone();
    for (i, j) in (0..N).cartesian_product(0..M) {
        if (input[i][j], input[i][(j + 1) % M]) == ('>', '.') {
            temp[i][j] = '.';
            temp[i][(j + 1) % M] = '>';
        }
    }
    let mut output = temp.clone();
    for (i, j) in (0..N).cartesian_product(0..M) {
        if (temp[i][j], temp[(i + 1) % N][j]) == ('v', '.') {
            output[i][j] = '.';
            output[(i + 1) % N][j] = 'v';
        }
    }
    // return new layout if it has changed. Otherwise, return none.
    if output
        .iter()
        .zip(input)
        .all(|(a, b)| a.iter().zip(b).all(|(x, y)| x == y))
    {
        return None;
    } else {
        return Some(output);
    }
}

fn part1(data: &Vec<Vec<char>>) -> u32 {
    let mut old = data.clone();
    let mut count = 1;

    while let Some(layout) = iter(&old) {
        old = layout;
        count += 1;
    }

    count
}

fn main() {
    let now = std::time::Instant::now();
    let data = parse_input("input/day25.txt");

    let ans_part1 = part1(&data);
    println!("part1: {}", ans_part1);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
