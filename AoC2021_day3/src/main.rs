use std::borrow::Borrow;
use std::io::BufRead;
use std::iter::Map;

fn main() {
    part_two();
}

fn part_one() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut line_count = 0;
    let mut totals: [i32;12] = [0; 12];

    for line in lines {
        line_count += 1;

        for (i, char) in line.chars().enumerate() {
            if char == '1' {
                totals[i] += 1;
            }
        }
    }

    let mut gamma: i32 = 0;
    let mut epsilon: i32 = 0;

    for bit in totals {
        gamma <<= 1;
        epsilon <<= 1;
        if bit > line_count/2 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!("Gamma: {}, Epsilon: {}, Product: {}", gamma, epsilon, gamma*epsilon);
}

fn part_two() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut totals: [i32;12] = [0; 12];

    let mut current_lines: Vec<String> = lines.collect();


    for bit in  0..12 {
        totals = [0; 12];
        for line in &current_lines {
            for (i, char) in line.chars().enumerate() {
                if char == '1' {
                    totals[i] += 1;
                }
            }
        }
        let ones = totals[bit];
        let zeroes = current_lines.len() as i32 - ones;
        let key;
        if ones < zeroes { // For Oxygen, use >=. For CO2, use <
            key = 1;
        } else {
            key = 0;
        }

        current_lines.retain(|l|l.as_bytes()[bit] == (0x30+key) as u8);

        if current_lines.len() == 1 {
            break;
        }
    }

    let oxygen_str = current_lines.iter().next().unwrap();

    let mut oxygen: i32 = 0;
    for char in oxygen_str.chars() {
        oxygen <<= 1;
        oxygen += char as i32 - 0x30;
    }

    println!("Oxygen: {}", oxygen);
}
