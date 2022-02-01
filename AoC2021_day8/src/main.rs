use std::io::BufRead;

fn main() {
    part_two();
}

fn part_one() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut total = 0;

    for line in lines {
        let mut split = line.split(" | ");
        split.next();
        total += split.next().unwrap().split_whitespace().map(|w|w.len()).filter(|n|{
            *n==2 || *n==3 || *n==4 || *n==7
        }).count();
    }

    println!("Total: {}", total);
}

fn part_two() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|l|l.unwrap());

    let total: usize = lines.map(|line| {
        let mut split = line.split(" | ");

        let mut dec_arr: [u8;7]   = [0b01111111u8;7];//indices: start at top, go down and across. LSB is top, MSB is bottom.
        let mut charray: [char;7] = ['\0';7];//LSB is top, MSB is bottom
        let test = split.next().unwrap();

        let len2 = test.split_whitespace().filter(|s|s.len()==2).next().unwrap().to_string();
        let len4 = test.split_whitespace().filter(|s|s.len()==4).next().unwrap().to_string();

        while charray.iter().map(|c|*c as i32).min().unwrap() == 0 {
            for i in 0..7 {
                for j in 0..7 {
                    if dec_arr[i] == 1 << (j) {
                        if charray[j] != (0x61 + (i as u8)) as char {
                            for k in 0..7 {
                                if k == i {
                                    continue;
                                }
                                dec_arr[k] &= !(1 << (j));
                            }
                            charray[j] = (0x61 + (i as u8)) as char;
                            continue;
                        }
                    }
                }
            }
            for ex in test.split_whitespace() {
                match ex.len() {
                    2 => {
                        for c in "abcdefg".chars() {
                            if ex.chars().any(|ch|ch==c) {
                                dec_arr[(c as usize - 0x61)] &= 0b00100100u8;
                            } else {
                                dec_arr[(c as usize - 0x61)] &= !0b00100100u8;
                            }
                        }
                    },
                    3 => {
                        for c in "abcdefg".chars() {
                            if ex.chars().any(|ch|ch==c) {
                                dec_arr[(c as usize - 0x61)] &= 0b00100101u8;
                            } else {
                                dec_arr[(c as usize - 0x61)] &= !0b00100101u8;
                            }
                        }
                    },
                    4 => {
                        for c in "abcdefg".chars() {
                            if ex.chars().any(|ch|ch==c) {
                                dec_arr[(c as usize - 0x61)] &= 0b00101110u8;
                            } else {
                                dec_arr[(c as usize - 0x61)] &= !0b00101110u8;
                            }
                        }
                    },
                    5 => {
                        if len2.chars().filter(|c|ex.chars().any(|ch|ch==*c)).count() == 2 {
                            for c in "abcdefg".chars() {
                                if ex.chars().any(|ch|ch==c) {
                                    dec_arr[(c as usize - 0x61)] &= 0b01101101u8;
                                } else {
                                    dec_arr[(c as usize - 0x61)] &= !0b01101101u8;
                                }
                            }
                        } else if len4.chars().filter(|c|ex.chars().any(|ch|ch==*c)).count() == 3 {
                            for c in "abcdefg".chars() {
                                if ex.chars().any(|ch|ch==c) {
                                    dec_arr[(c as usize - 0x61)] &= 0b01101011u8;
                                } else {
                                    dec_arr[(c as usize - 0x61)] &= !0b01101011u8;
                                }
                            }
                        } else {
                            for c in "abcdefg".chars() {
                                if ex.chars().any(|ch|ch==c) {
                                    dec_arr[(c as usize - 0x61)] &= 0b01011101u8;
                                } else {
                                    dec_arr[(c as usize - 0x61)] &= !0b01011101u8;
                                }
                            }
                        }
                    },
                    6 => {
                        if len4.chars().filter(|c|ex.chars().any(|ch|ch==*c)).count() == 4 {
                            for c in "abcdefg".chars() {
                                if ex.chars().any(|ch|ch==c) {
                                    dec_arr[(c as usize - 0x61)] &= 0b01101111u8;
                                } else {
                                    dec_arr[(c as usize - 0x61)] &= !0b01101111u8;
                                }
                            }
                        } else if len2.chars().filter(|c|ex.chars().any(|ch|ch==*c)).count() == 2 {
                            for c in "abcdefg".chars() {
                                if ex.chars().any(|ch|ch==c) {
                                    dec_arr[(c as usize - 0x61)] &= 0b01110111u8;
                                } else {
                                    dec_arr[(c as usize - 0x61)] &= !0b01110111u8;
                                }
                            }
                        } else {
                            for c in "abcdefg".chars() {
                                if ex.chars().any(|ch|ch==c) {
                                    dec_arr[(c as usize - 0x61)] &= 0b01111011u8;
                                } else {
                                    dec_arr[(c as usize - 0x61)] &= !0b01111011u8;
                                }
                            }
                        }
                    },
                    _ => {}
                }
            }
        }

        let mut value = 0;

        for str in split.next().unwrap().split_whitespace() {
            match str.len() {
                2 => {
                    value = value*10 + 1;
                },
                3 => {
                    value = value*10 + 7;
                },
                4 => {
                    value = value*10 + 4;
                },
                7 => {
                    value = value*10 + 8;
                },
                5 => {
                    if str.chars().any(|c| c == charray[4]) {
                        value = value*10 + 2;
                    } else if str.chars().any(|c| c == charray[2]) {
                        value = value*10 + 3;
                    } else {
                        value = value*10 + 5;
                    }
                },
                6 => {
                    if !str.chars().any(|c| c == charray[3]) {
                        value = value*10 + 0;
                    } else if str.chars().any(|c| c == charray[2]) {
                        value = value*10 + 9;
                    } else {
                        value = value*10 + 6;
                    }
                },
                _ => {
                    eprintln!("Default reached in last match");
                }
            }
        }
        value
    }).sum();

    println!("Total: {}", total);
}
