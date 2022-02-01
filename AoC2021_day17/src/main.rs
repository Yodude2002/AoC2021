const STEPS: i64 = 1000;

fn main() {
    part_two();
}

fn part_one() {
    let stdin = std::io::stdin();

    let mut input: String = String::new();
    stdin.read_line(&mut input).unwrap();

    let mut range_y: (i64, i64) = (0,0);
    let mut range_x: (i64, i64) = (0,0);

    for coord_str in input.strip_prefix("target area: ").unwrap().strip_suffix('\n').unwrap().split(", ") {
        let mut coord = coord_str.split("=");
        let which = coord.next().unwrap().chars().next().unwrap();

        let mut pair = coord.next().unwrap().split("..").map(|s|s.parse::<i64>());

        let first = pair.next().unwrap().unwrap();
        let last = pair.next().unwrap().unwrap();

        match which {
            'x' => range_x = (first, last),
            'y' => range_y = (first, last),
            _ => {}
        }
    }

    let mut valid_x: Vec<(i64,Vec<i64>)> = (0..STEPS).map(|n|{
        (n, (0..=n).filter(|&num|{
            let pos = triangle(n) - triangle(n-num);
            range_x.0 <= pos && pos <= range_x.1
        }).collect::<Vec<i64>>())
    }).filter(|(n,v)|!v.is_empty()).collect::<Vec<(i64,Vec<i64>)>>();

    let mut valid_y: Vec<(i64,Vec<i64>)> = (0..(-range_y.0)).map(|n|{
        (n, (0..=n).filter(|&num|{
            let pos = triangle(n) - triangle(n + num);
            range_y.0 <= pos && pos <= range_y.1
        }).map(|num|2*n + 1 + num).collect::<Vec<i64>>())
    }).filter(|(n,v)|!v.is_empty()).collect::<Vec<(i64,Vec<i64>)>>();

    let (x, y) = valid_y.iter().filter_map(|(y,y_vec)| {
        for (x, x_vec) in &valid_x {
            if y_vec.iter().any(|n|x_vec.contains(n)) {
                return Some((*x, *y));
            } else if x_vec.contains(x) && y_vec.iter().any(|&n|n >= *x) {
                return Some((*x, *y));
            }
        }
        None
    }).max_by(|a,b| {
        a.1.cmp(&b.1)
    }).unwrap();

    println!("Found Coordinates ({},{}) with height {}",x,y,triangle(y));
}

fn part_two() {
    let stdin = std::io::stdin();

    let mut input: String = String::new();
    stdin.read_line(&mut input).unwrap();

    let mut range_y: (i64, i64) = (0,0);
    let mut range_x: (i64, i64) = (0,0);

    for coord_str in input.strip_prefix("target area: ").unwrap().strip_suffix('\n').unwrap().split(", ") {
        let mut coord = coord_str.split("=");
        let which = coord.next().unwrap().chars().next().unwrap();

        let mut pair = coord.next().unwrap().split("..").map(|s|s.parse::<i64>());

        let first = pair.next().unwrap().unwrap();
        let last = pair.next().unwrap().unwrap();

        match which {
            'x' => range_x = (first, last),
            'y' => range_y = (first, last),
            _ => {}
        }
    }

    let mut valid_x: Vec<(i64,Vec<i64>)> = (0..STEPS).map(|n|{
        (n, (0..n).filter_map(|num| {
            let pos = ((-num+n)..=(n)).sum::<i64>();

            if range_x.0 <= pos && pos <= range_x.1 {
                Some(num+1)
            } else {
                None
            }
        }).collect::<Vec<i64>>())
    }).filter(|(n,v)|!v.is_empty()).collect::<Vec<(i64,Vec<i64>)>>();

    let mut valid_y: Vec<(i64,Vec<i64>)> = ((range_y.0)..(-range_y.0)).map(|n|{
        (n, (0..STEPS).filter_map(|num|{
            let pos = ((-num+n)..=(n)).sum::<i64>();
            //let pos = triangle(n) - triangle(n + num);
            if range_y.0 <= pos && pos <= range_y.1 {
                Some(num+1)
            } else {
                None
            }
        }).collect::<Vec<i64>>())
    }).filter(|(n,v)|!v.is_empty()).collect::<Vec<(i64,Vec<i64>)>>();

    let total: u64 = valid_y.iter().map(|(y, y_vec)|{
        valid_x.iter().map(|(x,x_vec)|{
            if x_vec.contains(x) && y_vec.iter().any(|&n|n > *x) ||
                y_vec.iter().any(|n|x_vec.contains(n)){
                println!("\"{},{}\",",x,y);
                1
            } else {
                0
            }
        }).sum::<u64>()
    }).sum();

    if false {
        println!("Valid X:");
        for (x, vec) in &valid_x {
            print!("\t{} ",x);
            for (i,n) in vec.iter().enumerate() {
                print!("{}",if i == 0 {
                    "- "
                } else {
                    ", "
                });
                print!("{}",n);
            }
            println!();
        }
        println!("\nValid Y:");
        for (y, vec) in &valid_y {
            print!("\t{} ",y);
            for (i,n) in vec.iter().enumerate() {
                print!("{}",if i == 0 {
                    "- "
                } else {
                    ", "
                });
                print!("{}",n);
            }
            println!();
        }
    }

    let real_total = (0..=-(range_x.1)).map(|x|
        ((range_y.0)..(-range_y.0)).filter(|&y|{
            if (0..STEPS).any(|n|{
                let pos_x = (0.min(-n+x)..=(x)).sum::<i64>();
                let pos_y = ((-n+y)..=(y)).sum::<i64>();

                range_x.0 <= pos_x && pos_x <= range_x.1 &&
                range_y.0 <= pos_y && pos_y <= range_y.1
            }) {
                //println!("\"{},{}\",",x,y);
                true
            } else {
                false
            }
        }).count() as u64
    ).sum::<u64>();

    println!("Found Total Possible Coordinates: {}",total);
}

fn triangle(n: i64) -> i64 {
    (n*n+n)/2
}
