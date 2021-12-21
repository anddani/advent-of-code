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
    for (coord, h) in &height_map {
        if check_adjacent_points(&height_map, *coord, *h) == 1 {
            //println!("Found low point {}", h);
            risk_level += h + 1;
        }
    }
    println!("Low points risk level: {}", risk_level);

}
