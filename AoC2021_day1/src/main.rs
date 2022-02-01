use std::io::{BufRead,stdin};

fn main() {
    part_two();
}

fn part_one() {
    let stdin = stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());

    let mut current: i32 = 0x7FFFFF;
    let mut count: i32 = 0;

    for line in lines {
        let next: i32 = line.parse().unwrap();

        if (current) < (next) {
            count += 1;
        }
        current = next;
    }

    println!("Count: {}",count);
}

fn part_two() {
    let stdin = stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());

    let mut a: i32 = 0x7FFFFF;//assigned to
    let mut b: i32 = 0x7FFFFF;//carry from a
    let mut c: i32 = 0x7FFFFF;//carry from b, discarded
    let mut count: i32 = 0;

    for line in lines {
        let next: i32 = line.parse().unwrap();

        if (a+b+c) < (next+a+b) {
            count += 1;
        }
        c = b;
        b = a;
        a = next;
    }

    println!("Count: {}",count);
}
