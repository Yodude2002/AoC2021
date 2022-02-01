use std::collections::HashSet;
use std::io::BufRead;
use std::ops::IndexMut;

fn main() {
    part_two();
}

fn part_one() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut dots_s: Vec<String> = Vec::new();
    let mut inst: Vec<String> = Vec::new();

    for line in lines {
        if let Some(_) = line.find(',') {
            dots_s.push(line);
        } else if let Some(_) = line.find("fold") {
            inst.push(line);
        }
    }

    let mut dots: HashSet<Vec<i32>> = dots_s.iter().map(|l| {
        l.split(',').map(|s|s.parse::<i32>().unwrap()).collect()
    }).collect();

    let instruction: Vec<&str> = inst.get(0).unwrap().split_at(11).1.split("=").collect();

    let pos = instruction.get(1).unwrap().parse::<i32>().unwrap();
    if instruction.get(0).unwrap().chars().next().unwrap() == 'x' {
        dots = dots.iter().map(|d| {
            if *(d.get(0).unwrap()) > pos {
                vec![2*pos - *(d.get(0).unwrap()),*(d.get(1).unwrap())]
            } else {
                d.clone()
            }
        }).collect();
    } else {
        dots = dots.iter().map(|d| {
            if *(d.get(1).unwrap()) > pos {
                vec![*(d.get(0).unwrap()),2*pos - *(d.get(1).unwrap())]
            } else {
                d.clone()
            }
        }).collect();
    }

    println!("Dots: {}",dots.len());
}

fn part_two() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut dots_s: Vec<String> = Vec::new();
    let mut inst: Vec<String> = Vec::new();

    for line in lines {
        if let Some(_) = line.find(',') {
            dots_s.push(line);
        } else if let Some(_) = line.find("fold") {
            inst.push(line);
        }
    }

    let mut dots: HashSet<Vec<i32>> = dots_s.iter().map(|l| {
        l.split(',').map(|s|s.parse::<i32>().unwrap()).collect()
    }).collect();

    for instruct in inst {
        let instruction: Vec<&str> = instruct.split_at(11).1.split("=").collect();

        let pos = instruction.get(1).unwrap().parse::<i32>().unwrap();
        if instruction.get(0).unwrap().chars().next().unwrap() == 'x' {
            dots = dots.iter().map(|d| {
                if *(d.get(0).unwrap()) > pos {
                    vec![2*pos - *(d.get(0).unwrap()),*(d.get(1).unwrap())]
                } else {
                    d.clone()
                }
            }).collect();
        } else {
            dots = dots.iter().map(|d| {
                if *(d.get(1).unwrap()) > pos {
                    vec![*(d.get(0).unwrap()),2*pos - *(d.get(1).unwrap())]
                } else {
                    d.clone()
                }
            }).collect();
        }
    }

    let max_x = *(dots.iter().map(|v|v.get(0).unwrap()).max().unwrap())+1;
    let max_y = *(dots.iter().map(|v|v.get(1).unwrap()).max().unwrap())+1;

    let mut grid: Vec<Vec<bool>> = vec![vec![false; max_x as usize]; max_y as usize];

    for dot in &dots {
        let vec: &mut Vec<bool> = grid.get_mut((*(dot.get(1).unwrap())) as usize).unwrap();
        vec[(*(dot.get(0).unwrap())) as usize] = true;
    }

    for row in grid {
        for char in row {
            if char {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }


    println!("Dots: {}",dots.len());
}