use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    part_two();
}

fn part_one() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l|l.unwrap());

    let first: Vec<u8> = lines.next().unwrap().chars().map(|c| if c == '#' {1} else {0}).collect();
    lines.next();

    let mut lower: (i64, i64) = (0,0);
    let mut upper: (i64, i64) = (0,0);

    let mut grid: HashMap<(i64,i64),u8> = HashMap::new();

    let mut background: u8 = 0;

    for (x,line) in lines.enumerate() {
        if x as i64 > upper.0 {
            upper.0 = x as i64;
        }
        for (y,char) in line.chars().enumerate() {
            if y as i64 > upper.1 {
                upper.1 = y as i64;
            }
            grid.insert((x as i64, y as i64),if char == '#' {1} else {0});
        }
    }
    for _ in 0..2{
        let mut new_grid: HashMap<(i64, i64), u8> = HashMap::new();
        for x in (lower.0 - 1)..=(upper.0 + 1) {
            for y in (lower.1 - 1)..=(upper.1 + 1) {
                let mut index: u16 = 0;
                for x_i in (x - 1)..=(x + 1) {
                    for y_i in (y - 1)..=(y + 1) {
                        if x_i < lower.0 || x_i > upper.0 || y_i < lower.1 || y_i > upper.1 {
                            index = index * 2 + background as u16;
                        } else {
                            index = index * 2 + (*grid.get(&(x_i, y_i)).unwrap()) as u16
                        }
                    }
                }
                new_grid.insert((x, y), *first.get(index as usize).unwrap());
            }
        }
        lower = (lower.0-1, lower.1-1);
        upper = (upper.0+1, upper.1+1);
        background = if background == 0 {
            *first.get(0).unwrap()
        } else {
            *first.get(511).unwrap()
        };
        grid = new_grid;
    }

    let on = grid.iter().filter(|t|*t.1 == 1).count();

    println!("On: {}",on);
}

fn part_two() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l|l.unwrap());

    let first: Vec<u8> = lines.next().unwrap().chars().map(|c| if c == '#' {1} else {0}).collect();
    lines.next();

    let mut lower: (i64, i64) = (0,0);
    let mut upper: (i64, i64) = (0,0);

    let mut grid: HashMap<(i64,i64),u8> = HashMap::new();

    let mut background: u8 = 0;

    for (x,line) in lines.enumerate() {
        if x as i64 > upper.0 {
            upper.0 = x as i64;
        }
        for (y,char) in line.chars().enumerate() {
            if y as i64 > upper.1 {
                upper.1 = y as i64;
            }
            grid.insert((x as i64, y as i64),if char == '#' {1} else {0});
        }
    }
    for _ in 0..50 {
        let mut new_grid: HashMap<(i64, i64), u8> = HashMap::new();
        for x in (lower.0 - 1)..=(upper.0 + 1) {
            for y in (lower.1 - 1)..=(upper.1 + 1) {
                let mut index: u16 = 0;
                for x_i in (x - 1)..=(x + 1) {
                    for y_i in (y - 1)..=(y + 1) {
                        if x_i < lower.0 || x_i > upper.0 || y_i < lower.1 || y_i > upper.1 {
                            index = index * 2 + background as u16;
                        } else {
                            index = index * 2 + (*grid.get(&(x_i, y_i)).unwrap()) as u16
                        }
                    }
                }
                new_grid.insert((x, y), *first.get(index as usize).unwrap());
            }
        }
        lower = (lower.0-1, lower.1-1);
        upper = (upper.0+1, upper.1+1);
        background = if background == 0 {
            *first.get(0).unwrap()
        } else {
            *first.get(511).unwrap()
        };
        grid = new_grid;
    }

    let on = grid.iter().filter(|t|*t.1 == 1).count();

    println!("On: {}",on);
}
