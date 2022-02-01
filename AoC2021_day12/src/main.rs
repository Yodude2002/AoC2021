use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    part_two();
}

fn part_one() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut connection_map: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let split: Vec<String> = line.split('-').map(|l|l.to_string()).collect();

        if let Some(vec) = connection_map.get_mut(split.get(0).unwrap()) {
            vec.push(split.get(1).unwrap().parse().unwrap());
        } else {
            let mut vec = Vec::new();
            vec.push(split.get(1).unwrap().parse().unwrap());
            connection_map.insert((*split.get(0).unwrap()).parse().unwrap(), vec);
        }
        if let Some(vec) = connection_map.get_mut(split.get(1).unwrap()) {
            vec.push(split.get(0).unwrap().parse().unwrap());
        } else {
            let mut vec = Vec::new();
            vec.push(split.get(0).unwrap().parse().unwrap());
            connection_map.insert((*split.get(1).unwrap()).parse().unwrap(), vec);
        }
    }

    let paths = p1_find_paths(&String::from("start"),&connection_map);

    println!("Paths: {}",paths);
}

fn p1_find_paths(path: &String, conn_map: &HashMap<String, Vec<String>>) -> i32 {
    let cave = path.rsplit('-').next().unwrap();

    if cave == "end" {
        return 1;
    }

    conn_map.get(cave).unwrap().iter().map(|s|{
        if s.chars().next().unwrap() > '`' {
            if let Some(_) = path.find(s) {
                return 0;
            }
        }
        let mut par = String::new();
        par.push_str(path);
        par.push('-');
        par.push_str(s);
        p1_find_paths(&par, conn_map)
    }).sum()
}

fn part_two() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut connection_map: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let split: Vec<String> = line.split('-').map(|l|l.to_string()).collect();

        if let Some(vec) = connection_map.get_mut(split.get(0).unwrap()) {
            vec.push(split.get(1).unwrap().parse().unwrap());
        } else {
            let mut vec = Vec::new();
            vec.push(split.get(1).unwrap().parse().unwrap());
            connection_map.insert((*split.get(0).unwrap()).parse().unwrap(), vec);
        }
        if let Some(vec) = connection_map.get_mut(split.get(1).unwrap()) {
            vec.push(split.get(0).unwrap().parse().unwrap());
        } else {
            let mut vec = Vec::new();
            vec.push(split.get(0).unwrap().parse().unwrap());
            connection_map.insert((*split.get(1).unwrap()).parse().unwrap(), vec);
        }
    }

    let paths = p2_find_paths(&String::from("start"),&connection_map);

    println!("Paths: {}",paths);
}

fn p2_find_paths(path: &String, conn_map: &HashMap<String, Vec<String>>) -> i32 {
    let cave = path.rsplit('-').next().unwrap();

    if cave == "end" {
        return 1;
    }

    conn_map.get(cave).unwrap().iter().map(|s|{
        if s == "start" {
            return 0;
        }
        if s.chars().next().unwrap() > '`' {
            if let Some(_) = path.find(s) {
                let mut sections: Vec<String> = path.split('-').filter(|&c|{
                    c.chars().next().unwrap() > '`'
                }).map(|f|f.to_string()).collect();
                sections.sort();
                sections.dedup();
                if path.split('-').filter(|&c|{
                    c.chars().next().unwrap() > '`'
                }).count() != sections.len() {
                    return 0;
                }
            }
        }
        let mut par = String::new();
        par.push_str(path);
        par.push('-');
        par.push_str(s);
        p2_find_paths(&par, conn_map)
    }).sum()
}
