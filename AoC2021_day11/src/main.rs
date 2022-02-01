use std::cmp::{min,max};
use std::io::BufRead;

fn main() {
    part_two();
}

fn part_one() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut grid: [[u8; 10]; 10] = [[0; 10]; 10];

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = (c as u8) - 0x30;
        }
    }

    let mut flashes: i32 =  (0..100).map(|_|{
        let mut flashed: [[bool; 10]; 10] = [[false; 10]; 10];

        for i in 0..10 {
            for j in 0..10 {
                grid[i][j] += 1;
            }
        }

        for i in 0..100 {
            if grid[i/10][i%10] > 9 {
                step_phase_two(&mut grid, &mut flashed, i);
            }
        }

        for i in 0..10 {
            for j in 0..10 {
                if flashed[i][j] {
                    grid[i][j] = 0;
                }
            }
        }

        flashed.iter().map(|row|row.iter().filter(|&&b|b).count() as i32).sum::<i32>()
    }).sum::<i32>();

    println!("Flashes: {}",flashes);
}

fn part_two() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut grid: [[u8; 10]; 10] = [[0; 10]; 10];

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = (c as u8) - 0x30;
        }
    }

    let mut steps = 0;
    loop {
        steps += 1;
        let mut flashed: [[bool; 10]; 10] = [[false; 10]; 10];

        for i in 0..10 {
            for j in 0..10 {
                grid[i][j] += 1;
            }
        }

        for i in 0..100 {
            if grid[i/10][i%10] > 9 {
                step_phase_two(&mut grid, &mut flashed, i);
            }
        }

        for i in 0..10 {
            for j in 0..10 {
                if flashed[i][j] {
                    grid[i][j] = 0;
                }
            }
        }

        if flashed.iter().map(|row|row.iter().filter(|&&b|b).count() as i32).sum::<i32>() == 100 {
            break;
        }
    }

    println!("Step for first flash: {}",steps);
}

fn step_phase_two(grid: &mut [[u8; 10]; 10], flashed: &mut [[bool; 10]; 10], cell: usize) {
    if flashed[cell/10 as usize][cell%10 as usize] {
        return;
    }
    flashed[cell/10 as usize][cell%10 as usize] = true;

    for i in (max(1, (cell/10))-1)..=(min(9, (cell/10)+1)) {
        for j in (max(1, (cell%10))-1)..=(min(9, (cell%10)+1)) {
            grid[i][j] += 1;
            if grid[i][j] > 9 {
                step_phase_two(grid, flashed, i*10 + j);
            }
        }
    }



}