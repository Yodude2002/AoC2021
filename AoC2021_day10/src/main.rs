use std::io::BufRead;

fn main() {
    part_two();
}

fn part_one() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l|l.unwrap());

    let sum: i32 = lines.map(|line| {
        let mut stack: Vec<char> = Vec::new();

        for c in line.chars() {
            match c {
                '('| '['| '{'| '<' => {
                    stack.push(c);
                }
                ')'| ']'| '}'| '>' => {
                    let ch = match stack.pop().unwrap() {
                        '(' => ')',
                        '[' => ']',
                        '{' => '}',
                        '<' => '>',
                        _ => '\0'
                    };
                    if ch != c {
                        return match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => 0
                        }
                    }
                }
                _ => {}
            }
        }

        0
    }).sum();

    println!("Sum: {}",sum);
}

fn part_two() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut sums: Vec<u64> = lines.map(|line| {
        let mut stack: Vec<char> = Vec::new();

        for c in line.chars() {
            match c {
                '('| '['| '{'| '<' => {
                    stack.push(c);
                }
                ')'| ']'| '}'| '>' => {
                    let ch = match stack.pop().unwrap() {
                        '(' => ')',
                        '[' => ']',
                        '{' => '}',
                        '<' => '>',
                        _ => '\0'
                    };
                    if ch != c {
                        return 0;
                    }
                }
                _ => {}
            }
        }

        let mut sum = 0;

        while !stack.is_empty() {
            sum *= 5;
            sum += match stack.pop().unwrap() {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0
            }
        }

        sum
    }).filter(|&n|n != 0).collect();;

    sums.sort();

    let sum = sums.get(sums.len()/2).unwrap();

    println!("Sum: {}",sum);
}
