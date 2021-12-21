use std::collections::HashMap;

fn check_adjacent_points(height_map: &HashMap<(i32,i32), u32>, point:(i32,i32), height: u32) -> usize {
    for x1 in -1..=1 {
        for y1 in -1..=1 {
            let neighbour = &(point.0+x1, point.1+y1);
            if (x1,y1) != (0,0) && (height_map.contains_key(neighbour)) {
                if height_map.get(neighbour).cloned().unwrap() < height {
                    return 0
                }
            }
        }
    }
    return 1
}

fn add_adjacent_points(height_map: &HashMap<(i32,i32), u32>, basin_map: &HashMap<(i32,i32), u32>, point:(i32,i32)) -> HashMap<(i32,i32), u32> {
    let mut basin_map_new = basin_map.clone();
    let direction = vec![(-1,0),(1,0),(0,-1),(0,1)];
    for (x1, y1) in direction.iter() {
        let neighbour = &(point.0+x1, point.1+y1);
        if (x1,y1) != (&0,&0) && (height_map.contains_key(neighbour)) {
            let neighbour_h = height_map.get(neighbour).cloned().unwrap();
            if (neighbour_h < 9) && (!basin_map_new.contains_key(neighbour)) {
                basin_map_new.insert(*neighbour, neighbour_h);
            }
        }        
    }
    return basin_map_new
}

fn expand_basin(height_map: &HashMap<(i32,i32), u32>, point:(i32,i32)) -> usize {
    let mut basin: HashMap<(i32,i32), u32> = HashMap::new();
    basin.insert(point, height_map.get(&point).cloned().unwrap());
    let mut current_size = basin.len();
    loop {
        let mut basin_new = basin.clone();
        //println!("{:?} {}", point,basin.len());
        for (c, _h) in &basin {
            basin_new = add_adjacent_points(&height_map, &basin_new, *c); 
            //println!("{:?}", basin_new);
        }

        if basin_new.len() == current_size { 
            break; 
        } else {
            current_size = basin_new.len();
            basin = basin_new;
        }
    }
    return current_size
}


fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let mut height_map: HashMap<(i32,i32), u32> = HashMap::new();
    
    for (r, l) in f.lines().enumerate() {
        for (c, h) in l.chars().enumerate() {
            height_map.insert((r as i32, c as i32),  h.to_digit(10).unwrap());
            //println!("Current point ({},{}): {}", r, c, h.to_digit(10).unwrap());
        } 
    }

    let mut risk_level: u32 = 0;
    let mut basin_sizes: Vec<usize> = Vec::new();
    for (coord, h) in &height_map {
        if check_adjacent_points(&height_map, *coord, *h) == 1 {
            risk_level += h + 1;
            let basin_size = expand_basin(&height_map, *coord);
            basin_sizes.push(basin_size);
        }
    }
    println!("Low points risk level: {}", risk_level);
    basin_sizes.sort();
    basin_sizes.reverse();
    println!("Product of 3 largest basin sizes: {}", basin_sizes[0]*basin_sizes[1]*basin_sizes[2]);
}
