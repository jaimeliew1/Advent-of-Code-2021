use std::fs;

fn to_decimal(binary_str: &str) -> Option<u64> {
    let mut out_num: u64 = 0;
    for c in binary_str.trim().chars() {
        match c.to_digit(10).unwrap() {
            d if d <= 1 => out_num = out_num * 2 + d as u64,
            _ => return None,
        }
    }
    Some(out_num)
}

fn parse_input(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("can't find file");
    contents
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .collect::<Vec<_>>()
        .join("")
}

fn parse_packet(packet: &str) -> Option<(&str, u64, u64)> {
    if packet.len() == 0 {
        return None;
    }
    let mut version = to_decimal(&packet[0..3]).unwrap();
    let ptype = to_decimal(&packet[3..6]).unwrap();
    match ptype {
        4 => {
            // literal
            let mut substring = &packet[6..];
            let mut literal = Vec::new();
            while substring.chars().nth(0).unwrap() != '0' {
                literal.push(&substring[1..5]);
                substring = &substring[5..];
            }
            literal.push(&substring[1..5]);
            let literal = to_decimal(&literal.join("")).unwrap();

            return Some((&substring[5..], version, literal));
        }
        op => {
            // operation
            let mut packet_rem;
            let mut values = Vec::new();

            match to_decimal(&packet[6..7]).unwrap() {
                0 => {
                    // substring length
                    let length = to_decimal(&packet[7..22]).unwrap() as usize;
                    let mut subpacket = &packet[22..(length + 22)];
                    packet_rem = &packet[(length + 22)..];
                    while let Some((x, vsum, val)) = parse_packet(&subpacket) {
                        subpacket = x;
                        version += vsum;
                        values.push(val)
                    }
                }
                1 => {
                    // number of subpackets
                    let npackets = to_decimal(&packet[7..18]).unwrap() as usize;
                    packet_rem = &packet[18..];
                    for _ in 0..npackets {
                        let (x, vsum, val) = parse_packet(&packet_rem).unwrap();
                        packet_rem = x;
                        version += vsum;
                        values.push(val);
                    }
                }
                _ => unreachable!(),
            }
            // perform operation.
            let value: u64 = match op {
                0 => values.iter().sum(),
                1 => values.iter().product(),
                2 => *values.iter().min().unwrap(),
                3 => *values.iter().max().unwrap(),
                5 if values[0] > values[1] => 1,
                6 if values[0] < values[1] => 1,
                7 if values[0] == values[1] => 1,
                5 | 6 | 7 => 0,
                _ => unreachable!(),
            };
            return Some((packet_rem, version, value));
        }
    };
}

fn main() {
    let now = std::time::Instant::now();

    let data = parse_input("input/day16.txt");

    let (_, version_sum, final_value) = parse_packet(&data).unwrap();
    println!("part1: {}", version_sum);
    println!("part2: {}", final_value);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
