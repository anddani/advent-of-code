use std::collections::HashMap;

fn stimulate_1day(population_begin: &Vec<usize>) ->  Vec<usize> {
    let mut current_population = population_begin.clone();
    let mut new_fishes: Vec<usize> = vec![];
    for (i,fish) in population_begin.iter().enumerate() {
        if *fish > 0 {
            current_population[i] -=1;
        } else {
            current_population[i] = 6;
            new_fishes.push(8);
        }
    }
    current_population.append(&mut new_fishes);
    return current_population
}


fn stimulate_1day_fast(population_begin: &HashMap<usize, usize>) ->  HashMap<usize, usize> {
    // Some fancy bitshift / bit rotate stuffs!
    // Or Hashmap
    let mut current_population: HashMap<usize, usize> = population_begin.clone(); // days, n_fishes

    // First handle normal fishes
    for (internal_day, n_fish) in population_begin.iter() {
        if (1..=8).contains(&*internal_day) {
            current_population.insert(*internal_day-1, *n_fish);
        }
    }
    // Then handle newborn fishes
    *current_population.entry(6).or_insert(0) += population_begin.get(&0).unwrap();
    current_population.insert(8, *population_begin.get(&0).unwrap_or(&0));
    
    return current_population
}

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let v = f.lines()
            .collect::<Vec<&str>>()
            .into_iter()
            .flat_map(|l| l.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>())
            .collect::<Vec<usize>>();
    println!("Number of fish in d0 {}", v.len());
    let mut current_population = v.clone();
    for _day in 1..=80 {
        current_population = stimulate_1day(&current_population);
    }
    println!("Number of fish in d79 {}", current_population.len());
    
    let mut current_population2: HashMap<usize, usize> = HashMap::new();
    for internal_d in 0..=8 {
        let count = v.iter().filter(|&x| *x == internal_d).count();
        current_population2.insert(internal_d, count);
    }
    for _day in 1..=256 {
        current_population2 = stimulate_1day_fast(&current_population2);
    }
    let sum: usize = current_population2.values().cloned().collect::<Vec<usize>>().iter().sum();
    println!("Number of fish in d256 {:?}", sum);
}
