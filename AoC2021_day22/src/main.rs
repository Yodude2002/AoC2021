use std::collections::{HashSet};
use std::io::BufRead;

type Vec3 = (i64,i64,i64);

fn main() {
    part_two_bad_2();
}

fn part_one() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut small_grid: HashSet<(i64,i64,i64)> = HashSet::new();

    for line in lines {
        let mut spl = line.split(" ");
        let on = spl.next().unwrap() == "on";
        let mut coords:((i64,i64,i64),(i64,i64,i64)) = ((0,0,0),(0,0,0));
        for (i,s) in spl.next().unwrap().split(",").enumerate() {
            let mut range = s.rsplit("=").next().unwrap().split("..");
            let r1 = range.next().unwrap().parse::<i64>().unwrap();
            let r2 = range.next().unwrap().parse::<i64>().unwrap();
            match i {
                0 => {
                    coords.0.0 = r1;
                    coords.1.0 = r2;
                },
                1 => {
                    coords.0.1 = r1;
                    coords.1.1 = r2;
                },
                2 => {
                    coords.0.2 = r1;
                    coords.1.2 = r2;
                },
                _ => unreachable!()
            }
        };
        if coords.0.0.abs() > 50 {
            break;
        }
        if on {
            for x in coords.0.0..=coords.1.0 {
                for y in coords.0.1..=coords.1.1 {
                    for z in coords.0.2..=coords.1.2 {
                        small_grid.insert((x,y,z));
                    }
                }
            }
        } else {
            small_grid.retain(|&(x,y,z)|{
                coords.0.0 > x || x > coords.1.0 ||
                coords.0.1 > y || y > coords.1.1 ||
                coords.0.2 > z || z > coords.1.2
            })
        }
    }

    let on = small_grid.len();

    println!("Size: {}",on);
}

fn part_two() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut vec_x: Vec<i64> = Vec::new();
    let mut vec_y: Vec<i64> = Vec::new();
    let mut vec_z: Vec<i64> = Vec::new();

    let mut vec_grid: Vec<(Vec3, Vec3)> = Vec::new();

    let mut instructions: Vec<((Vec3, Vec3),bool)> = Vec::new();

    for line in lines {
        let mut spl = line.split(" ");
        let on = spl.next().unwrap() == "on";
        let mut coords:((i64,i64,i64),(i64,i64,i64)) = ((0,0,0),(0,0,0));
        for (i,s) in spl.next().unwrap().split(",").enumerate() {
            let mut range = s.rsplit("=").next().unwrap().split("..");
            let r1 = range.next().unwrap().parse::<i64>().unwrap();
            let r2 = range.next().unwrap().parse::<i64>().unwrap();
            match i {
                0 => {
                    coords.0.0 = r1;
                    coords.1.0 = r2;
                },
                1 => {
                    coords.0.1 = r1;
                    coords.1.1 = r2;
                },
                2 => {
                    coords.0.2 = r1;
                    coords.1.2 = r2;
                },
                _ => unreachable!()
            }
        };

        instructions.push((coords, on));

        vec_x.push(coords.0.0);
        vec_x.push(coords.1.0);
        vec_y.push(coords.0.1);
        vec_y.push(coords.1.1);
        vec_z.push(coords.0.2);
        vec_z.push(coords.1.2);
    }

    let mut sum = 0_i64;

    for &(pos, on) in instructions.iter().rev() {
        //let ((lx, ly, lz), (ux, uy, uz)) = pos;
        if on {
            let d_sum = range_outside(&pos, &vec_grid);
            sum += d_sum;
            /*
            let dx = ((ux-lx).abs()+1);
            let dy = ((uy-ly).abs()+1);
            let dz = ((uz-lz).abs()+1);
            let amt = dx*dy*dz;
            sum += amt;*/
        }
        vec_grid.push(pos);
    }

    println!("Sum: {}",sum);
}

fn part_two_bad_2() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut vec_x: Vec<i64> = Vec::new();
    let mut vec_y: Vec<i64> = Vec::new();
    let mut vec_z: Vec<i64> = Vec::new();

    let mut vec_grid: Vec<((Vec3, Vec3),bool)> = Vec::new();

    let mut instructions: Vec<((Vec3, Vec3),bool)> = Vec::new();

    for line in lines {
        let mut spl = line.split(" ");
        let on = spl.next().unwrap() == "on";
        let mut coords:(Vec3,Vec3) = ((0,0,0),(0,0,0));
        for (i,s) in spl.next().unwrap().split(",").enumerate() {
            let mut range = s.rsplit("=").next().unwrap().split("..");
            let r1 = range.next().unwrap().parse::<i64>().unwrap();
            let r2 = range.next().unwrap().parse::<i64>().unwrap();
            match i {
                0 => {
                    coords.0.0 = r1;
                    coords.1.0 = r2;
                },
                1 => {
                    coords.0.1 = r1;
                    coords.1.1 = r2;
                },
                2 => {
                    coords.0.2 = r1;
                    coords.1.2 = r2;
                },
                _ => unreachable!()
            }
        };

        instructions.push((coords, on));

        vec_x.push(coords.0.0);
        vec_x.push(coords.1.0);
        vec_y.push(coords.0.1);
        vec_y.push(coords.1.1);
        vec_z.push(coords.0.2);
        vec_z.push(coords.1.2);
    }

    vec_x.sort();
    vec_y.sort();
    vec_z.sort();
    vec_x.dedup();
    vec_y.dedup();
    vec_z.dedup();

    let mut x_last: Option<i64> = None;
    let mut y_last: Option<i64> = None;
    let mut z_last: Option<i64> = None;

    let mut pair_x: Vec<(i64,i64)> = Vec::new();
    let mut pair_y: Vec<(i64,i64)> = Vec::new();
    let mut pair_z: Vec<(i64,i64)> = Vec::new();

    for x in vec_x {
        if let Some(last_x) = x_last {
            pair_x.push((last_x+1,x-1));
        }
        x_last = Some(x);
        pair_x.push((x,x));
    }
    for y in vec_y {
        if let Some(last_y) = y_last {
            pair_y.push((last_y+1,y-1));
        }
        y_last = Some(y);
        pair_y.push((y,y));
    }
    for z in vec_z {
        if let Some(last_z) = z_last {
            pair_z.push((last_z+1,z-1));
        }
        z_last = Some(z);
        pair_z.push((z,z));
    }

    pair_x.retain(|&(l,u)|l<=u);
    pair_y.retain(|&(l,u)|l<=u);
    pair_z.retain(|&(l,u)|l<=u);

    //TEST
    /* *
    pair_x = (-50..=50).map(|i|(i,i)).collect();
    pair_y = (-50..=50).map(|i|(i,i)).collect();
    pair_z = (-50..=50).map(|i|(i,i)).collect();
    * */

    let mut vec_grid_eff: Vec<Vec<Vec<bool>>> = vec![vec![vec![false;pair_z.len()];pair_y.len()];pair_x.len()];

    /*
    for (i,&(lx,ux)) in pair_x.iter().enumerate() {
        for (j,&(ly,uy)) in pair_y.iter().enumerate() {
            for (k,&(lz,uz)) in pair_z.iter().enumerate() {
                vec_grid.push((((lx,ly,lz),(ux,uy,uz)),false));
            }
        }
    }*/

    //size should be (2x+1)*(2y+1)*(2z+1) = 8xyz + 4yz + 4xz + 4xy + 2x + 2y + 2z + 1
    //size = 8xyz + 4yz + 4xz + 4xy + 2x + 2y + 2z + 1
    /*
    for &x in &vec_x {
        if let Some(last_x) = x_last {
            for &y in &vec_y {
                if let Some(last_y) = y_last {
                    for &z in &vec_z {
                        if let Some(last_z) = z_last {
                            vec_grid.push((((last_x+1, last_y+1,   last_z+1),  (x-1,   y-1,z-1)),false));
                            vec_grid.push((((last_x+1, last_y+1,   z),         (x-1,   y-1,z)),false));
                            vec_grid.push((((last_x+1, y,          last_z+1),  (x-1,   y,  z-1)),false));
                            vec_grid.push((((last_x+1, y,          z),         (x-1,   y,  z)),false));
                            vec_grid.push((((x,        last_y+1,   last_z+1),  (x,     y-1,z-1)),false));
                            vec_grid.push((((x,        last_y+1,   z),         (x,     y-1,z)),false));
                            vec_grid.push((((x,        y,          last_z+1),  (x,     y,  z-1)),false));
                            vec_grid.push((((x,        y,          z),         (x,     y,  z)),false));
                        } else {
                            vec_grid.push((((last_x+1,last_y+1,z),(x-1,y-1,z)),false));
                            vec_grid.push((((last_x+1,y,z),(x-1,y,z)),false));
                            vec_grid.push((((x,last_y+1,z),(x,y-1,z)),false));
                            vec_grid.push((((x,y,z),(x,y,z)),false));
                        }
                        /*if let Some(last_z) = z_last {
                            grid.insert(((last_x+1, last_y+1,   last_z+1),  (x-1,   y-1,z-1)),false);
                            grid.insert(((last_x+1, last_y+1,   z),         (x-1,   y-1,z)),false);
                            grid.insert(((last_x+1, y,          last_z+1),  (x-1,   y,  z-1)),false);
                            grid.insert(((last_x+1, y,          z),         (x-1,   y,  z)),false);
                            grid.insert(((x,        last_y+1,   last_z+1),  (x,     y-1,z-1)),false);
                            grid.insert(((x,        last_y+1,   z),         (x,     y-1,z)),false);
                            grid.insert(((x,        y,          last_z+1),  (x,     y,  z-1)),false);
                            grid.insert(((x,        y,          z),         (x,     y,  z)),false);
                        } else {
                            grid.insert(((last_x+1,last_y+1,z),(x-1,y-1,z)),false);
                            grid.insert(((last_x+1,y,z),(x-1,y,z)),false);
                            grid.insert(((x-1,last_y+1,z),(x,y-1,z)),false);
                            grid.insert(((x-1,y,z),(x,y,z)),false);
                        }*/
                        z_last = Some(z);
                    }
                    z_last = None;
                } else {
                    for &z in &vec_z {
                        if let Some(last_z) = z_last {
                            vec_grid.push((((last_x+1, y,  last_z+1),  (x-1,   y,  z-1)),false));
                            vec_grid.push((((last_x+1, y,  z),         (x-1,   y,  z)),false));
                            vec_grid.push((((x,        y,  last_z+1),  (x,     y,  z-1)),false));
                            vec_grid.push((((x,        y,  z),         (x,     y,  z)),false));
                        } else {
                            vec_grid.push((((last_x+1,y,z),(x-1,y,z)),false));
                            vec_grid.push((((x,y,z),(x,y,z)),false));
                        }
                        z_last = Some(z);
                    }
                    z_last = None;
                }
                y_last = Some(y);
            }
            y_last = None;
        } else {
            for &y in &vec_y {
                if let Some(last_y) = y_last {
                    for &z in &vec_z {
                        if let Some(last_z) = z_last {
                            vec_grid.push((((x,last_y+1,   last_z+1),  (x, y-1,z-1)),false));
                            vec_grid.push((((x,last_y+1,   z),         (x, y-1,z)),false));
                            vec_grid.push((((x,y,          last_z+1),  (x, y,  z-1)),false));
                            vec_grid.push((((x,y,          z),         (x, y,  z)),false));
                        } else {
                            vec_grid.push((((x,last_y+1,z),(x,y-1,z)),false));
                            vec_grid.push((((x,y,z),(x,y,z)),false));
                        }
                        z_last = Some(z);
                    }
                    z_last = None;
                } else {
                    for &z in &vec_z {
                        if let Some(last_z) = z_last {
                            vec_grid.push((((x,y,  last_z+1),  (x, y,  z-1)),false));
                            vec_grid.push((((x,y,  z),         (x, y,  z)),false));
                        } else {
                            vec_grid.push((((x,y,z),(x,y,z)),false));
                        }
                        z_last = Some(z);
                    }
                    z_last = None;
                }
                y_last = Some(y);
            }
            y_last = None;
        }
        x_last = Some(x);
    }*/

    vec_grid.retain(|(t,_b)|{
        t.0.0 <= t.1.0 &&
        t.0.1 <= t.1.1 &&
        t.0.2 <= t.1.2
    });

    /*
    for o in &vec_grid {
        for i in &vec_grid {
            if *o != *i && range_overlap(&o.0,&i.0) {
                print!("")
            }
        }
    }*/


    //yeah theres definitely faster ways to do this, but I can't be bothered.
    for (i,(pos,on)) in instructions.iter().enumerate() {
        for x in 0..pair_x.len() {
            let x_pair = pair_x.get(x).unwrap();
            if pos.0.0 <= x_pair.0 &&  x_pair.1 <= pos.1.0 {
                for y in 0..pair_y.len() {
                    let y_pair = pair_y.get(y).unwrap();
                    if pos.0.1 <= y_pair.0 &&  y_pair.1 <= pos.1.1 {
                        for z in 0..pair_z.len() {
                            let z_pair = pair_z.get(z).unwrap();
                            if pos.0.2 <= z_pair.0 &&  z_pair.1 <= pos.1.2 {
                                vec_grid_eff[x][y][z] = *on;
                            }
                        }
                    }
                }
            }
        }
        /*
        for (grid_pos, grid_on) in &mut vec_grid {
            if  pos.0.0 <= grid_pos.0.0 &&  grid_pos.1.0 <= pos.1.0 &&
                pos.0.1 <= grid_pos.0.1 &&  grid_pos.1.1 <= pos.1.1 &&
                pos.0.2 <= grid_pos.0.2 &&  grid_pos.1.2 <= pos.1.2 {
                *grid_on = *on;
            }
        }*/
        println!("Processed Intruction {}",i);
    }

    let voxels_on: i64 = vec_grid_eff.iter().enumerate().map(|(x,y_vec)|{
        y_vec.iter().enumerate().map(|(y,z_vec)|{
            z_vec.iter().enumerate().filter_map(|(z,on)| {
                if *on {
                    let rx = pair_x.get(x).unwrap();
                    let ry = pair_y.get(y).unwrap();
                    let rz = pair_z.get(z).unwrap();
                    let dx = (rx.1-rx.0).abs()+1;
                    let dy = (ry.1-ry.0).abs()+1;
                    let dz = (rz.1-rz.0).abs()+1;
                    let amt = dx*dy*dz;
                    Some(amt)
                } else {
                    None
                }
            }).sum::<i64>()
        }).sum::<i64>()
    }).sum::<i64>();

    /*
    let voxels_on: i64 = vec_grid.iter().filter_map(|((a,b),on)|{
        if *on {
            let dx = (b.0-a.0).abs()+1;
            let dy = (b.1-a.1).abs()+1;
            let dz = (b.2-a.2).abs()+1;
            let amt = dx*dy*dz;
            Some(amt)
        } else {
            None
        }
    }).sum();*/

    println!("Size: {}",voxels_on);
}

fn part_two_bad() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines().map(|l|l.unwrap());

    let mut grid: HashSet<((i64, i64, i64), (i64, i64, i64))> = HashSet::new();

    let mut neg_grig: HashSet<((i64, i64, i64), (i64, i64, i64))> = HashSet::new();

    for line in lines {
        let mut spl = line.split(" ");
        let on = spl.next().unwrap() == "on";
        let mut coords:((i64,i64,i64),(i64,i64,i64)) = ((0,0,0),(0,0,0));
        for (i,s) in spl.next().unwrap().split(",").enumerate() {
            let mut range = s.rsplit("=").next().unwrap().split("..");
            let r1 = range.next().unwrap().parse::<i64>().unwrap();
            let r2 = range.next().unwrap().parse::<i64>().unwrap();
            match i {
                0 => {
                    coords.0.0 = r1;
                    coords.1.0 = r2;
                },
                1 => {
                    coords.0.1 = r1;
                    coords.1.1 = r2;
                },
                2 => {
                    coords.0.2 = r1;
                    coords.1.2 = r2;
                },
                _ => unreachable!()
            }
        };

        let mut overlaps: Vec<((i64,i64,i64),(i64,i64,i64))> = grid.iter().filter(|t|
            range_overlap(*t,&coords)).map(|&t|t).collect();
        /*loop {
            if overlaps.is_empty() {
                break;
            }
            let t = overlaps.get(0).unwrap();
            let ((mut lx,mut ly,mut lz),(mut ux,mut uy,mut uz)) = *t;
            let mut vec: Vec<((i64,i64,i64),(i64,i64,i64))> = Vec::new();
            /*let vec_old =
                {
            vec![
                ((lx,           ly,             lz),            (coords.0.0-1,    coords.0.1-1,   coords.0.2-1)),
                ((lx,           ly,             coords.0.2),    (coords.0.0-1,    coords.0.1-1,   coords.1.2)),
                ((lx,           ly,             coords.1.2+1),  (coords.0.0-1,    coords.0.1-1,   uz)),
                ((lx,           coords.0.1,     lz),            (coords.0.0-1,    coords.1.1,     coords.0.2-1)),
                ((lx,           coords.0.1,     coords.0.2),    (coords.0.0-1,    coords.1.1,     coords.1.2)),
                ((lx,           coords.0.1,     coords.1.2+1),  (coords.0.0-1,    coords.1.1,     uz)),
                ((lx,           coords.1.1+1,   lz),            (coords.0.0-1,    uy,             coords.0.2-1)),
                ((lx,           coords.1.1+1,   coords.0.2),    (coords.0.0-1,    uy,             coords.1.2)),
                ((lx,           coords.1.1+1,   coords.1.2+1),  (coords.0.0-1,    uy,             uz)),

                ((coords.0.0,   ly,             uz),            (coords.1.0,    coords.0.1-1,   coords.0.2-1)),
                ((coords.0.0,   ly,             coords.0.2),    (coords.1.0,    coords.0.1-1,   coords.1.2)),
                ((coords.0.0,   ly,             coords.1.2+1),  (coords.1.0,    coords.0.1-1,   uz)),
                ((coords.0.0,   coords.1.0,     uz),            (coords.1.0,    coords.1.1,     coords.0.2-1)),

                ((coords.0.0,   coords.1.0,     coords.1.2+1),  (coords.1.0,    coords.1.1,     uz)),
                ((coords.0.0,   coords.1.1+1,   uz),            (coords.1.0,    uy,             coords.0.2-1)),
                ((coords.0.0,   coords.1.1+1,   coords.0.2),    (coords.1.0,    uy,             coords.1.2)),
                ((coords.0.0,   coords.1.1+1,   coords.1.2+1),  (coords.1.0,    uy,             uz)),

                ((coords.1.0+1, ly,             lz),            (ux,            coords.0.1-1,   coords.0.2-1)),
                ((coords.1.0+1, ly,             coords.0.2),    (ux,            coords.0.1-1,   coords.1.2)),
                ((coords.1.0+1, ly,             coords.1.2+1),  (ux,            coords.0.1-1,   uz)),
                ((coords.1.0+1, coords.0.1,     lz),            (ux,            coords.1.1,     coords.0.2-1)),
                ((coords.1.0+1, coords.0.1,     coords.0.2),    (ux,            coords.1.1,     coords.1.2)),
                ((coords.1.0+1, coords.0.1,     coords.1.2+1),  (ux,            coords.1.1,     uz)),
                ((coords.1.0+1, coords.1.1+1,   lz),            (ux,            uy,             coords.0.2-1)),
                ((coords.1.0+1, coords.1.1+1,   coords.0.2),    (ux,            uy,             coords.1.2)),
                ((coords.1.0+1, coords.1.1+1,   coords.1.2+1),  (ux,            uy,             uz))
            ];*/
            loop {
                if lx < coords.0.0 && coords.0.0 < ux {
                    vec.push(((lx,ly,lz),(coords.0.0-1,uy,uz)));
                    lx = coords.0.0;
                } else if lx < coords.1.0 && coords.1.0 < ux {
                    vec.push(((coords.1.0+1,ly,lz),(ux,uy,uz)));
                    ux = coords.1.0;
                } else if ly < coords.0.1 && coords.0.1 < uy {
                    vec.push(((lx,ly,lz),(ux,coords.0.1-1,uz)));
                    ly = coords.0.1;
                } else if ly < coords.1.1 && coords.1.1 < uy {
                    vec.push(((lx,coords.1.1+1,lz),(ux,uy,uz)));
                    uy = coords.1.1;
                } else if lz < coords.0.2 && coords.0.2 < uz {
                    vec.push(((lx,ly,lz),(ux,uy,coords.0.2-1)));
                    lz = coords.0.2;
                } else if lz < coords.1.2 && coords.1.2 < uz {
                    vec.push(((lx,ly,coords.1.2+1),(ux,uy,uz)));
                    uz = coords.1.2;
                } else {
                    break;
                }
            }

            //((t1.1.0.min(t2.1.0) - t1.0.0.max(t2.0.0)).abs()+1) *
            //                 ((t1.1.1.min(t2.1.1) - t1.0.1.max(t2.0.1)).abs()+1) *
            //                 ((t1.1.2.min(t2.1.2) - t1.0.2.max(t2.0.2)).abs()+1);

            //grid.remove(t);
            let vec_valid: Vec<((i64,i64,i64),(i64,i64,i64))> = vec.iter()
                .filter(|&t|{
                    if !range_overlap(&coords,t){
                        true
                    } else {
                        print!("");
                        false
                    }
                }).filter(|&t_out|{
                if !vec.iter().any(|&t_in| {
                    if range_overlap(t_out, &t_in) {
                        *t_out != t_in
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    print!("");
                    false
                }
            })
                .map(|&t|t).collect();
            for &((lx,ly,lz),(ux,uy,uz)) in &vec_valid {
                //grid.insert(((lx,ly,lz),(ux,uy,uz)));
            }
        }*/
        for t in &overlaps {
            let (( lx, ly, lz),( ux, uy, uz)) = *t;
            neg_grig.insert(((lx.max(coords.0.0),ly.max(coords.0.1),lz.max(coords.0.2)),
                             (ux.min(coords.1.0),uy.min(coords.1.1),uz.min(coords.1.2))));
        }
        if on {
            grid.insert(coords);
        }
    }

    let mut overlap_on: i64 = 0;

    for t1 in &grid {
        for t2 in &grid {
            if *t1 == *t2 {
                continue;
            }
            if range_overlap(t1,t2) {
                /*overlap_on +=
                ((t1.1.0.min(t2.1.0) - t1.0.0.max(t2.0.0)).abs()+1) *
                ((t1.1.1.min(t2.1.1) - t1.0.1.max(t2.0.1)).abs()+1) *
                ((t1.1.2.min(t2.1.2) - t1.0.2.max(t2.0.2)).abs()+1);*/
                    print!("");
            }
        }
    }

    overlap_on += neg_grig.iter().map(|(a,b)|{
        let dx = (b.0-a.0).abs()+1;
        let dy = (b.1-a.1).abs()+1;
        let dz = (b.2-a.2).abs()+1;
        dx*dy*dz
    }).sum::<i64>();

    let on: i64 = grid.iter().map(|(a,b)|{
        let dx = (b.0-a.0).abs()+1;
        let dy = (b.1-a.1).abs()+1;
        let dz = (b.2-a.2).abs()+1;
        dx*dy*dz
    }).sum();

    println!("Size: {}",on - (overlap_on));
}

fn range_overlap(a: &(Vec3,Vec3), b:&(Vec3,Vec3)) -> bool {
    ((a.0.0 <= b.0.0 && b.0.0 <= a.1.0) || (a.0.0 <= b.1.0 && b.1.0 <= a.1.0)) &&
    ((a.0.1 <= b.0.1 && b.0.1 <= a.1.1) || (a.0.1 <= b.1.1 && b.1.1 <= a.1.1)) &&
    ((a.0.2 <= b.0.2 && b.0.2 <= a.1.2) || (a.0.2 <= b.1.2 && b.1.2 <= a.1.2))
}

fn range_outside(range: &(Vec3,Vec3), vec: &Vec<(Vec3,Vec3)>) -> i64 {
    let mut overlap_vec: Vec<(Vec3, Vec3)> = vec.iter().filter_map(|t|{
        if range_overlap(range, t) {
            Some(*t)
        } else {
            None
        }
    }).collect();

    if overlap_vec.is_empty() {
        let dx = (range.1.0-range.0.0).abs()+1;
        let dy = (range.1.1-range.0.1).abs()+1;
        let dz = (range.1.2-range.0.2).abs()+1;
        return dx*dy*dz;
    }

    match 1 {
        0 => {
            let mut x_vals = vec![range.0.0, range.1.0];
            let mut y_vals = vec![range.0.1, range.1.1];
            let mut z_vals = vec![range.0.2, range.1.2];
            for &((lx,ly,lz),(ux,uy,uz)) in &overlap_vec {
                x_vals.push(lx);
                x_vals.push(ux);
                y_vals.push(ly);
                y_vals.push(uy);
                z_vals.push(lz);
                z_vals.push(uz);
            }

            x_vals.sort();
            x_vals.dedup();
            y_vals.sort();
            y_vals.dedup();
            z_vals.sort();
            z_vals.dedup();

            x_vals.retain(|&x| range.0.0 <= x && x <= range.1.0);
            y_vals.retain(|&y| range.0.1 <= y && y <= range.1.1);
            z_vals.retain(|&z| range.0.2 <= z && z <= range.1.2);

            let mut x_pairs: Vec<(i64,i64)> = Vec::new();
            let mut y_pairs: Vec<(i64,i64)> = Vec::new();
            let mut z_pairs: Vec<(i64,i64)> = Vec::new();

            let mut x_last: Option<i64> = None;
            let mut y_last: Option<i64> = None;
            let mut z_last: Option<i64> = None;

            for &x in &x_vals {
                if let Some(last_x) = x_last {
                    x_pairs.push((last_x+1,x-1));
                }
                x_last = Some(x);
                x_pairs.push((x,x));
            }
            for &y in &y_vals {
                if let Some(last_y) = y_last {
                    y_pairs.push((last_y+1,y-1));
                }
                y_last = Some(y);
                y_pairs.push((y,y));
            }
            for &z in &z_vals {
                if let Some(last_z) = z_last {
                    z_pairs.push((last_z+1,z-1));
                }
                z_last = Some(z);
                z_pairs.push((z,z));
            }

            x_pairs.retain(|&(l,u)|l<=u);
            y_pairs.retain(|&(l,u)|l<=u);
            z_pairs.retain(|&(l,u)|l<=u);

            //x_pairs = (*x_vals.iter().min().unwrap()..=*x_vals.iter().max().unwrap()).map(|i|(i,i)).collect();
            //y_pairs = (*y_vals.iter().min().unwrap()..=*y_vals.iter().max().unwrap()).map(|i|(i,i)).collect();
            //z_pairs = (*z_vals.iter().min().unwrap()..=*z_vals.iter().max().unwrap()).map(|i|(i,i)).collect();

            let mut pos_vec: Vec<((Vec3,Vec3),bool)> = Vec::new();
            for x in &x_pairs {
                for y in &y_pairs {
                    for z in &z_pairs {
                        pos_vec.push((((x.0,y.0,z.0),(x.1,y.1,z.1)),false));
                    }
                }
            }

            for (grid_pos, grid_on) in &mut pos_vec {
                if  range_overlap(range, grid_pos) {
                    *grid_on = true;
                }
            }

            for pos in &overlap_vec {
                for (grid_pos, grid_on) in &mut pos_vec {
                    if range_overlap(pos, grid_pos) {
                        *grid_on = false;
                    }
                }
            }

            pos_vec.iter().filter_map(|((a,b),on)|{
                if *on {
                    let dx = (b.0-a.0).abs()+1;
                    let dy = (b.1-a.1).abs()+1;
                    let dz = (b.2-a.2).abs()+1;
                    let amt = dx*dy*dz;
                    Some(amt)
                } else {
                    None
                }
            }).sum()
        }
        1 => {
            let mut piece_vec: Vec<(Vec3,Vec3)> = vec![*range];
            for overlap in &overlap_vec {
                let mut new_piece_vec: Vec<(Vec3,Vec3)> = Vec::new();
                for piece in &piece_vec {
                    if range_overlap(piece, overlap) {
                        let candidates = range_union_intersect_regions(piece,overlap);
                        for p in candidates.iter().filter(|&p| {
                            !range_overlap(overlap, p)
                        }) {
                            new_piece_vec.push(*p);
                        }
                    } else {
                        new_piece_vec.push(*piece);
                    }
                }
                piece_vec = new_piece_vec;

                /*
                let mut piece_not_overlapping: Vec<(Vec3, Vec3)> = Vec::new();
                'l:
                loop {
                    let mut to_be_added: Vec<(Vec3, Vec3)> = Vec::new();
                    let mut to_be_removed: Vec<(Vec3, Vec3)> = Vec::new();
                    for piece in piece_vec.iter().filter(|&p|range_overlap(overlap,p)) {
                        if (overlap.0.0 < piece.1.0) && (piece.0.0 < overlap.0.0) {
                            to_be_removed.push(*piece);
                            to_be_added.push((piece.0,(overlap.0.0-1,piece.1.1,piece.1.2)));
                            to_be_added.push(((overlap.0.0, piece.0.1, piece.0.2),piece.1));
                        } else if (overlap.0.1 < piece.1.1) && (piece.0.1 < overlap.0.1) {
                            to_be_removed.push(*piece);
                            to_be_added.push((piece.0,(piece.1.0,overlap.0.1-1,piece.1.2)));
                            to_be_added.push(((piece.0.0, overlap.0.1, piece.0.2),piece.1));
                        } else if (overlap.0.2 < piece.1.2) && (piece.0.2 < overlap.0.2) {
                            to_be_removed.push(*piece);
                            to_be_added.push((piece.0,(piece.1.0,piece.1.1,overlap.0.2-1)));
                            to_be_added.push(((piece.0.0, piece.0.1,overlap.0.2),piece.1));
                        } else if (piece.0.0 < overlap.1.0) && (overlap.1.0 < piece.1.0) {
                            to_be_removed.push(*piece);
                            to_be_added.push(((overlap.1.0+1,piece.0.1,piece.0.2),piece.1));
                            to_be_added.push((piece.0,(overlap.1.0, piece.1.1, piece.1.2)));
                        } else if (piece.0.1 < overlap.1.1) && (overlap.1.1 < piece.1.1) {
                            to_be_removed.push(*piece);
                            to_be_added.push(((piece.0.0,overlap.1.1+1,piece.0.2),piece.1));
                            to_be_added.push((piece.0,(piece.1.0, overlap.1.1, piece.1.2)));
                        } else if (piece.0.2 < overlap.1.2) && (overlap.1.2 < piece.1.2) {
                            to_be_removed.push(*piece);
                            to_be_added.push(((piece.0.0,piece.0.1,overlap.1.2+1),piece.1));
                            to_be_added.push((piece.0,(piece.1.0, piece.1.1, overlap.1.2)));
                        } else {
                            to_be_removed.push(*piece);
                            break 'l;
                        }
                    }
                    if to_be_removed.is_empty() {
                        break 'l;
                    }
                    piece_vec.retain(|p|!to_be_removed.contains(p));
                    to_be_added.sort();
                    to_be_added.dedup();
                    to_be_added.retain(range_valid);
                    for p in to_be_added {
                        if range_overlap(overlap, &p) {

                            piece_vec.push(p);
                        } else {
                            piece_not_overlapping.push(p);
                        }
                    }
                }
                for p in piece_not_overlapping {
                    piece_vec.push(p);
                }*/
            }

            piece_vec.sort();
            piece_vec.dedup();
            piece_vec.retain(|p|!overlap_vec.iter().any(|q|*p == *q));

            let tbr: Vec<(Vec3,Vec3)> = piece_vec.iter().filter_map(|p|{
                if !piece_vec.iter().any(|o|{
                    range_intersect(p,o) == *p
                }) {
                    Some(*p)
                } else {
                    None
                }
            }).collect();
            piece_vec.retain(|p|!tbr.contains(p));

            for (i,p) in piece_vec.iter().enumerate() {
                for (j,q) in piece_vec.iter().enumerate() {
                    if i != j {
                        if range_overlap(p, q) {
                            print!("");
                        }
                    }
                }
            }
            for (_i,p) in piece_vec.iter().enumerate() {
                for (_j,q) in overlap_vec.iter().enumerate() {
                    if range_overlap(p, q) {
                        print!("");
                    }
                }
            }

            piece_vec.retain(|p|{
                !overlap_vec.iter().any(|o|{
                    range_intersect(p,o) == *p
                })
            });
            piece_vec.retain(|p|{
                !overlap_vec.iter().any(|o|{
                    if range_overlap(p,o) {
                        print!("");
                        true
                    } else {
                        false
                    }
                })
            });

            piece_vec.iter().filter_map(|(a,b)|{
                let dx = (b.0-a.0)+1;
                let dy = (b.1-a.1)+1;
                let dz = (b.2-a.2)+1;
                let amt = dx*dy*dz;
                Some(amt)
            }).sum()
        }
        2 => {
            let size = range_size(range);
            let volume = range_volume_dedup(&overlap_vec.iter().map(|o| {
                let ((lx,ly,lz),(ux,uy,uz)) = range_intersect(range,o);
                ((lx,ly,lz),(ux+1,uy+1,uz+1))
            }).collect());
            size - volume
        }
        _ => unreachable!()
    }
}

fn range_valid(range: &(Vec3, Vec3)) -> bool {
    range.0.0 < range.1.0 &&
    range.0.1 < range.1.1 &&
    range.0.2 < range.1.2
}

fn range_intersect(r1: &(Vec3, Vec3), r2: &(Vec3, Vec3)) -> (Vec3, Vec3) {
    ((
        r1.0.0.max(r2.0.0),
        r1.0.1.max(r2.0.1),
        r1.0.2.max(r2.0.2)
     ),
     (
         r1.1.0.min(r2.1.0),
         r1.1.1.min(r2.1.1),
         r1.1.2.min(r2.1.2)
     )
    )
}

fn range_size(range: &(Vec3, Vec3)) -> i64 {
    let dx = (range.1.0-range.0.0).abs()+1;
    let dy = (range.1.1-range.0.1).abs()+1;
    let dz = (range.1.2-range.0.2).abs()+1;
    let amt = dx*dy*dz;
    amt
}

fn range_size_other(range: &(Vec3, Vec3)) -> i64 {
    let dx = (range.1.0-range.0.0).abs();
    let dy = (range.1.1-range.0.1).abs();
    let dz = (range.1.2-range.0.2).abs();
    let amt = dx*dy*dz;
    amt
}

fn range_volume_dedup(ranges: &Vec<(Vec3,Vec3)>) -> i64 {
    if ranges.is_empty() {
        return 0;
    } else if ranges.len() == 1 {
        return range_size_other(ranges.get(0).unwrap());
    }
    let candidate = ranges.get(0).unwrap();
    let mut others: Vec<(Vec3,Vec3)> = (1..ranges.len()).map(|i|*ranges.get(i).unwrap()).collect();
    let positive = range_volume_dedup(&others);
    for t in &mut others {
        *t = range_intersect(candidate,t);
    }
    others.retain(range_valid);
    let negative = range_volume_dedup(&others);
    range_size_other(candidate) - negative + positive
}

fn range_union_intersect_regions(r1: &(Vec3,Vec3), r2: &(Vec3,Vec3)) -> Vec<(Vec3,Vec3)> {
    let mut x_vals = vec![r1.0.0,r1.1.0,r2.0.0,r2.1.0];
    let mut y_vals = vec![r1.0.1,r1.1.1,r2.0.1,r2.1.1];
    let mut z_vals = vec![r1.0.2,r1.1.2,r2.0.2,r2.1.2];
    x_vals.sort();
    y_vals.sort();
    z_vals.sort();

    let mut x_pairs: Vec<(i64,i64)> = Vec::new();
    let mut y_pairs: Vec<(i64,i64)> = Vec::new();
    let mut z_pairs: Vec<(i64,i64)> = Vec::new();

    let mut x_last: Option<i64> = None;
    let mut y_last: Option<i64> = None;
    let mut z_last: Option<i64> = None;

    for &x in &x_vals {
        if let Some(last_x) = x_last {
            x_pairs.push((last_x,x-1));
        }
        x_last = Some(x);
        //x_pairs.push((x,x));
    }
    for &y in &y_vals {
        if let Some(last_y) = y_last {
            y_pairs.push((last_y+1,y-1));
        }
        y_last = Some(y);
        y_pairs.push((y,y));
    }
    for &z in &z_vals {
        if let Some(last_z) = z_last {
            z_pairs.push((last_z+1,z-1));
        }
        z_last = Some(z);
        z_pairs.push((z,z));
    }

    x_pairs = vec![(*x_vals.get(0).unwrap(),*x_vals.get(1).unwrap()-1),(*x_vals.get(1).unwrap(),*x_vals.get(2).unwrap()),(*x_vals.get(2).unwrap()+1,*x_vals.get(3).unwrap())];
    y_pairs = vec![(*y_vals.get(0).unwrap(),*y_vals.get(1).unwrap()-1),(*y_vals.get(1).unwrap(),*y_vals.get(2).unwrap()),(*y_vals.get(2).unwrap()+1,*y_vals.get(3).unwrap())];
    z_pairs = vec![(*z_vals.get(0).unwrap(),*z_vals.get(1).unwrap()-1),(*z_vals.get(1).unwrap(),*z_vals.get(2).unwrap()),(*z_vals.get(2).unwrap()+1,*z_vals.get(3).unwrap())];

    x_pairs.retain(|&(l,u)|l<=u);
    y_pairs.retain(|&(l,u)|l<=u);
    z_pairs.retain(|&(l,u)|l<=u);

    //x_pairs = (*x_vals.iter().min().unwrap()..=*x_vals.iter().max().unwrap()).map(|i|(i,i)).collect();
    //y_pairs = (*y_vals.iter().min().unwrap()..=*y_vals.iter().max().unwrap()).map(|i|(i,i)).collect();
    //z_pairs = (*z_vals.iter().min().unwrap()..=*z_vals.iter().max().unwrap()).map(|i|(i,i)).collect();

    let mut pos_vec: Vec<(Vec3,Vec3)> = Vec::new();
    for x in &x_pairs {
        for y in &y_pairs {
            for z in &z_pairs {
                pos_vec.push(((x.0,y.0,z.0),(x.1,y.1,z.1)));
            }
        }
    }

    pos_vec.retain(|p|{
        range_overlap(r1,p) || range_overlap(r2,p)
    });

    pos_vec
}

#[cfg(test)]
mod test {
    use crate::{range_outside, range_volume_dedup};

    #[test]
    fn test_range_outside() {
        assert_eq!(2,range_outside(&((1,0,0),(3,0,0)),&vec![((0,0,0),(1,0,0))]));
        assert_eq!(1,range_outside(&((1,0,0),(3,0,0)),&vec![((0,0,0),(1,0,0)),((3,0,0),(4,0,0))]));
        assert_eq!(1,range_outside(&((1,0,0),(3,0,0)),&vec![((0,0,0),(1,0,0)),((3,0,0),(4,0,0)),((3,0,0),(4,1,0))]));
    }

    #[test]
    fn test_volume_dedup() {
        assert_eq!(3,range_volume_dedup(&vec![((1,0,0),(3,1,1)),((2,0,0),(4,1,1))]))
    }
}