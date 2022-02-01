use std::collections::HashSet;
use std::io::BufRead;
use std::ops::Not;

const GRID_SIZE: usize = 100;

fn main() {
    part_two();
    println!("Hello, world!");
}

fn part_one() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut grid: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = ((c as u8) - 0x30);
        }
    }

    let mut low_points: Vec<i32> = Vec::new();

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            if ((i == 0 || grid[i-1][j] > grid[i][j]) &&
                (i+1 == GRID_SIZE || grid[i+1][j] > grid[i][j])) &&
               ((j == 0 || grid[i][j-1] > grid[i][j]) &&
                (j+1 == GRID_SIZE || grid[i][j+1] > grid[i][j])) {
                low_points.push((i * GRID_SIZE) as i32 + j as i32);
            }
        }
    }

    let sum: i32 = low_points.iter().map(|&p| {
        (grid[(p as usize)/GRID_SIZE][(p as usize)%GRID_SIZE] + 1) as i32
    }).sum();

    println!("Sum: {}",sum);
}

fn part_two() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut grid: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = ((c as u8) - 0x30);
        }
    }

    let mut low_points: Vec<i32> = Vec::new();

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            if ((i == 0 || grid[i-1][j] > grid[i][j]) &&
                (i+1 == GRID_SIZE || grid[i+1][j] > grid[i][j])) &&
                ((j == 0 || grid[i][j-1] > grid[i][j]) &&
                    (j+1 == GRID_SIZE || grid[i][j+1] > grid[i][j])) {
                low_points.push((i * GRID_SIZE) as i32 + j as i32);
            }
        }
    }

    let mut sizes: Vec<usize> = low_points.iter().map(|&p|{
        let mut vec = HashSet::new();
        basin_size(&grid, &mut vec, p);
        vec.len()
    }).collect();
    sizes.sort();
    let mut product = 1;
    for _ in 0..3 {
        let m: usize = *sizes.iter().max().unwrap();
        product *= m;
        sizes.retain(|&n|n != m);
    }

    println!("Product: {}",product);
}

fn basin_size(grid: &[[u8; GRID_SIZE]; GRID_SIZE], coords: &mut HashSet<i32>, coord:i32) {
    let i = (coord as usize)/GRID_SIZE;
    let j = (coord as usize)%GRID_SIZE;
    if coords.contains(&coord) || grid[i][j] >= 9{
        return;
    }
    coords.insert(coord);

    if i != 0 {
        basin_size(grid, coords, coord - GRID_SIZE as i32);
    }
    if j != 0 {
        basin_size(grid, coords, coord - 1);
    }
    if i != GRID_SIZE-1 {
        basin_size(grid, coords, coord + GRID_SIZE as i32);
    }
    if j != GRID_SIZE-1 {
        basin_size(grid, coords, coord + 1);
    }

}
