use std::cmp::{max,min};
use std::io::{BufRead};
use crate::Transform::TRANSFORM;

enum Transform {
    TRANSFORM(u8)
}

impl Transform {
    pub fn transform(&self, n: (i64,i64,i64)) -> (i64,i64,i64) {
        if false {
            match self {
                TRANSFORM(0) => (n.0, n.1, n.2),
                TRANSFORM(1) => (n.0, -n.1, -n.2),
                TRANSFORM(2) => (n.0, n.2, -n.1),
                TRANSFORM(3) => (n.0, -n.2, n.1),
                TRANSFORM(4) => (-n.0, n.1, -n.2),
                TRANSFORM(5) => (-n.0, -n.1, n.2),
                TRANSFORM(6) => (-n.0, n.2, n.1),
                TRANSFORM(7) => (-n.0, -n.2, -n.1),
                TRANSFORM(8) => (n.1, n.0, -n.2),
                TRANSFORM(9) => (n.1, -n.0, n.2),
                TRANSFORM(10) => (n.1, n.2, n.0),
                TRANSFORM(11) => (n.1, -n.2, -n.0),
                TRANSFORM(12) => (-n.1, n.0, n.2),
                TRANSFORM(13) => (-n.1, -n.0, -n.2),
                TRANSFORM(14) => (-n.1, n.2, -n.0),
                TRANSFORM(15) => (-n.1, -n.2, n.0),
                TRANSFORM(16) => (n.2, n.0, n.1),
                TRANSFORM(17) => (n.2, -n.0, -n.1),
                TRANSFORM(18) => (n.2, n.1, -n.0),
                TRANSFORM(19) => (n.2, -n.1, n.0),
                TRANSFORM(20) => (-n.2, n.0, -n.1),
                TRANSFORM(21) => (-n.2, -n.0, n.1),
                TRANSFORM(22) => (-n.2, n.1, -n.0),
                TRANSFORM(23) => (-n.2, -n.1, n.0),
                _ => unreachable!()
            }
        } else {
            let Transform::TRANSFORM(op) = self;
            let x_pos = op / 16;
            let x_sgn = (op / 8) % 2;
            let mut y_pos = (op / 4) % 2;
            if x_pos <= y_pos {
                y_pos += 1;
            }
            let y_sgn = (op/2) % 2;
            let z_sgn = op % 2;

            let x_val = if x_sgn > 0 {-n.0} else {n.0};
            let y_val = if y_sgn > 0 {-n.1} else {n.1};
            let z_val = if z_sgn > 0 {-n.2} else {n.2};

            (if x_pos == 0 {x_val} else if y_pos == 0 {y_val} else {z_val},
             if x_pos == 1 {x_val} else if y_pos == 1 {y_val} else {z_val},
             if x_pos == 2 {x_val} else if y_pos == 2 {y_val} else {z_val})
        }
    }
}

fn main() {
    part_two();
}

fn part_one() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut scanners: Vec<Vec<(i64,i64,i64)>> = Vec::new();

    for line in lines {
        if line == "" {
            continue;
        }
        if line.chars().rev().next().unwrap() == '-' {
            scanners.push(Vec::new());
            continue;
        }

        let mut coord: (i64,i64,i64) = (0,0,0);

        for (i,n) in line.split(",").enumerate() {
            match i {
                0 => coord.0 = n.parse::<i64>().unwrap(),
                1 => coord.1 = n.parse::<i64>().unwrap(),
                2 => coord.2 = n.parse::<i64>().unwrap(),
                _ => {}
            }

        }
        scanners.last_mut().unwrap().push(coord);
    }

    let mut scanners_abs: Vec<((i64,i64,i64),Vec<(i64,i64,i64)>)> = Vec::new();
    scanners_abs.push(((0,0,0),scanners.remove(0)));


    while !scanners.is_empty() {
        let mut remove_queue: Vec<usize> = Vec::new();
        'scanner_for:
        for (scanner_index,scanner) in scanners.iter().enumerate() {
            //TODO: revert to the right way
            for transform in (0..48).map(|t|TRANSFORM(t)) {
                let scanner_t: Vec<(i64,i64,i64)> = scanner.iter().map(|t|transform.transform(*t)).collect();

                let foo = scanner_t.get(0).unwrap();
                //println!("({},{},{})",foo.0,foo.1,foo.2);
                if let TRANSFORM(47) = transform {
                    print!("")
                }

                for beacons in scanners_abs.iter().map(|((ax,ay,az),vec)|{
                    vec.iter().map(|&(x,y,z)|{
                        (x+*ax,y+*ay,z+*az)
                    }).collect::<Vec<(i64,i64,i64)>>()
                }) {
                    for i_b in 0..beacons.len() {
                        for i_s in 0..scanner_t.len() {
                            let scanner_m = mod_rel_vec(&scanner_t, i_s as u8);
                            let beacons_m = mod_rel_vec(&beacons, i_b as u8);

                            let matches = scanner_m.iter().filter(|tuple|
                                beacons_m.contains(*tuple)
                            ).count();

                            if matches > 2 {
                                print!("");
                            }

                            if matches >= 12 {
                                let &(x_b,y_b,z_b) = beacons.get(i_b).unwrap();
                                let &(x_s,y_s,z_s) = scanner_t.get(i_s).unwrap();
                                let scanner_pos_abs = (x_b - x_s, y_b - y_s, z_b - z_s);
                                scanners_abs.push((scanner_pos_abs,scanner_t));
                                remove_queue.push(scanner_index);
                                println!("Scanner found at position ({},{},{})",scanner_pos_abs.0,scanner_pos_abs.1,scanner_pos_abs.2);
                                continue 'scanner_for;
                            }
                        }
                    }
                }
            }
        }

        if remove_queue.is_empty() {
            /* *
            while !scanners.is_empty() {
                scanners.remove(0);
            }* */
            //continue;

            for (scanner_index,scanner) in scanners.iter().enumerate() {
                'scanner_for_2:
                for transform in (0..48).map(|t|TRANSFORM(t)) {
                    if let TRANSFORM(6) = transform {
                        print!("");
                    }

                    let scanner_t: Vec<(i64,i64,i64)> = scanner.iter().map(|t|transform.transform(*t)).collect();

                    let mut beacons: Vec<(i64,i64,i64)> = Vec::new();
                    for vec in scanners_abs.iter().map(|((ax,ay,az),vec)|{
                        vec.iter().map(|&(x,y,z)|{
                            (x+*ax,y+*ay,z+*az)
                        }).collect::<Vec<(i64,i64,i64)>>()
                    }) {
                        for t in vec {
                            beacons.push(t);
                        }
                    }
                    beacons.sort();
                    beacons.dedup();

                    for i_b in 0..beacons.len() {
                        for i_s in 0..scanner_t.len() {
                            let scanner_m = mod_rel_vec(&scanner_t, i_s as u8);
                            let beacons_m = mod_rel_vec(&beacons, i_b as u8);

                            let matches = scanner_m.iter().filter(|tuple| {
                                beacons_m.contains(tuple)// && (diff.0 <= 1000 && diff.1 <= 1000 && diff.2 <= 1000)
                            }).count();

                            //if scanner_m.contains(-x,z,y) { 0, 1, 1, 0 }

                            if scanner_m.contains(&(889,-600,563)) {
                                print!("");
                            }

                            if matches > 2 {
                                print!("")
                            }

                            if matches >= 12 {
                                let &(x_b,y_b,z_b) = beacons.get(i_b).unwrap();
                                let &(x_s,y_s,z_s) = scanner_t.get(i_s).unwrap();
                                let scanner_pos_abs = (x_b - x_s, y_b - y_s, z_b - z_s);
                                scanners_abs.push((scanner_pos_abs,scanner_t));
                                remove_queue.push(scanner_index);
                                println!("Scanner found at position ({},{},{})",scanner_pos_abs.0,scanner_pos_abs.1,scanner_pos_abs.2);
                                break 'scanner_for_2;
                            }
                        }
                    }
                }
            }
        }

        remove_queue.sort();

        for &i in remove_queue.iter().rev() {
            if i < scanners.len() {
                scanners.remove(i);
            }
        }
    }

    let beacons_vec = scanners_abs.iter().map(|((ax,ay,az),vec)|{
        vec.iter().map(|&(x,y,z)|{
            (x+*ax,y+*ay,z+*az)
        }).collect::<Vec<(i64,i64,i64)>>()
    }).collect::<Vec<Vec<(i64,i64,i64)>>>();
    let mut beacons: Vec<(i64,i64,i64)> = Vec::new();
    for vec in beacons_vec {
        for t in vec {
            beacons.push(t);
        }
    }
    beacons.sort();
    beacons.dedup();

    for &t in &beacons {
        if (t.0-1105).abs() <= 1000 && (t.1+1205).abs() <= 1000 && (t.2-1229).abs() <= 1000 {
            //println!("Beacon: ({}, {}, {})",t.0,t.1,t.2);
        }
    }

    println!("Beacons: {}",beacons.len());
}

fn part_two() {
    /* */
    let scanners: Vec<(i64,i64,i64)> = vec![
            (108,-1254,-76),
            (-1155,-1259,-2),
            (101,-1192,-1169),
            (41,-1150,1244),
            (-1007,-1211,1138),
            (94,-1150,-2517),
            (84,-1142,2403),
            (-2255,-1123,-11),
            (1222,-1292,1106),
            (42,-84,1161),
            (59,1202,1119),
            (1248,-92,1150),
            (1226,1228,1133),
            (1337,-2482,1108),
            (1380,-1177,2341),
            (147,-2303,-2414),
            (2528,-2466,1184),
            (63,-2422,2442),
            (3650,-2428,1199),
            (2512,-32,1212),
            (1386,1215,23),
            (1359,-2363,2386),
            (1219,2450,1192),
            (3788,-1230,1220),
            (1274,2314,-144),
            (1316,3662,1082)
    ];/* */
    /*
    let scanners: Vec<(i64,i64,i64)> = vec![
        (68,-1246,-43),
        (-92,-2380,-20),
        (-20,-1133,1061),
        (1105,-1205,1229)
    ];*/

    let max = scanners.iter().map(|&(ax, ay, az)| {
        scanners.iter().map(|&(bx, by, bz)| {
            (ax - bx).abs() + (ay - by).abs() + (az - bz).abs()
        }).max().unwrap()
    }).max().unwrap();

    println!("Max Dist: {}",max);
}

fn mod_rel_vec(vec: &Vec<(i64,i64,i64)>, index: u8) -> Vec<(i64,i64,i64)> {
    let first = vec.get(index as usize).unwrap();
    vec.iter().map(|&(a,b,c)|(a-first.0,b-first.1,c-first.2)).collect()
}

fn tuple_abs(tuple: &(i64,i64,i64)) -> (i64,i64,i64) {
    //*tuple
    (tuple.0.abs(),tuple.1.abs(),tuple.2.abs())
}