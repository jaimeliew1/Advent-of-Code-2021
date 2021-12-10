use std::fs;

fn parse_input(filename: &str) -> Vec<u64> {
    let contents = fs::read_to_string(filename).expect("can't find file");
    let data: Vec<u64> = contents.split(",").map(|s| s.parse().unwrap()).collect();
    let mut fishes: Vec<u64> = vec![0; 9];
    data.iter().for_each(|&f| fishes[f as usize] += 1);
    fishes
}

fn simulate(fishes: &Vec<u64>, days: usize) -> Vec<u64> {
    // simulate lanternfishes of given ages for number of days.
    let mut fishes = fishes.clone();
    for _day in 0..days {
        let new_fishes = fishes[0];
        fishes = fishes[1..].to_vec();
        fishes.push(new_fishes);
        fishes[6] += new_fishes;
    }

    fishes
}

fn part1(fishes: &Vec<u64>) -> u64 {
    simulate(fishes, 80).iter().sum()
}

fn part2(fishes: &Vec<u64>) -> u64 {
    simulate(fishes, 256).iter().sum()
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
