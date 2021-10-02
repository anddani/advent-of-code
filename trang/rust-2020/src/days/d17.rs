use std::collections::HashMap;

fn run_cycle(mut coords: HashMap<(i64,i64,i64,i64), usize>, dim: usize) -> HashMap<(i64,i64,i64,i64), usize> {
    let old_coords = coords.clone();
    for (c, &state) in &old_coords {
        let mut a_n = 0;
        for x1 in -1..2 { 
            for y1 in -1..2 {
                for z1 in -1..2 {
                    if dim == 3 {
                        if (x1,y1,z1) != (0,0,0) {
                            let neighbour = &(c.0+x1, c.1+y1, c.2+z1, 0);
                            //println!("{:?}, {:?}", neighbour,old_coords.get(neighbour).cloned().unwrap_or(0));
                            a_n += old_coords.get(neighbour).cloned().unwrap_or(0);
                        }
                    } else {
                        for w1 in -1..2 {
                            if (x1,y1,z1,w1) != (0,0,0,0) {
                                let neighbour = &(c.0+x1, c.1+y1, c.2+z1, c.3+w1);
                                //println!("{:?}, {:?}", neighbour,old_coords.get(neighbour).cloned().unwrap_or(0));
                                a_n += old_coords.get(neighbour).cloned().unwrap_or(0);
                            }
                        }
                    }
                }
            }
        }
        //println!("{:?}:{}, neighbors: {} active {}", (x,y,z), state, i, a_n);
        if a_n ==2 && state == 1 {
            coords.insert(*c, 1);
        } else { coords.insert(*c, 0); }
        if a_n==3 {
            coords.insert(*c, 1);
        }
    }
    let n_active: usize = coords.values().cloned().collect::<Vec<usize>>().iter().sum();
    println!("Number of active cubes after this cycle: {}", n_active);
    return coords
}

fn expand(mut coords: HashMap<(i64,i64,i64,i64), usize>, dim: usize) -> HashMap<(i64,i64,i64,i64), usize> {
    let old_coords = coords.clone();
    for (c, _) in &old_coords {
        for x1 in -1..2 { 
            for y1 in -1..2 {
                for z1 in -1..2 {
                    if dim == 3 {
                        let neighbour = &(c.0+x1, c.1+y1, c.2+z1, 0);                    
                        if !coords.contains_key(neighbour) { coords.insert(*neighbour, 0); }
                    } else {
                        for w1 in -1..2 {
                            let neighbour = &(c.0+x1, c.1+y1, c.2+z1, c.3+w1);
                            if !coords.contains_key(neighbour) { coords.insert(*neighbour, 0); }
                        }
                    }
                }
            }
        }
    }
    return coords;
}

pub fn run() {
    let f = std::fs::read_to_string("./data/input_d17.txt").unwrap();
    //let mut coords = f.lines().map(|line| line.chars().position(|c| c == '#').unwrap()).collect();

    let mut coords1: HashMap<(i64,i64,i64,i64), usize> = HashMap::new();
    let mut coords2: HashMap<(i64,i64,i64,i64), usize> = HashMap::new();
    for (x, l) in f.lines().enumerate() {
        for (y, c) in l.chars().enumerate() {
            coords1.insert((x as i64,y as i64,0, 0), if c=='#' { 1 } else { 0 });
            coords2.insert((x as i64,y as i64,0, 0), if c=='#' { 1 } else { 0 });
        }
    }
    
    let n_active: usize = coords1.values().cloned().collect::<Vec<usize>>().iter().sum();
    println!("Number of active cubes initially: {}", n_active);

    for _ in 0..6 {
        coords1 = expand(coords1, 3);
        coords1 = run_cycle(coords1, 3);
    }
    let n_active1: usize = coords1.values().cloned().collect::<Vec<usize>>().iter().sum();
    println!("Part 1: Number of active cubes after 6 cycles: {}", n_active1);

    for _ in 0..6 {
        coords2 = expand(coords2, 4);
        coords2 = run_cycle(coords2, 4);
    }
    let n_active2: usize = coords2.values().cloned().collect::<Vec<usize>>().iter().sum();
    println!("Part 2: Number of active cubes after 6 cycles: {}", n_active2);

}