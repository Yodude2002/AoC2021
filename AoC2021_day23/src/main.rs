use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::BufRead;

impl Eq for State {}

impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

#[derive(Clone, Debug)]
struct State {
    hallway: [char;11],
    rooms: [char;16],
    weight: u64,
    last: Option<Box<State>>
}

fn main() {
    part_two();
}

fn part_one() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    lines.next();
    lines.next();

    let mut rooms: ((char,char),(char,char),(char,char),(char,char)) = (('\0','\0'),('\0','\0'),('\0','\0'),('\0','\0'));

    {
        let mut line = lines.next().unwrap().unwrap();
        let mut chars = line.chars();
        chars.next();
        chars.next();
        chars.next();
        rooms.0.0 = chars.next().unwrap();
        chars.next();
        rooms.1.0 = chars.next().unwrap();
        chars.next();
        rooms.2.0 = chars.next().unwrap();
        chars.next();
        rooms.3.0 = chars.next().unwrap();
    }
    {
        let mut line = lines.next().unwrap().unwrap();
        let mut chars = line.chars();
        chars.next();
        chars.next();
        chars.next();
        rooms.0.1 = chars.next().unwrap();
        chars.next();
        rooms.1.1 = chars.next().unwrap();
        chars.next();
        rooms.2.1 = chars.next().unwrap();
        chars.next();
        rooms.3.1 = chars.next().unwrap();
    }

    let init_state = State {
        hallway: ['\0';11],
        rooms: [rooms.0.0,rooms.1.0,rooms.2.0,rooms.3.0,rooms.0.1,rooms.1.1,rooms.2.1,rooms.3.1,'\0','\0','\0','\0','\0','\0','\0','\0'],
        weight: 0,
        last: None
    };

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(init_state);

    let mut memo: Vec<State> = Vec::new();

    let mut path = loop {
        if let Some(mut state) = heap.pop() {
            let state = state;
            if  state.rooms[0] == 'A' && state.rooms[4] == 'A' &&
                state.rooms[1] == 'B' && state.rooms[5] == 'B' &&
                state.rooms[2] == 'C' && state.rooms[6] == 'C' &&
                state.rooms[3] == 'D' && state.rooms[7] == 'D' {
                break state;
            }

            if memo.iter().any(|s|{
                s.rooms == state.rooms && s.hallway == state.hallway
            }) {
                continue;
            }

            let mut moves: Vec<State> = Vec::new();

            for i in 0..11_usize {
                let pod = state.hallway[i];
                if pod == '\0' {
                    continue;
                }

                let target: usize = match pod {
                    'A' => 2,
                    'B' => 4,
                    'C' => 6,
                    'D' => 8,
                    _ => unreachable!()
                };

                if i < target {
                    if ((i+1)..=target).all(|j|state.hallway[j] == '\0') {
                        if state.rooms[(target-2)/2] == '\0' {
                            let movement_cost = target-i + 1;
                            let cost_per: usize = match pod {
                                'A' => 1,
                                'B' => 10,
                                'C' => 100,
                                'D' => 1000,
                                _ => unreachable!()
                            };
                            if state.rooms[(target-2)/2 + 4] == '\0' {
                                let mut next_state = state.clone();
                                next_state.rooms[(target-2)/2 + 4] = next_state.hallway[i];
                                next_state.hallway[i] = '\0';
                                next_state.weight += ((movement_cost + 1)*cost_per) as u64;
                                moves.push(next_state);
                            } else if state.rooms[(target-2)/2 + 4] == pod {
                                let mut next_state = state.clone();
                                next_state.rooms[(target-2)/2] = next_state.hallway[i];
                                next_state.hallway[i] = '\0';
                                next_state.weight += (movement_cost*cost_per) as u64;
                                moves.push(next_state);
                            } else {
                                print!("")
                            }
                        }
                    }
                } else {
                    if (target..i).all(|j|state.hallway[j] == '\0') {
                        let movement_cost = i - target + 1;
                        let cost_per: usize = match pod {
                            'A' => 1,
                            'B' => 10,
                            'C' => 100,
                            'D' => 1000,
                            _ => unreachable!()
                        };
                        if state.rooms[(target-2)/2 + 4] == '\0' {
                            let mut next_state = state.clone();
                            next_state.rooms[(target-2)/2 + 4] = next_state.hallway[i];
                            next_state.hallway[i] = '\0';
                            next_state.weight += ((movement_cost + 1)*cost_per) as u64;
                            moves.push(next_state);
                        } else if state.rooms[(target-2)/2 + 4] == pod {
                            let mut next_state = state.clone();
                            next_state.rooms[(target-2)/2] = next_state.hallway[i];
                            next_state.hallway[i] = '\0';
                            next_state.weight += (movement_cost*cost_per) as u64;
                            moves.push(next_state);
                        } else {
                            print!("")
                        }
                    }
                }
            }

            if moves.is_empty() {
                for i in 0..8_usize {
                    let pod = state.rooms[i];
                    let correct_char = (((i%4) + 0x41) as u8) as char;
                    if pod == '\0' {
                        print!("");
                        continue;
                    }
                    if i > 3 && state.rooms[i-4] != '\0' {
                        print!("");
                        continue;
                    }
                    if i > 3 && pod == correct_char {
                        print!("");
                        continue;
                    }
                    if i < 4 && pod == correct_char && state.rooms[i+4] == correct_char {
                        print!("");
                        continue;
                    }

                    for j in 0..11_usize {
                        if j == 2 || j == 4 || j == 6 || j == 8 {
                            continue;
                        }
                        let h_pos = (i%4)*2 + 2;
                        if h_pos < j {
                            if (h_pos..=j).all(|k|state.hallway[k] == '\0') {
                                let movement_cost = j - h_pos + 1 + if i > 3 {1} else {0};
                                let cost_per = match pod {
                                    'A' => 1,
                                    'B' => 10,
                                    'C' => 100,
                                    'D' => 1000,
                                    _ => unreachable!()
                                };
                                let mut next_state = state.clone();
                                next_state.rooms[i] = '\0';
                                next_state.hallway[j] = pod;
                                next_state.weight += ((movement_cost)*cost_per) as u64;
                                moves.push(next_state);
                            }
                        } else {
                            if (j..=h_pos).all(|k|state.hallway[k] == '\0') {
                                let movement_cost = h_pos - j + 1 + if i > 3 {1} else {0};
                                let cost_per = match pod {
                                    'A' => 1,
                                    'B' => 10,
                                    'C' => 100,
                                    'D' => 1000,
                                    _ => unreachable!()
                                };
                                let mut next_state = state.clone();
                                next_state.rooms[i] = '\0';
                                next_state.hallway[j] = pod;
                                next_state.weight += (movement_cost*cost_per) as u64;
                                moves.push(next_state);
                            }
                        }
                    }
                }
            }


            for mut m in moves {
                //m.last = Some(Box::from(state.clone()));
                heap.push(m);
            }

            memo.push(state);
        } else {
            assert!(false);
            //break;
        }

    };

    //let path = find_path_p1(&state);

    let mut path_vec: Vec<State> = vec![path];

    loop {
        let st;
        if let Some(mut state) = path_vec.get_mut(0) {

            st = state.last.take();
        } else {
            break;
        }
        if let Some(s) = st {
            path_vec.insert(0, *s);
        } else {
            break;
        }
    }

    println!("Shortest Path: {}",path_vec.iter().rev().next().unwrap().weight);
}

fn part_two() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    lines.next();
    lines.next();

    let mut rooms: ((char,char),(char,char),(char,char),(char,char)) = (('\0','\0'),('\0','\0'),('\0','\0'),('\0','\0'));

    {
        let mut line = lines.next().unwrap().unwrap();
        let mut chars = line.chars();
        chars.next();
        chars.next();
        chars.next();
        rooms.0.0 = chars.next().unwrap();
        chars.next();
        rooms.1.0 = chars.next().unwrap();
        chars.next();
        rooms.2.0 = chars.next().unwrap();
        chars.next();
        rooms.3.0 = chars.next().unwrap();
    }
    {
        let mut line = lines.next().unwrap().unwrap();
        let mut chars = line.chars();
        chars.next();
        chars.next();
        chars.next();
        rooms.0.1 = chars.next().unwrap();
        chars.next();
        rooms.1.1 = chars.next().unwrap();
        chars.next();
        rooms.2.1 = chars.next().unwrap();
        chars.next();
        rooms.3.1 = chars.next().unwrap();
    }

    let init_state = State {
        hallway: ['\0';11],
        rooms: [
            rooms.0.0,rooms.1.0,rooms.2.0,rooms.3.0,
            'D', 'C', 'B', 'A',
            'D', 'B', 'A', 'C',
            rooms.0.1,rooms.1.1,rooms.2.1,rooms.3.1
        ],
        weight: 0,
        last: None
    };

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(init_state);

    let mut memo: Vec<State> = Vec::new();

    let mut progress: u64 = 0;

    let mut path = loop {
        if let Some(mut state) = heap.pop() {
            let state = state;
            if  state.rooms[0] == 'A' && state.rooms[4] == 'A' &&
                state.rooms[1] == 'B' && state.rooms[5] == 'B' &&
                state.rooms[2] == 'C' && state.rooms[6] == 'C' &&
                state.rooms[3] == 'D' && state.rooms[7] == 'D' &&
                state.rooms[8] == 'A' && state.rooms[12] == 'A' &&
                state.rooms[9] == 'B' && state.rooms[13] == 'B' &&
                state.rooms[10] == 'C' && state.rooms[14] == 'C' &&
                state.rooms[11] == 'D' && state.rooms[15] == 'D'{
                break state;
            }

            if state.weight > 1000 + progress {
                progress += 1000;
                println!("Progress passed {}",progress);
            }

            if memo.iter().any(|s|{
                s.rooms == state.rooms && s.hallway == state.hallway
            }) {
                continue;
            }

            let mut moves: Vec<State> = Vec::new();

            for i in 0..11_usize {
                let pod = state.hallway[i];
                if pod == '\0' {
                    continue;
                }

                let target: usize = match pod {
                    'A' => 2,
                    'B' => 4,
                    'C' => 6,
                    'D' => 8,
                    _ => unreachable!()
                };

                let room = (target-2)/2;

                if i < target {
                    if ((i+1)..=target).all(|j|state.hallway[j] == '\0') {
                        if let Some(depth) = (0..4_usize).filter(|&d|{
                            (0..=d).all(|j|{
                                state.rooms[room+(j*4)] == '\0'
                            }) && ((d+1)..4).all(|j|{
                                state.rooms[room+(j*4)] == pod
                            })
                        }).max() {
                            let movement_cost = target-i + 1 + depth;
                            let cost_per: usize = match pod {
                                'A' => 1,
                                'B' => 10,
                                'C' => 100,
                                'D' => 1000,
                                _ => unreachable!()
                            };
                            let mut next_state = state.clone();
                            next_state.rooms[room + 4*depth] = next_state.hallway[i];
                            next_state.hallway[i] = '\0';
                            next_state.weight += (movement_cost*cost_per) as u64;
                            moves.push(next_state);//TODO: propagate these changes
                        }
                    }
                } else {
                    if (target..i).all(|j|state.hallway[j] == '\0') {
                        if let Some(depth) = (0..4_usize).filter(|&d|{
                            (0..=d).all(|j|{
                                state.rooms[room+(j*4)] == '\0'
                            }) && ((d+1)..4).all(|j|{
                                state.rooms[room+(j*4)] == pod
                            })
                        }).max() {
                            let movement_cost = i - target + 1 + depth;
                            let cost_per: usize = match pod {
                                'A' => 1,
                                'B' => 10,
                                'C' => 100,
                                'D' => 1000,
                                _ => unreachable!()
                            };
                            let mut next_state = state.clone();
                            next_state.rooms[room + 4*depth] = next_state.hallway[i];
                            next_state.hallway[i] = '\0';
                            next_state.weight += (movement_cost*cost_per) as u64;
                            moves.push(next_state);//TODO: propagate these changes
                        }
                    }
                }
            }

            if moves.is_empty() {
                for i in 0..16_usize {
                    let pod = state.rooms[i];
                    let correct_char = (((i%4) + 0x41) as u8) as char;
                    if pod == '\0' {
                        print!("");
                        continue;
                    }
                    if (0..(i/4)).any(|j|state.rooms[j*4 + i%4] != '\0') {
                        continue;
                    }
                    if ((i%4)..4).all(|j|state.rooms[j*4 + i%4] == correct_char) {
                        continue;
                    }

                    for j in 0..11_usize {
                        if j == 2 || j == 4 || j == 6 || j == 8 {
                            continue;
                        }
                        let h_pos = (i%4)*2 + 2;
                        if h_pos < j {
                            if (h_pos..=j).all(|k|state.hallway[k] == '\0') {
                                let movement_cost = j - h_pos + 1 + (i/4);
                                let cost_per = match pod {
                                    'A' => 1,
                                    'B' => 10,
                                    'C' => 100,
                                    'D' => 1000,
                                    _ => unreachable!()
                                };
                                let mut next_state = state.clone();
                                next_state.rooms[i] = '\0';
                                next_state.hallway[j] = pod;
                                next_state.weight += (movement_cost*cost_per) as u64;
                                moves.push(next_state);
                            }
                        } else {
                            if (j..=h_pos).all(|k|state.hallway[k] == '\0') {
                                let movement_cost = h_pos - j + 1 + (i/4);
                                let cost_per = match pod {
                                    'A' => 1,
                                    'B' => 10,
                                    'C' => 100,
                                    'D' => 1000,
                                    _ => unreachable!()
                                };
                                let mut next_state = state.clone();
                                next_state.rooms[i] = '\0';
                                next_state.hallway[j] = pod;
                                next_state.weight += (movement_cost*cost_per) as u64;
                                moves.push(next_state);
                            }
                        }
                    }
                }
            }


            for mut m in moves {
                //m.last = Some(Box::from(state.clone()));
                heap.push(m);
            }

            memo.push(state);
        } else {
            assert!(false);
            //break;
        }

    };

    println!("Shortest Path: {}",path.weight);
}

fn find_path_p1(rooms: &State) -> Option<u64> {
    if  rooms.rooms[0] == 'A' && rooms.rooms[4] == 'A' &&
        rooms.rooms[1] == 'B' && rooms.rooms[5] == 'B' &&
        rooms.rooms[2] == 'C' && rooms.rooms[6] == 'C' &&
        rooms.rooms[3] == 'D' && rooms.rooms[7] == 'D' {
        return Some(0);
    }

    let mut moves: Vec<(State, u64)> = Vec::new();

    for i in 0..11_usize {
        let pod = rooms.hallway[i];
        if pod == '\0' {
            continue;
        }

        let target: usize = match pod {
            'A' => 2,
            'B' => 4,
            'C' => 6,
            'D' => 8,
            _ => unreachable!()
        };

        if i < target {
            if ((i+1)..=target).all(|j|rooms.hallway[j] == '\0') {
                if rooms.rooms[(target-2)/2] == '\0' {
                    let movement_cost = target-i + 1;
                    let cost_per: usize = match pod {
                        'A' => 1,
                        'B' => 10,
                        'C' => 100,
                        'D' => 1000,
                        _ => unreachable!()
                    };
                    if rooms.rooms[(target-2)/2 + 4] == '\0' {
                        let mut next_state = rooms.clone();
                        next_state.rooms[(target-2)/2 + 4] = next_state.hallway[i];
                        next_state.hallway[i] = '\0';
                        moves.push((next_state,((movement_cost + 1)*cost_per) as u64));
                    } else if rooms.rooms[(target-2)/2 + 4] == pod {
                        let mut next_state = rooms.clone();
                        next_state.rooms[(target-2)/2] = next_state.hallway[i];
                        next_state.hallway[i] = '\0';
                        moves.push((next_state,(movement_cost*cost_per) as u64));
                    } else {
                        print!("")
                    }
                }
            }
        } else {
            if ((i)..target).all(|j|rooms.hallway[j] == '\0') {
                let movement_cost = i - target + 1;
                let cost_per: usize = match pod {
                    'A' => 1,
                    'B' => 10,
                    'C' => 100,
                    'D' => 1000,
                    _ => unreachable!()
                };
                if rooms.rooms[(target-2)/2 + 4] == '\0' {
                    let mut next_state = rooms.clone();
                    next_state.rooms[(target-2)/2 + 4] = next_state.hallway[i];
                    next_state.hallway[i] = '\0';
                    moves.push((next_state,((movement_cost + 1)*cost_per) as u64));
                } else if rooms.rooms[(target-2)/2 + 4] == pod {
                    let mut next_state = rooms.clone();
                    next_state.rooms[(target-2)/2] = next_state.hallway[i];
                    next_state.hallway[i] = '\0';
                    moves.push((next_state,(movement_cost*cost_per) as u64));
                } else {
                    print!("")
                }
            }
        }
    }

    for i in 0..8_usize {
        let pod = rooms.rooms[i];
        let correct_char = (((i%4) + 0x41) as u8) as char;
        if pod == '\0' {
            print!("");
            continue;
        }
        if i > 3 && rooms.rooms[i-4] != '\0' {
            print!("");
            continue;
        }
        if i > 3 && pod == correct_char {
            print!("");
            continue;
        }
        if i < 4 && pod == correct_char && rooms.rooms[i+4] == correct_char {
            print!("");
            continue;
        }

        for j in 0..11_usize {
            if j == 2 || j == 2 || j == 2 || j == 2 {
                continue;
            }
            let h_pos = (i%4)*2 + 2;
            if h_pos < j {
                if (h_pos..=j).all(|k|rooms.hallway[k] == '\0') {
                    let movement_cost = j - h_pos + 1 + if i > 3 {1} else {0};
                    let cost_per = match pod {
                        'A' => 1,
                        'B' => 10,
                        'C' => 100,
                        'D' => 1000,
                        _ => unreachable!()
                    };
                    let mut next_state = rooms.clone();
                    next_state.rooms[i] = '\0';
                    next_state.hallway[j] = pod;
                    moves.push((next_state,((movement_cost)*cost_per) as u64));
                }
            } else {
                if (j..=h_pos).all(|k|rooms.hallway[k] == '\0') {
                    let movement_cost = h_pos - j + 1 + if i > 3 {1} else {0};
                    let cost_per = match pod {
                        'A' => 1,
                        'B' => 10,
                        'C' => 100,
                        'D' => 1000,
                        _ => unreachable!()
                    };
                    let mut next_state = rooms.clone();
                    next_state.rooms[i] = '\0';
                    next_state.hallway[j] = pod;
                    moves.push((next_state,(movement_cost*cost_per) as u64));
                }
            }
        }
    }

    moves.iter().filter_map(|(s,c)|{
        if let Some(v) = find_path_p1(s) {
            Some(v+*c)
        } else {
            None
        }
    }).min()
}
