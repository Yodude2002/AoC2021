use std::io::{BufRead, stdin};

const GRID_SIZE: usize = 100;

fn main() {
    part_two();
}

fn part_one() {
    let stdin = stdin();
    let lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut grid: [[u8;GRID_SIZE];GRID_SIZE] = [[0; GRID_SIZE];GRID_SIZE];

    for (i, s) in lines.enumerate() {
        for (j, c) in s.chars().enumerate() {
            grid[i][j] = (c as u8) - 0x30;
        }
    }

    let mut danger_paths: [u64;GRID_SIZE*GRID_SIZE] = [0xFFFF_FFFF_FFFF_FFFF;GRID_SIZE*GRID_SIZE];

    let mut paths: Vec<(usize, u64)> = Vec::new();
    paths.push((0, 0));

    let danger: u64 = loop {
        let (pos, d) = paths.pop().unwrap();

        if pos == GRID_SIZE*GRID_SIZE - 1 {
            break d;
        }

        if d < danger_paths[pos] {
            danger_paths[pos] = d;
            if pos/GRID_SIZE != 0 {
                if (d + grid[pos/GRID_SIZE - 1][pos%GRID_SIZE] as u64) < danger_paths[pos - GRID_SIZE] {
                    paths.push((pos-GRID_SIZE,d + grid[pos/GRID_SIZE - 1][pos%GRID_SIZE] as u64));
                }
            }
            if pos%GRID_SIZE != 0 {
                if (d + grid[pos/GRID_SIZE][pos%GRID_SIZE - 1] as u64) < danger_paths[pos - 1] {
                    paths.push((pos - 1,d + grid[pos/GRID_SIZE][pos%GRID_SIZE - 1] as u64));
                }
            }
            if pos/GRID_SIZE + 1 < GRID_SIZE {
                if (d + grid[pos/GRID_SIZE + 1][pos%GRID_SIZE] as u64) < danger_paths[pos + GRID_SIZE] {
                    paths.push((pos+GRID_SIZE,d + grid[pos/GRID_SIZE + 1][pos%GRID_SIZE] as u64));
                }
            }
            if pos%GRID_SIZE + 1 < GRID_SIZE {
                if (d + grid[pos/GRID_SIZE][pos%GRID_SIZE + 1] as u64) < danger_paths[pos + 1] {
                    paths.push((pos + 1,d + grid[pos/GRID_SIZE][pos%GRID_SIZE + 1] as u64));
                }
            }
        }

        paths.sort_by(|a, b| b.1.cmp(&a.1));
    };

    println!("Danger: {}", danger);
}

fn part_two() {
    let stdin = stdin();
    let lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut grid: [[u8;GRID_SIZE*5];GRID_SIZE*5] = [[0; GRID_SIZE*5];GRID_SIZE*5];

    for (i, s) in lines.enumerate() {
        for (j, c) in s.chars().enumerate() {
            for row in 0..5 {
                for col in 0..5 {
                    grid[i + GRID_SIZE*row][j + GRID_SIZE*col] = ((c as u8) - 0x30 + row as u8 + col as u8);
                    if grid[i + GRID_SIZE*row][j + GRID_SIZE*col] > 9 {
                        grid[i + GRID_SIZE*row][j + GRID_SIZE*col] -= 9;
                    }
                }
            }
        }
    }

    let mut danger_paths: [u64;GRID_SIZE*GRID_SIZE*25] = [0xFFFF_FFFF_FFFF_FFFF;GRID_SIZE*GRID_SIZE*25];

    let mut paths: Vec<(usize, u64)> = Vec::new();
    paths.push((0, 0));

    let danger: u64 = loop {
        let (pos, d) = paths.pop().unwrap();

        if pos == GRID_SIZE*GRID_SIZE*25 - 1 {
            break d;
        }

        if d < danger_paths[pos] {
            danger_paths[pos] = d;
            if pos/(GRID_SIZE*5) != 0 {
                if (d + grid[pos/(GRID_SIZE*5) - 1][pos%(GRID_SIZE*5)] as u64) < danger_paths[pos - (GRID_SIZE*5)] {
                    paths.push((pos-(GRID_SIZE*5),d + grid[pos/(GRID_SIZE*5) - 1][pos%(GRID_SIZE*5)] as u64));
                }
            }
            if pos%(GRID_SIZE*5) != 0 {
                if (d + grid[pos/(GRID_SIZE*5)][pos%(GRID_SIZE*5) - 1] as u64) < danger_paths[pos - 1] {
                    paths.push((pos - 1,d + grid[pos/(GRID_SIZE*5)][pos%(GRID_SIZE*5) - 1] as u64));
                }
            }
            if pos/(GRID_SIZE*5) + 1 < (GRID_SIZE*5) {
                if (d + grid[pos/(GRID_SIZE*5) + 1][pos%(GRID_SIZE*5)] as u64) < danger_paths[pos + (GRID_SIZE*5)] {
                    paths.push((pos+(GRID_SIZE*5),d + grid[pos/(GRID_SIZE*5) + 1][pos%(GRID_SIZE*5)] as u64));
                }
            }
            if pos%(GRID_SIZE*5) + 1 < (GRID_SIZE*5) {
                if (d + grid[pos/(GRID_SIZE*5)][pos%(GRID_SIZE*5) + 1] as u64) < danger_paths[pos + 1] {
                    paths.push((pos + 1,d + grid[pos/(GRID_SIZE*5)][pos%(GRID_SIZE*5) + 1] as u64));
                }
            }
        }

        paths.sort_by(|a, b| b.1.cmp(&a.1));
    };

    println!("Danger: {}", danger);
}
