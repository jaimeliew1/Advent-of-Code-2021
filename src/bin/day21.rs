use itertools::Itertools;

static ROLLWAYS: &[(i64, i64)] = &[(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn part1(start1: u64, start2: u64) -> u64 {
    let rolls: Vec<u64> = (1..1000)
        .into_iter()
        .tuples()
        .map(|(a, b, c)| a + b + c)
        .collect();

    let (mut posx, mut posy) = (start1, start2);
    let (mut scorex, mut scorey) = (0, 0);
    for (i, roll) in rolls.iter().enumerate() {
        match i {
            i if i % 2 == 0 => {
                posx = (posx + roll - 1) % 10 + 1;
                scorex += posx;
            }
            _ => {
                posy = (posy + roll - 1) % 10 + 1;
                scorey += posy;
            }
        }
        if scorey >= 1000 {
            return scorex * 3 * (i + 1) as u64;
        }
        if scorex >= 1000 {
            return scorey * 3 * (i + 1) as u64;
        }
    }
    0
}

fn recurse(pos: i64, score: i64, target: i64, nrolls: usize) -> i64 {
    match score {
        s if s > target => return 0,
        s if (s == target) & (nrolls == 0) => return 1,
        s if (s == target) & (nrolls != 0) => return 0,
        s if s >= 21 => return 0,
        _ => (),
    };

    let mut waysofwinningfromhere = 0;
    for (roll, ways) in ROLLWAYS {
        let newpos = (pos + *roll - 1) % 10 + 1;
        waysofwinningfromhere += ways * recurse(newpos, score + newpos, target, nrolls - 1);
    }
    return waysofwinningfromhere;
}

fn part2(start1: i64, start2: i64) -> u64 {
    let mut ans: u64 = 0;
    for nrolls in 1..11 {
        for target_win in 21..31 {
            let wayswin = recurse(start1, 0, target_win, nrolls);
            let wayslose: i64 = (1..21).map(|t| recurse(start2, 0, t, nrolls - 1)).sum();
            ans += (wayswin * wayslose) as u64;
        }
    }
    ans
}

fn main() {
    let now = std::time::Instant::now();

    let ans_part1 = part1(4, 5);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(4, 5);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
