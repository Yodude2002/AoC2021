use std::io::BufRead;

fn main() {
    part_two();
}

fn part_one() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut depth: i32 = 0;
    let mut horiz: i32 = 0;

    for line in lines {
        let mut pair = line.split_whitespace();

        let command = pair.next().unwrap();

        if command == "forward" {
            horiz += pair.next().unwrap().parse::<i32>().unwrap();
        } else if command == "down" {
            depth += pair.next().unwrap().parse::<i32>().unwrap();
        } else if command == "up" {
            depth -= pair.next().unwrap().parse::<i32>().unwrap();
        }
    }


    println!("depth: {}, horiz: {}",depth, horiz);
}

fn part_two() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut aim: i32 = 0;

    let mut depth: i32 = 0;
    let mut horiz: i32 = 0;

    for line in lines {
        let mut pair = line.split_whitespace();

        let command = pair.next().unwrap();

        if command == "forward" {
            let dist: i32 = pair.next().unwrap().parse().unwrap();
            horiz += dist;
            depth += dist*aim;
        } else if command == "down" {
            aim += pair.next().unwrap().parse::<i32>().unwrap();
        } else if command == "up" {
            aim -= pair.next().unwrap().parse::<i32>().unwrap();
        }
    }


    println!("depth: {}, horiz: {}, mult.: {}",depth, horiz, depth*horiz);
}
