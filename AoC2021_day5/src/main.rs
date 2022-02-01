use std::cmp::{min,max};
use std::io::BufRead;

const COUNT: i32 = 1000;

fn main() {
    part_one();
}

fn part_one() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());

    let mut arr: [[i8; COUNT as usize]; COUNT as usize] = [[0; COUNT as usize]; COUNT as usize];

    for line in lines {
        let mut coords = [0;4];
        for (i, pair) in line.split(" -> ").enumerate() {
            for (j, n) in pair.split(",").map(|n|n.parse::<i32>()).enumerate() {
                coords[i*2 + j] = n.unwrap();
            }
        }
        if coords[0] == coords[2] {
            for i in min(coords[1],coords[3])..=max(coords[1],coords[3]) {
                arr[(coords[0]) as usize][i as usize] += 1;
            }
        } else if coords[1] == coords[3] {
            for i in min(coords[0],coords[2])..=max(coords[0],coords[2]) {
                arr[(i) as usize][coords[1] as usize] += 1;
            }
        } else {
            let mut dx = coords[2] - coords[0];
            let mut dy = coords[3] - coords[1];
            if dx.abs() != dy.abs() {
                println!("diffs are diff!");
            }
            dx /= dx.abs();
            dy /= dy.abs();

            for i in 0..=(coords[2]-coords[0]).abs() {
                arr[(coords[0] + i*dx) as usize][(coords[1] + i*dy) as usize] += 1;
            }
        }

    }

    let mut gt1 = 0;

    for i in 0..COUNT {
        for j in 0..COUNT {
            if arr[i as usize][j as usize] > 1 {
                gt1 += 1;
            }
        }
    }

    println!("Total >1: {}", gt1);
}