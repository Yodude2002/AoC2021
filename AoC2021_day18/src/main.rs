use std::borrow::BorrowMut;
use std::fmt::{Debug, Formatter, Pointer};
use std::io::BufRead;
use std::mem::transmute;
use std::str::Chars;
use std::option::Option::None;
use crate::SnailFish::{NUMBER, PAIR};

#[derive(Debug, Clone)]
enum SnailFish {
    NUMBER(u64),
    PAIR(Box<SnailFish>,Box<SnailFish>)
}

/*
impl Debug for SnailFish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PAIR(a,b) => {
                f.debug_tuple("")
                    .field(a)
                    .field(b)
                    .finish()
            }
            NUMBER(n) => {
                let num = *n;
                f.fmt(num)
            }
        }
    }
}*/

impl SnailFish {
    pub fn from_iter(chars: &mut Chars) -> Self {
        let leading = chars.next().unwrap();
        if leading == '[' {
            let first = Self::from_iter(chars);
            if chars.next().unwrap() != ',' {
                panic!("Bad char in Snailfish num");
            }
            let last = Self::from_iter(chars);
            if chars.next().unwrap() != ']' {
                panic!("Bad char in Snailfish num");
            }
            PAIR(Box::from(first), Box::from(last))
        } else {
            NUMBER(leading as u64 - 0x30)
        }
    }

    pub fn magnitude(&self) -> u64 {
        match self {
            SnailFish::NUMBER(n) => *n,
            SnailFish::PAIR(left, right) => {
                3*left.magnitude() + 2*right.magnitude()
            }
        }
    }

    pub fn reduce(&mut self) {
        if let NUMBER(_) = self {
            return;
        } else if let PAIR(..) = self {
            loop {
                //explode
                if let Some(_) = self.reduce_explode_int(4) {
                    continue;
                }

                //split
                if let Some(_) = self.reduce_split_int() {
                    continue;
                }

                break;
            }
        }
    }

    fn reduce_explode_int(&mut self, depth: u8) -> Option<(i8,i8)> {
        if let NUMBER(_) = self {
            return None;
        } else if let PAIR(a,b) = self {
            if depth == 0 {
                if let NUMBER(n_a) = a.borrow_mut() {
                    if let NUMBER(n_b) = b.borrow_mut() {
                        return Some((*n_a as i8, *n_b as i8));
                    } else {
                        panic!("Not a Number!");
                    }
                } else {
                    panic!("Not a Number!");
                }
            } else {
                if let Some((left, right)) = a.reduce_explode_int(depth-1) {
                    if left == -1 && right == -1 {
                        return Some((-1, -1));
                    }
                    let mut num = b.borrow_mut();
                    while let PAIR(a,b) = num {
                        num = a.borrow_mut();
                    }
                    if let NUMBER(n) = num {
                        if right != -1 {
                            *n += right as u64;
                        }
                    } else {
                        dbg!(self);
                        panic!("I'm an idiot");
                    }
                    if left > -1 && right > -1 {
                        *a = Box::from(NUMBER(0));
                    }
                    return Some((left, -1));
                }
                if let Some((left, right)) = b.reduce_explode_int(depth-1) {
                    if left == -1 && right == -1 {
                        return Some((-1, -1));
                    }
                    let mut num = a.borrow_mut();
                    while let PAIR(a,b) = num {
                        num = b;
                    }
                    if let NUMBER(n) = num {
                        if left != -1 {
                            *n += left as u64;
                        }
                    } else {
                        panic!("I'm an idiot");
                    }
                    if left > -1 && right > -1 {
                        *b = Box::from(NUMBER(0));
                    }
                    return Some((-1, right));
                }
            }
        }

        None
    }
    fn reduce_split_int(&mut self) -> Option<bool> {
        if let NUMBER(n) = self {
            return if *n > 9 {
                Some(true)
            } else {
                None
            }
        } else if let PAIR(a,b) = self {

            if let Some(bl) = a.reduce_split_int() {
                if bl {
                    if let NUMBER(n) = a.borrow_mut() {
                        let left = *n/2;
                        let right = if *n%2 == 0 {
                            *n/2
                        } else {
                            *n/2 + 1
                        };
                        *a = Box::from(PAIR(Box::from(NUMBER(left)), Box::from(NUMBER(right))));
                    } else {
                        panic!("Not a number!");
                    }
                }
                return Some(false);
            }
            if let Some(bl) = b.reduce_split_int() {
                if bl {
                    if let NUMBER(n) = b.borrow_mut() {
                        let left = *n/2;
                        let right = if *n%2 == 0 {
                            *n/2
                        } else {
                            *n/2 + 1
                        };
                        *b = Box::from(PAIR(Box::from(NUMBER(left)), Box::from(NUMBER(right))));
                    } else {
                        panic!("Not a number!");
                    }
                }
                return Some(false);
            }
        }
        None
    }

    pub fn flatten(&self) -> Vec<u64> {
        let mut ret = Vec::new();
        if let PAIR(a, b) = self {
            for n in a.flatten() {
                ret.push(n);
            }
            for n in b.flatten() {
                ret.push(n);
            }
        } else if let NUMBER(n) = self {
            ret.push(*n);
        }

        ret
    }
}

fn main() {
    part_two();
}

fn part_one() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut pair = SnailFish::from_iter(&mut lines.next().unwrap().chars());

    for line in lines {
        pair = PAIR(Box::from(pair), Box::from(SnailFish::from_iter(&mut line.chars())));

        pair.reduce();
    }

    let snailfish = pair.magnitude();

    println!("Snailfish: {}",snailfish);
}

fn part_two() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut nums: Vec<SnailFish> = Vec::new();

    for line in lines {
        nums.push(SnailFish::from_iter(&mut line.chars()));
    }

    let max = nums.iter().map(|a| {
        nums.iter().map(|b|{
            let mut sum = PAIR(Box::from(a.clone()),Box::from(b.clone()));
            sum.reduce();
            sum.magnitude() as u64
        }).max().unwrap()
    }).max().unwrap();

    println!("Snailfish: {}", max);
}
