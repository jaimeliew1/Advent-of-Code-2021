use std::collections::HashSet;
use std::fs;

fn parse_input(filename: &str) -> Vec<Vec<u32>> {
    let contents = fs::read_to_string(filename).expect("can't find file");
    let data: Vec<Vec<u32>> = contents
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    data
}

fn dijkstra(grid: &Vec<Vec<u32>>, start: (usize, usize), end: (usize, usize)) -> u32 {
    let N = grid.len();
    let mut visited = vec![vec![false; N]; N];
    let mut pathsum = vec![vec![999999_u32; N]; N];
    pathsum[start.0][start.1] = 0;
    let mut frontier: HashSet<(usize, usize)> = HashSet::from([start]);
    let (mut x, mut y) = start;

    while !visited[end.0][end.1] {
        for (xnew, ynew) in [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)] {
            if grid.get(xnew).and_then(|row| row.get(ynew)).is_none() {
                continue;
            }
            if visited[xnew][ynew] {
                continue;
            }
            if (pathsum[x][y] + grid[xnew][ynew]) < pathsum[xnew][ynew] {
                pathsum[xnew][ynew] = pathsum[x][y] + grid[xnew][ynew];
                frontier.insert((xnew, ynew));
            }
        }
        visited[x][y] = true;
        frontier.remove(&(x, y));
        if let Some(newxy) = frontier.iter().min_by_key(|(xx, yy)| pathsum[*xx][*yy]) {
            x = newxy.0;
            y = newxy.1;
        }
    }
    pathsum[end.0][end.1].into()
}
fn part1(data: &Vec<Vec<u32>>) -> u32 {
    let end = (data.len() - 1, data.len() - 1);
    dijkstra(&data, (0, 0), end)
}

fn part2(data: &Vec<Vec<u32>>) -> u32 {
    let (N, M) = (data.len(), data[0].len());
    let mut expanded: Vec<Vec<u32>> = vec![vec![0; 5 * M]; 5 * N];
    for i in 0..expanded.len() {
        for j in 0..expanded[0].len() {
            expanded[i][j] = (data[i % N][j % M] + (i / N + j / M) as u32 - 1) % 9 + 1;
        }
    }
    let end = (expanded.len() - 1, expanded.len() - 1);
    dijkstra(&expanded, (0, 0), end)
}

fn main() {
    let now = std::time::Instant::now();

    let data = parse_input("input/day15.txt");

    let ans_part1 = part1(&data);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&data);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
