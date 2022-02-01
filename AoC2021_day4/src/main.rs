use std::io::BufRead;

#[derive(Debug)]
struct Board {
    board: [i32; 25],
    guess: Vec<i32>,
}

impl Board {
    pub fn winner(&self) -> bool {
        for i in 0..5 {
            let mut count: i32 = 0;
            for j in 0..5 {
                if self.guess.contains(&self.board[i * 5 + j]) {
                    count += 1;
                }
            }
            if count == 5 {
                return true;
            }
            count = 0;
            for j in 0..5 {
                if self.guess.contains(&self.board[j * 5 + i]) {
                    count += 1;
                }
            }
            if count == 5 {
                return true;
            }
        }

        /*
        let mut count: i32 = 0;
        for i in 0..5 {
            if self.guess.contains(&self.board[i * 5 + i]) {
                count += 1;
            }
        }
        if count == 5 {
            return true;
        }

        count = 0;
        for i in 0..5 {
            if self.guess.contains(&self.board[i * 5 + (4-i)]) {
                count += 1;
            }
        }
        if count == 5 {
            return true;
        }*/

        false
    }
}

fn main() {
    part_two();
}

fn part_one() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let calls: Vec<String> = lines.next().unwrap().unwrap().split(",").map(|l| l.clone().to_string()).collect();

    let mut boards: Vec<Board> = Vec::new();// = [;0].to_vec();

    let mut board: Board = Board {board: [0; 25], guess: [0;0].to_vec()};
    for (i, line) in lines.map(|l|l.unwrap()).enumerate() {
        if line.len() == 0 {
            continue;
        }

        for (j, num) in line.split_whitespace()
                            .filter(|l| l.len() > 0)
                            .map(|l| l.parse::<i32>())
                            .enumerate() {
            board.board[((i%6)-1) * 5 + j] = num.unwrap();
        }

        if i % 6 == 5 {
            //add board, and reset
            boards.push(board);
            board = Board {board: [0; 25], guess: [0;0].to_vec()};
        }
    }

    for num in calls.iter().map(|l|l.parse::<i32>().unwrap()) {
        for board in &mut boards {
            board.guess.push(num);
            if board.winner() {
                let mut sum = 0;
                for n in board.board.iter().filter(|n| !board.guess.contains(n)) {
                    sum += n;
                }
                println!("Winning Board sum: {}, Winning number: {}, product: {}",sum, num, sum*num);
                return;
            }
        }

    }

    println!();
}

fn part_two() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let calls: Vec<String> = lines.next().unwrap().unwrap().split(",").map(|l| l.clone().to_string()).collect();

    let mut boards: Vec<Board> = Vec::new();// = [;0].to_vec();

    let mut board: Board = Board {board: [0; 25], guess: [0;0].to_vec()};
    for (i, line) in lines.map(|l|l.unwrap()).enumerate() {
        if line.len() == 0 {
            continue;
        }

        for (j, num) in line.split_whitespace()
            .filter(|l| l.len() > 0)
            .map(|l| l.parse::<i32>())
            .enumerate() {
            board.board[((i%6)-1) * 5 + j] = num.unwrap();
        }

        if i % 6 == 5 {
            //add board, and reset
            boards.push(board);
            board = Board {board: [0; 25], guess: [0;0].to_vec()};
        }
    }

    for num in calls.iter().map(|l|l.parse::<i32>().unwrap()) {
        let len = boards.len();
        for board in &mut boards {
            board.guess.push(num);
            if board.winner() && len == 1 {
                let mut sum = 0;
                for n in board.board.iter().filter(|n| !board.guess.contains(n)) {
                    sum += n;
                }
                println!("Winning Board sum: {}, Winning number: {}, product: {}",sum, num, sum*num);
                return;
            }
        }

        if boards.len() > 1 {
            boards.retain(|b| !b.winner());
        }
    }

    println!();
}