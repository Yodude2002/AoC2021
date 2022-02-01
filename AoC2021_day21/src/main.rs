use std::io::BufRead;

trait Die {
    fn die_roll(&mut self) -> u64;
    fn total_rolls(&self) -> u64;
}

struct DeterministicDie {
    total_rolls:u64,
    current_roll:u64
}

impl DeterministicDie {
    pub fn new() -> Self {
        Self {total_rolls: 0, current_roll: 0 }
    }
}

impl Die for DeterministicDie {
    fn die_roll(&mut self) -> u64 {
        self.total_rolls += 1;
        self.current_roll = self.current_roll%1000 + 1;
        self.current_roll
    }

    fn total_rolls(&self) -> u64 {
        self.total_rolls
    }
}


fn main() {
    part_two();
}

fn part_one() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut player_positions = lines.map(|s|s.chars().rev().next().unwrap() as u64 - 0x30);
    let mut player_pos = (player_positions.next().unwrap(), player_positions.next().unwrap());
    let mut player_scores = (0,0);

    let mut die = DeterministicDie::new();

    let score = loop {
        let roll1: u64 = (0..3).map(|_|die.die_roll()).sum();
        player_pos.0 += roll1 ;
        player_pos.0 %= 10;
        if player_pos.0 == 0 {
            player_pos.0 = 10;
        }
        player_scores.0 += player_pos.0;
        if player_scores.0 >= 1000 {
            break player_scores.1;
        }


        let roll2: u64 = (0..3).map(|_|die.die_roll()).sum();
        player_pos.1 += roll2;
        player_pos.1 %= 10;
        if player_pos.1 == 0 {
            player_pos.1 = 10;
        }
        player_scores.1 += player_pos.1;
        if player_scores.1 >= 1000 {
            break player_scores.0;
        }
    };

    println!("Output: {}",die.total_rolls() * score);
}

fn part_two() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut player_positions = lines.map(|s|s.chars().rev().next().unwrap() as u64 - 0x30);
    let mut player_pos = (player_positions.next().unwrap(), player_positions.next().unwrap());
    let mut player_scores = (0,0);

    let game_outcome = p2_calc_game(player_pos, player_scores);

    let winner = game_outcome.0.max(game_outcome.1);

    println!("Output: {}",winner);
}

///
/// Calculates a game.
///
/// pos.0 is the position of the current player's piece, and score.0 is their score.
///
/// returns a pair containing (number of universes current player wins in, number they lose in).
fn p2_calc_game(pos: (u64,u64), score: (u64,u64)) -> (u64,u64) {
    let roll_vec: &'static [(u64, u64); 7] = &[(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    let mut ret = (0,0);

    for (w, l) in roll_vec.iter().map(|&(roll, times)| {
        let mut position = (pos.0 + roll)%10;
        if position == 0 {
            position = 10;
        }
        let sc = score.0 + position;
        if sc >= 21 {
            (times,0)
        } else {
            let g = p2_calc_game((pos.1,position),(score.1, sc));
            (times*g.0, times*g.1)
        }
    }) {
        ret.0 += l;
        ret.1 += w;
    }

    ret
}
