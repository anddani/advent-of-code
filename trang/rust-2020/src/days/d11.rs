use std::collections::HashMap;

fn run_cycle(mut coords: HashMap<(i32,i32), usize>) -> HashMap<(i32,i32), usize> {
    let mut n_change: usize = 0;    
    let old_coords = coords.clone();
    for (c, &state) in &old_coords {
        let mut n_occupied: usize = 0;
        for x1 in -1..2 {
            for y1 in -1..2 {
                if (x1,y1) != (0,0) {
                    let neighbour = &(c.0+x1, c.1+y1);
                    n_occupied += old_coords.get(neighbour).cloned().unwrap_or(0);
                }
            }
        }
        if n_occupied ==0 && state == 0 {
            coords.insert(*c, 1);
            n_change += 1;
        } 
        if n_occupied >= 4 && state == 1 {
            coords.insert(*c, 0);
            n_change += 1;
        }
    }
    return coords
}

pub fn run() {
    let f = std::fs::read_to_string("./data/input_d11.txt").unwrap();
    let mut coords: HashMap<(i32,i32), usize> = HashMap::new();
    for (x, l) in f.lines().enumerate() {
        for (y, c) in l.chars().enumerate() {
            if c=='L' { coords.insert((x as i32,y as i32), 0); }
        }
    }
    println!("Number of seats: {}", coords.len());

    let mut n_change: usize = 0;
    let mut old_coords = coords.clone();
    coords = run_cycle(coords);
    while old_coords != coords {
        old_coords = coords.clone();
        coords = run_cycle(coords);
    }
    println!("Stabilized! Number of occupied seats: {}", coords.values().filter(|&n| *n == 1).count());

}