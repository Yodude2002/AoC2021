use std::io::BufRead;

fn main() {
    part_two();
}

fn part_one() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let crabs: Vec<i32> = lines.next().unwrap().unwrap().split(",").map(|s|s.parse::<i32>().unwrap()).collect();

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    let result: i32 = (min..=max).map(|i|{
        crabs.iter().map(|n|(*n-i).abs()).sum()
    }).min().unwrap();

    println!("Min Crabs: {}", result);
}

fn part_two() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let crabs: Vec<i32> = lines.next().unwrap().unwrap().split(",").map(|s|s.parse::<i32>().unwrap()).collect();

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    let result: i32 = (min..=max).map(|i|{
        crabs.iter().map(|n|(*n-i).abs()).map(|n|((n*(n+1))/2)).sum()
    }).min().unwrap();

    println!("Min Crabs: {}", result);
}

fn part_two_one_line() {//ok I got it in two but that's close enough.
    let crabs: Vec<i32> = std::io::stdin().lock().lines().next().unwrap().unwrap().split(",").map(|s|s.parse::<i32>().unwrap()).collect();
    let result: i32 = ((*crabs.iter().min().unwrap())..=(*crabs.iter().max().unwrap())).map(|i|{ crabs.iter().map(|n|(*n-i).abs()).map(|n|((n*(n+1))/2)).sum() }).min().unwrap();

    println!("Min Crabs: {}", result);
}