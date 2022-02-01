use std::io::BufRead;

//const ROWS: usize = 9;
//const COLS: usize = 10;

const ROWS: usize = 137;
const COLS: usize = 139;


fn main() {
    part_one();
}

fn part_one() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut grid: [[char;COLS];ROWS] = [['\0';COLS];ROWS];

    for (r, line) in lines.enumerate() {
        for (c, char) in line.chars().enumerate() {
            grid[r][c] = char;
        }
    }

    let mut last: i64 = -1;

    for i in 1.. {
        let mut new_grid: [[char;COLS];ROWS] = [['.';COLS];ROWS];

        let mut dirty: bool = false;

        for (r, row) in grid.iter().enumerate() {
            for (c,&cell) in row.iter().enumerate() {
                if cell == '>' {
                    if grid[r][(c+1)%COLS] == '.' {
                        new_grid[r][(c+1)%COLS] = '>';
                        dirty = true;
                    } else {
                        new_grid[r][c] = '>';
                    }
                }
            }
        }

        for (r, row) in grid.iter_mut().enumerate() {
            for (c, cell) in row.iter_mut().enumerate() {
                if *cell == '>' {
                    *cell = '.'
                }
                if new_grid[r][c] == '>' {
                    *cell = '>'
                }
            }
        }
        //grid = new_grid;
        new_grid = [['.';COLS];ROWS];

        for (r, row) in grid.iter().enumerate() {
            for (c,&cell) in row.iter().enumerate() {
                if cell == 'v' {
                    if grid[(r+1)%ROWS][c] == '.' {
                        new_grid[(r+1)%ROWS][c] = 'v';
                        dirty = true;
                    } else {
                        new_grid[r][c] = 'v';
                    }
                }
            }
        }

        for (r, row) in grid.iter_mut().enumerate() {
            for (c, cell) in row.iter_mut().enumerate() {
                if *cell == 'v' {
                    *cell = '.'
                }
                if new_grid[r][c] == 'v' {
                    *cell = 'v'
                }
            }
        }

        if !dirty {
            last = i;
            break;
        }

        println!("Processed {} iterations",i);
        println!("Total of {} v, {} >",grid.iter().map(|r|r.iter().filter(|&&c|c=='v').count()).sum::<usize>(),grid.iter().map(|r|r.iter().filter(|&&c|c=='>').count()).sum::<usize>());
    };

    println!("Last: {}",last);
}
