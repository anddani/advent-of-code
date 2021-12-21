//use std::collections::HashMap;
use itertools::Itertools;

fn compute_fuel(all_crabs: Vec<usize>, k: usize) -> usize {
    let all_crabs_not_k = all_crabs.iter()
                                .filter(|&n| *n != k)
                                .map(|&n| (n as i32 - k as i32).abs() as usize)
                                .collect::<Vec<usize>>();
    let fuel = all_crabs_not_k.iter().sum();
    return fuel
}

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let v = f.lines()
            .collect::<Vec<&str>>()
            .into_iter()
            .flat_map(|l| l.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>())
            .collect::<Vec<usize>>();
    println!("Number of crabs {}", v.len());

    let mut v_unique = v.clone().into_iter().unique().collect::<Vec<usize>>();
    println!("Unique values {:?}", v_unique.len());
    
    v_unique.sort();
    let mut least_fuel: usize = 1000000000000;
    println!("max value {}", least_fuel);
    for position in v_unique.iter() {
        let fuel = compute_fuel(v.clone(), *position);
        println!("Fuel to move all crabs to {}: {}",position,fuel);
        if fuel < least_fuel {
            least_fuel = fuel;
        }
    }
    println!("Least fuel {}",least_fuel);
}
