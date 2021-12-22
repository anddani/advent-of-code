use std::collections::HashMap;

fn flash(energy_map: &HashMap<(i32,i32), u32>, point: (i32, i32)) -> HashMap<(i32,i32), u32> {
    // Function to change energy level of all adjacent octopuses of an octopus with level > 9
    let mut energy_map_new = energy_map.clone();
    energy_map_new.insert(point, 0);
    for x1 in -1..=1 {
        for y1 in -1..=1 {
            let neighbour = &(point.0+x1, point.1+y1);
            if (x1,y1) != (0,0) && (energy_map.contains_key(neighbour)) {
                let current_el = energy_map.get(&neighbour).clone().unwrap();
                // each otopus only flashes once
                if *current_el != 0 {
                    energy_map_new.insert(*neighbour, current_el + 1);
                }
            }
        }
    }
    return energy_map_new
}

fn run_1_step(energy_map :&HashMap<(i32,i32), u32>) -> (HashMap<(i32,i32), u32>, usize) {
    let mut energy_map_new = energy_map.clone();
    let mut n_flash = 0;
    for (coord, el) in energy_map.iter() {
        // First, the energy level of each octopus increases by 1.
        let el_new = el +1;
        energy_map_new.insert(*coord,el_new);
    }
    //println!("Number of zeroes at step 1 is {}", count_value(&energy_map_new,0));

    //let mut energy_map_previous = energy_map_new.clone();
    let mut energy_map_new2 = energy_map_new.clone();
    loop {
        // any octopus with an energy level greater than 9 flashes
        //If this causes an octopus to have an energy level greater than 9, it also flashes.
        for (coord_n, el_n) in energy_map_new.iter() {        
            if *el_n > 9 {
                energy_map_new2 = flash(&energy_map_new2, *coord_n);
                n_flash +=1;            
            }
        }
        energy_map_new = energy_map_new2.clone();
        let n0 = energy_map_new.values().clone().filter(|&v| *v > 9).count();
        if n0 == 0 {
            break
        }
    }    

    //println!("current map {:?}", energy_map_new2);
    return (energy_map_new, n_flash)
}

fn count_value(energy_map :&HashMap<(i32,i32), u32>, value: u32)-> usize{
    let n0 = energy_map.values().clone().filter(|&v| *v == value).count();
    return n0
}

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let mut energy_map0: HashMap<(i32,i32), u32> = HashMap::new();
    
    for (r, l) in f.lines().enumerate() {
        for (c, h) in l.chars().enumerate() {
            energy_map0.insert((r as i32, c as i32),  h.to_digit(10).unwrap());
            //println!("Current point ({},{}): {}", r, c, h.to_digit(10).unwrap());
        } 
    }

    //Part 1
    let mut energy_map = energy_map0.clone();
    let mut flash_count: usize = 0;
    for _d in 1..=100 {
        let (energy_map_new, n_flash) = run_1_step(&energy_map);
        flash_count += n_flash;
        energy_map = energy_map_new;
        //println!("Number of zeroes at day {} is {}", _d, count_value(&energy_map,0));
        
    }
    println!("Number of flashes after 100 rounds {}", flash_count);

}
