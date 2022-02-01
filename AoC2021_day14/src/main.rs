use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    part_two();
}

fn part_one() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut polymer = lines.next().unwrap();
    lines.next();

    let mut inserts: HashMap<String, char> = HashMap::new();

    for mut vec in lines.map(|line| { line.split(" -> ").map(|s|s.to_string()).collect::<Vec<String>>() }) {
        inserts.insert(vec.remove(0), vec.get(0).unwrap().chars().next().unwrap());
    }

    for _ in 0..10 {

        let mut new_polymer: String = String::new();
        let mut last: char = '\0';

        for c in polymer.chars() {
            if last != '\0' {
                new_polymer.push(*inserts.get(String::from_iter(vec![last, c]).as_str()).unwrap());
            }
            last = c;
            new_polymer.push(c);
        }

        polymer = new_polymer;
    }

    let mut dedup = polymer.chars().collect::<Vec<char>>();
    dedup.dedup();

    let ma = dedup.iter().map(|&c|{ polymer.chars().filter(|&ch|ch==c).count() }).max().unwrap();
    let mi = dedup.iter().map(|&c|{ polymer.chars().filter(|&ch|ch==c).count() }).min().unwrap();

    println!("Max-Min = {}",ma-mi);
}

fn part_two() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut polymer = lines.next().unwrap();
    lines.next();

    //let mut inserts: HashMap<String, char> = HashMap::new();

    let mut update: HashMap<String,(String,String)> = HashMap::new();

    for mut vec in lines.map(|line| { line.split(" -> ").map(|s|s.to_string()).collect::<Vec<String>>() }) {
        let key = vec.remove(0);
        let val = vec.get(0).unwrap().chars().next().unwrap();

        let mut it = key.chars();
        let first = it.next().unwrap();
        let last = it.next().unwrap();

        update.insert(key, (String::from_iter(vec![first, val]),String::from_iter(vec![val,last])));
    }

    let mut pairs: HashMap<String, u64> = HashMap::new();
    let mut last_char = '\0';
    for c in polymer.chars() {
        if last_char != '\0' {
            if let Some(mut n) = pairs.get_mut(String::from_iter(vec![last_char,c]).as_str()) {
                *n += 1;
            } else {
                pairs.insert(String::from_iter(vec![last_char,c]), 1);
            }
        }
        last_char = c;
    }

    for _ in 0..40 {
        let mut new_pairs: HashMap<String, u64> = HashMap::new();
        for (s,&num) in &pairs {
            let (a,b) = update.get(s.as_str()).unwrap();
            if let Some(mut n) = new_pairs.get_mut(a) {
                *n += num;
            } else {
                new_pairs.insert(a.clone(), num);
            }
            if let Some(mut n) = new_pairs.get_mut(b) {
                *n += num;
            } else {
                new_pairs.insert(b.clone(), num);
            }
        }
        pairs = new_pairs;
    }

    let mut by_char:Vec<(char,u64)> = pairs.iter().map(|(s,&n)|{
        let mut chars = s.chars();
        chars.next();
        (chars.next().unwrap(),n)
    }).collect::<Vec<(char,u64)>>();

    let char = polymer.chars().next().unwrap();

    let mut map_by_char: HashMap<char, u64> = HashMap::new();

    for (c,num) in &mut by_char {
        if let Some(mut n) = map_by_char.get_mut(c) {
            *n += *num;
        } else {
            map_by_char.insert(*c, *num);
        }
    }
    *map_by_char.get_mut(&char).unwrap() += 1;

    let ma = map_by_char.iter().map(|(s,n)|*n).max().unwrap();
    let mi = map_by_char.iter().map(|(s,n)|*n).min().unwrap();

    println!("Max-Min = {}",ma-mi);
}
