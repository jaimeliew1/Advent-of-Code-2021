use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

fn parse_input(filename: &str) -> (String, HashMap<String, (String, String)>) {
    let content = fs::read_to_string(filename).expect("can't find file");
    let (raw1, raw2) = content.split_once("\n\n").unwrap();

    let mut lookup = HashMap::new();
    for line in raw2.lines() {
        let (input, output) = line.split_once(" -> ").unwrap();
        let (mut out1, mut out2) = (input[0..1].to_owned(), output[0..1].to_owned());
        out1.push_str(output);
        out2.push_str(&input[1..2]);
        lookup.insert(input.to_owned(), (out1, out2));
    }
    (raw1.to_owned(), lookup)
}

fn polymer_count(input: &str, niter: usize, lookup: &HashMap<String, (String, String)>) -> u64 {
    let mut polymer = compress_polymer(input);
    for _ in 0..niter {
        polymer = step(&polymer, &lookup);
    }

    let mut count = HashMap::new();
    for (key, val) in polymer.iter() {
        key.chars()
            .for_each(|c| *count.entry(c).or_insert(0) += val);
    }

    // correct for boundary condition.
    *count.entry(input.chars().next().unwrap()).or_insert(0) += 1;
    *count.entry(input.chars().last().unwrap()).or_insert(0) += 1;

    (count.values().max().unwrap() - count.values().min().unwrap()) / 2
}

fn compress_polymer(input: &str) -> HashMap<String, u64> {
    input
        .chars()
        .collect_vec()
        .windows(2)
        .map(|w| (w.into_iter().collect(), 1))
        .collect()
}

fn step(
    input: &HashMap<String, u64>,
    lookup: &HashMap<String, (String, String)>,
) -> HashMap<String, u64> {
    let mut out = input.clone();
    for (key, val) in input.iter() {
        let val = *val;
        let (a, b) = lookup.get(key).unwrap();
        *out.get_mut(key).unwrap() -= val;
        *out.entry(a.to_string()).or_insert(0) += val;
        *out.entry(b.to_string()).or_insert(0) += val;
    }
    out
}

fn main() {
    let now = std::time::Instant::now();

    let (input, lookup) = parse_input("input/day14.txt");

    let ans_part1 = polymer_count(&input, 10, &lookup);
    println!("part1: {}", ans_part1);

    let ans_part2 = polymer_count(&input, 40, &lookup);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
