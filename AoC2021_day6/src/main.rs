use std::io::BufRead;

fn main() {
    part_two();
}

fn part_one() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let mut fish: Vec<i32> = lines.next().unwrap().unwrap().split(",").map(|s|s.parse::<i32>().unwrap()).collect();

    for z in 0..80 {
        let new_fish = fish.iter().filter(|n|**n == 0).count();
        fish = fish.iter().map(|n| {
            if *n == 0 {
                6
            } else {
                *n-1
            }
        }).collect();
        for i in 0..new_fish {
            fish.push(8);
        }
    }

    println!("Fish: {}", fish.len());
}

fn part_two() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let mut fish: Vec<i32> = lines.next().unwrap().unwrap().split(",").map(|s|s.parse::<i32>().unwrap()).collect();

    let mut fish_counts: [u64; 9] = [0;9];

    for x in fish {
        fish_counts[x as usize] += 1;
    }

    for z in 0..256 {
        let new_fish = fish_counts[0];
        fish_counts[0] = fish_counts[1];
        fish_counts[1] = fish_counts[2];
        fish_counts[2] = fish_counts[3];
        fish_counts[3] = fish_counts[4];
        fish_counts[4] = fish_counts[5];
        fish_counts[5] = fish_counts[6];
        fish_counts[6] = fish_counts[7] + new_fish;
        fish_counts[7] = fish_counts[8];
        fish_counts[8] = new_fish;
    }

    let mut fish_sum = 0;
    for x in fish_counts {
        fish_sum += x;
    }

    println!("{} Fish",fish_sum);
}