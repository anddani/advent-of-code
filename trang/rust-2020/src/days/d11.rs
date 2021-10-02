use std::collections::HashMap;

fn run_cycle(mut coords: HashMap<(i32,i32), usize>) -> HashMap<(i32,i32), usize> {    
    let old_coords = coords.clone();
    for (c, &state) in &old_coords {
        let n_occupied = look_adjacent_occupied(&old_coords, *c);
        if n_occupied ==0 && state == 0 {
            coords.insert(*c, 1);
        } 
        if n_occupied >= 4 && state == 1 {
            coords.insert(*c, 0);
        }
    }
    return coords
}

fn run_cycle_visible(mut coords: HashMap<(i32,i32), usize>) -> HashMap<(i32,i32), usize> {  
    let old_coords = coords.clone();
    for (c, &state) in &old_coords {
        let mut n_visible_occupied: usize = 0;
        n_visible_occupied += look_visible_occupied(&old_coords, *c, -1,-1);
        n_visible_occupied += look_visible_occupied(&old_coords, *c, 1,1);
        n_visible_occupied += look_visible_occupied(&old_coords, *c, -1,1);
        n_visible_occupied += look_visible_occupied(&old_coords, *c, 1,-1);
        n_visible_occupied += look_visible_occupied(&old_coords, *c, 0,-1);
        n_visible_occupied += look_visible_occupied(&old_coords, *c, 0,1);
        n_visible_occupied += look_visible_occupied(&old_coords, *c, -1,0);
        n_visible_occupied += look_visible_occupied(&old_coords, *c, 1,0);
        //println!("Point {:?}, see {} occupied", c, n_occupied);

        //let n_occupied = look_adjacent_occupied(&old_coords, *c);
        if n_visible_occupied ==0 && state == 0 {
            coords.insert(*c, 1);
        } 
        if n_visible_occupied >= 5 && state == 1 {
            coords.insert(*c, 0);
        }
    }
    return coords
}

fn look_adjacent_occupied(coords: &HashMap<(i32,i32), usize>, point:(i32,i32)) -> usize {
    let mut n_occupied: usize = 0;
    for x1 in -1..2 {
        for y1 in -1..2 {
            if (x1,y1) != (0,0) {
                let neighbour = &(point.0+x1, point.1+y1);
                n_occupied += coords.get(neighbour).cloned().unwrap_or(0);
            }
        }
    }
    return n_occupied
}

fn look_visible_occupied(coords: &HashMap<(i32,i32), usize>, point:(i32,i32) , r_n: i32, c_n: i32) -> usize {
    let mut n_visible_ocupied: usize = 0;
    let (mut current_x, mut current_y) = (point.0+r_n, point.1+c_n);
    loop {
    //while n_visible_ocupied==0 || (0..11).contains(&current_x) || (0..11).contains(&current_y) {
        let neighbour = &(current_x, current_y);
        n_visible_ocupied += match coords.get(neighbour).cloned() {
            Some(v) => if v==0 { return 0 } else { 1 } ,
            None => 0,
        };
        current_x += r_n;
        current_y += c_n;
        if n_visible_ocupied > 0 || !(0..90).contains(&current_x) || !(0..95).contains(&current_y) { 
            break; 
        }
    }
    return n_visible_ocupied
}

fn print(coords: &HashMap<(i32,i32),usize>) {
    for x in 0..10 {
        let mut line = "".to_owned();
        for y in 0..10 {
            let a: usize = coords.get(&(x, y)).cloned().unwrap_or(2);

            line.push_str(
                match a {
                    0 => "L",
                    1 => "#",
                    _ => "."
                }
            );
        }
        println!("{}",line);
    }
}

pub fn run() {
    let run_part1 = false;
    let run_part2 = true;
    let f = std::fs::read_to_string("./data/input_d11.txt").unwrap();
    let mut coords: HashMap<(i32,i32), usize> = HashMap::new();
    for (x, l) in f.lines().enumerate() {
        for (y, c) in l.chars().enumerate() {
            if c=='L' { coords.insert((x as i32,y as i32), 0); }
        }
    }
    println!("Number of seats: {}", coords.len());
    //println!("{:?}",coords.keys());
    println!("Max x {}, max y {}", f.lines().count(), f.lines().nth(0).unwrap().chars().count());
    if run_part1 {
        let mut old_coords = coords.clone();
        coords = run_cycle(coords);
        while old_coords != coords {
            old_coords = coords.clone();
            coords = run_cycle(coords);
        }
        println!("Stabilized! Number of occupied seats: {}", coords.values().filter(|&n| *n == 1).count());    
    }

    if run_part2 {
        let mut old_coords = coords.clone();
        coords = run_cycle_visible(coords);
        //println!("Number of occupied seats: {}", coords.values().filter(|&n| *n == 1).count());        
        while old_coords != coords {
            old_coords = coords.clone();
            //print(&coords);
            coords = run_cycle_visible(coords);            
            //println!("Number of occupied seats: {}", coords.values().filter(|&n| *n == 1).count());        

        }
        println!("Stabilized! Number of occupied seats: {}", coords.values().filter(|&n| *n == 1).count());    
    }
}