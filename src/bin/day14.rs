use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn parse_input(filename: &str) -> (String, HashMap<(char, char), char>) {
    let content = fs::read_to_string(filename).expect("can't find file");
    let (raw1, raw2) = content.split_once("\n\n").unwrap();

    let mut lookup = HashMap::new();
    for line in raw2.lines() {
        let (input, output) = line.split_once(" -> ").unwrap();
        lookup.insert(
            (input.as_bytes()[0] as char, input.as_bytes()[1] as char),
            output.as_bytes()[0] as char,
        );
    }
    (raw1.to_owned(), lookup)
}

fn polymer_count(input: &str, niter: usize, lookup: &HashMap<(char, char), char>) -> u64 {
    let compressed_input: HashMap<(char, char), usize> =
        input.chars().into_iter().tuple_windows().counts();
    let polymer = (0..niter).fold(compressed_input, |acc, _| step(&acc, &lookup));

    let mut count = HashMap::new();
    for ((a, b), val) in polymer.iter() {
        *count.entry(*a).or_insert(0) += val;
        *count.entry(*b).or_insert(0) += val;
    }

    // correct for boundary condition.
    *count.entry(input.chars().next().unwrap()).or_insert(0) += 1;
    *count.entry(input.chars().last().unwrap()).or_insert(0) += 1;

    let (min, max) = count.values().minmax().into_option().unwrap();
    ((max - min) / 2) as u64
}

fn step(
    input: &HashMap<(char, char), usize>,
    lookup: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    let mut out = input.clone();
    for ((a, b), val) in input.iter() {
        let val = *val;
        let c = lookup.get(&(*a, *b)).unwrap();
        *out.get_mut(&(*a, *b)).unwrap() -= val;
        *out.entry((*a, *c)).or_insert(0) += val;
        *out.entry((*c, *b)).or_insert(0) += val;
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
