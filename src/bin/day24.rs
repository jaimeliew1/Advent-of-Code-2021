use std::fs;

fn parse_input(filename: &str) -> Vec<(u32, u64, u32)> {
    let contents = fs::read_to_string(filename).expect("can't find file");
    let data: Vec<Vec<&str>> = contents
        .split("inp w\n")
        .skip(1)
        .map(|s| s.lines().collect())
        .collect();

    let mut stack = Vec::new();
    let mut equalities = Vec::new();
    for (digit, line) in data.iter().enumerate() {
        let shift = line[3].strip_prefix("div z ").unwrap();
        let check = line[4].strip_prefix("add x ").unwrap().parse::<i32>().unwrap();
        let add = line[14].strip_prefix("add y ").unwrap().parse::<i32>().unwrap();
        if shift == "1"{
            stack.push((digit as u32, add));
         } else {
            let popped = stack.pop().unwrap();
            match popped.1 + check {
                x if x > 0 => equalities.push((popped.0, x as u64, digit as u32)),
                x => equalities.push((digit as u32, (-x) as u64, popped.0)),
            }
        }
    }
    equalities
}

fn part1(equalities: &Vec<(u32, u64, u32)>) -> u64 {
    equalities.iter().fold(0, |acc, &(lhs, add, rhs)| {
        acc + (9 - add) * 10_u64.pow(13 - lhs) + 9 * 10_u64.pow(13 - rhs)
    })
}

fn part2(equalities: &Vec<(u32, u64, u32)>) -> u64 {
    equalities.iter().fold(0, |acc, &(lhs, add, rhs)| {
        acc + 1 * 10_u64.pow(13 - lhs) + (1 + add) * 10_u64.pow(13 - rhs)
    })
}

fn print_equalities(equalities: &Vec<(u32, u64, u32)>) {
    let chars: Vec<char> = "ABCDEFGHIJKLMN".chars().collect();
    for &(lhs, add, rhs) in equalities {
        println!("{} + {} == {}",chars[lhs as usize], add, chars[rhs as usize]);
    }
}
fn main() {
    let now = std::time::Instant::now();

    let equalities = parse_input("input/day24.txt");
    print_equalities(&equalities);

    let ans_part1 = part1(&equalities);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&equalities);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
