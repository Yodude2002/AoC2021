use std::collections::HashMap;
use std::io::BufRead;
use crate::Inst::{ADD, ADDI, DIV, DIVI, EQL, EQLI, INP, MOD, MODI, MUL, MULI};

enum Inst {
    INP(u8),
    ADD(u8,u8),
    MUL(u8,u8),
    DIV(u8,u8),
    MOD(u8,u8),
    EQL(u8,u8),
    ADDI(u8,i64),
    MULI(u8,i64),
    DIVI(u8,i64),
    MODI(u8,i64),
    EQLI(u8,i64)
}

impl Inst {
    pub fn operate(&self, state: &mut [i64; 4]) -> Result<(),()> {
        match self {
            &Inst::INP(_r_a) => {
                assert!(false);
                Result::Ok(())
            },
            &Inst::ADD(r_a,r_b) => {
                state[r_a as usize] += state[r_b as usize];
                Result::Ok(())
            },
            &Inst::MUL(r_a,r_b) => {
                state[r_a as usize] *= state[r_b as usize];
                Result::Ok(())
            },
            &Inst::DIV(r_a,r_b) => {
                if state[r_b as usize] == 0 {
                    Result::Err(())
                } else {
                    state[r_a as usize] /= state[r_b as usize];
                    Result::Ok(())
                }
            },
            &Inst::MOD(r_a,r_b) => {
                if state[r_a as usize] < 0 || state[r_b as usize] <= 0 {
                    Result::Err(())
                } else {
                    state[r_a as usize] %= state[r_b as usize];
                    Result::Ok(())
                }
            },
            &Inst::EQL(r_a,r_b) => {
                state[r_a as usize] = if state[r_a as usize] == state[r_b as usize] {1} else {0};
                Result::Ok(())
            }
            &Inst::ADDI(r_a, i) => {
                state[r_a as usize] += i;
                Result::Ok(())
            }
            &Inst::MULI(r_a, i) => {
                state[r_a as usize] *= i;
                Result::Ok(())
            }
            &Inst::DIVI(r_a, i) => {
                if i == 0 {
                    Result::Err(())
                } else {
                    state[r_a as usize] /= i;
                    Result::Ok(())
                }
            }
            &Inst::MODI(r_a, i) => {
                if state[r_a as usize] < 0 || i <= 0 {
                    Result::Err(())
                } else {
                    state[r_a as usize] %= i;
                    Result::Ok(())
                }
            }
            &Inst::EQLI(r_a, i) => {
                state[r_a as usize] = if state[r_a as usize] == i {1} else {0};
                Result::Ok(())
            }
        }
    }

    pub fn from(str: &String) -> Self {
        let mut spl = str.split(" ");
        match spl.next().unwrap() {
            "inp" => {
                let a = spl.next().unwrap().chars().next().unwrap();
                INP(char_to_reg(a))
            }
            "add" => {
                let a = spl.next().unwrap().chars().next().unwrap();
                let sec = spl.next().unwrap();
                if let Ok(i) = sec.parse::<i64>() {
                    ADDI(char_to_reg(a),i)
                } else {
                    let b = sec.chars().next().unwrap();
                    ADD(char_to_reg(a),char_to_reg(b))
                }
            }
            "mul" => {
                let a = spl.next().unwrap().chars().next().unwrap();
                let sec = spl.next().unwrap();
                if let Ok(i) = sec.parse::<i64>() {
                    MULI(char_to_reg(a),i)
                } else {
                    let b = sec.chars().next().unwrap();
                    MUL(char_to_reg(a),char_to_reg(b))
                }

            }
            "div" => {
                let a = spl.next().unwrap().chars().next().unwrap();
                let sec = spl.next().unwrap();
                if let Ok(i) = sec.parse::<i64>() {
                    DIVI(char_to_reg(a),i)
                } else {
                    let b = sec.chars().next().unwrap();
                    DIV(char_to_reg(a),char_to_reg(b))
                }

            }
            "mod" => {
                let a = spl.next().unwrap().chars().next().unwrap();
                let sec = spl.next().unwrap();
                if let Ok(i) = sec.parse::<i64>() {
                    MODI(char_to_reg(a),i)
                } else {
                    let b = sec.chars().next().unwrap();
                    MOD(char_to_reg(a),char_to_reg(b))
                }
            }
            "eql" => {
                let a = spl.next().unwrap().chars().next().unwrap();
                let sec = spl.next().unwrap();
                if let Ok(i) = sec.parse::<i64>() {
                    EQLI(char_to_reg(a),i)
                } else {
                    let b = sec.chars().next().unwrap();
                    EQL(char_to_reg(a),char_to_reg(b))
                }
            }
            &_ => {
                assert!(false);
                INP(0)
            }
        }
    }
}

fn main() {
    part_one_validate();
}

fn part_one() {
    let values: Vec<(bool, i64,i64)> = vec![
        (false, 14, 12),
        (false, 10, 9),
        (false, 13, 8),
        (true, -8, 3),
        (false, 11, 0),
        (false, 11, 11),
        (false, 14, 10),
        (true, -11, 13),
        (false, 14, 3),
        (true, -1, 10),
        (true, -8, 10),
        (true, -5, 14),
        (true, -16, 6),
        (true, -6, 5)
    ];

    let mut state_vec: Vec<(u64, i64)> = vec![(0,0)];

}

fn part_one_validate() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut chains: [Vec<Inst>;14] = [
        Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),
        Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new()
    ];

    let mut index = -1;

    for l in lines {
        let inst = Inst::from(&l);
        if let INP(_) = inst {
            index += 1;
        }
        chains[index as usize].push(inst);
    }

    let mut state = [0_i64;4];


    let mut model = "18116121134117".chars().map(|c|{
        ((c as u8) - 0x30)
    });

    for (i,chain) in chains.into_iter().enumerate() {
        print!("");
        for inst in chain {
            if let INP(r) = inst {
                state[r as usize] = model.next().unwrap() as i64;
            } else {
                if let Err(_) = inst.operate(&mut state) {
                    assert!(false);
                }
            }
        }
        println!("After {} steps, z has a value of {}", i+1, state[2]);
    }

    println!("");
}

fn part_one_bad() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut chains: [Vec<Inst>;14] = [
        Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),
        Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new()
    ];

    let mut index = -1;

    for l in lines {
        let inst = Inst::from(&l);
        if let INP(_) = inst {
            index += 1;
        }
        chains[index as usize].push(inst);
    }

    //optimize: dedup values
    //let mut state_map: HashMap<u64, [i64;4]> = HashMap::new();
    let mut state_vec: Vec<(u64,[i64;4])> = Vec::new();
    //state_map.insert(0,[0;4]);
    state_vec.push((0,[0;4]));

    for (ch_num,chain) in chains.iter().enumerate() {
        //let mut new_state_map: HashMap<u64, [i64;4]> = HashMap::new();
        let mut new_state_vec: Vec<(u64,[i64;4])> = Vec::new();
        let mut memo: Vec<[i64;4]> = Vec::new();
        'val:
        for (val, state) in &state_vec {
            'i:
            for i in (1..=9).rev() {
                let mut new_state = state.clone();
                'inst:
                for inst in chain {
                    if let &INP(r) = inst {
                        new_state[r as usize] = i;
                    } else {
                        if let Err(_) = inst.operate(&mut new_state) {
                            print!("");
                            continue 'i;
                        }
                    }
                }

                if !(memo.contains(&new_state)) {
                    memo.push(new_state.clone());
                    new_state_vec.push((*val*10 + i as u64, new_state));
                }
            }
        }
        state_vec = new_state_vec;
        state_vec.sort_by(|a,b|{
            b.0.cmp(&a.0)
        });
        println!("Evaluated chain {}",ch_num);
    }

    let m = state_vec.iter().filter_map(|(v, s)| {
        if s[2] == 0 {
            Some(*v)
        } else {
            None
        }
    }).max().unwrap();

    println!("Max: {}",m);
}

fn char_to_reg(c: char) -> u8 {
    match c {
        'x' => 0,
        'y' => 1,
        'z' => 2,
        'w' => 3,
        _ => {
            assert!(false);
            0
        }
    }
}
