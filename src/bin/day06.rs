use std::collections::HashMap;
use std::fs;

fn parse_input(filename: &str) -> Vec<u64> {
    let contents = fs::read_to_string(filename).expect("can't find file");
    let data: Vec<u64> = contents.split(",").map(|s| s.parse().unwrap()).collect();
    data
}

fn simulate(mut fishes: Vec<u64>, days: usize) -> Vec<u64> {
    // simulate lanternfishes of given ages for number of days.
    for day in 0..days {
        let n_new_fishes = fishes.iter().filter(|&&f| f == 0).count();
        (0..n_new_fishes).for_each(|_| fishes.push(9));
        fishes = fishes
            .iter()
            .map(|&f| match f {
                0 => 6,
                x => x - 1,
            })
            .collect();
    }
    fishes
}

fn part1(data: &Vec<u64>) -> u64 {
    let mut count_lookup = HashMap::new();
    for age in 0..8 {
        count_lookup.insert(age, simulate(vec![age], 80).iter().count());
    }
    data.iter()
        .fold(0, |acc, f| acc + *count_lookup.get(f).unwrap() as u64)
}

fn part2(data: &Vec<u64>) -> u64 {
    let mut count_lookup = HashMap::new();
    for age in 0..9 {
        count_lookup.insert(age, simulate(vec![age], 156).iter().count());
    }
    let fishes_inter = simulate(data.to_vec(), 100);
    fishes_inter
        .iter()
        .fold(0, |acc, f| acc + *count_lookup.get(f).unwrap() as u64)
}

fn main() {
    let now = std::time::Instant::now();

    let data = parse_input("input/day06.txt");

    let ans_part1 = part1(&data);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&data);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
