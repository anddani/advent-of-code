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

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let v = f.lines()
            .collect::<Vec<&str>>()
            .into_iter()
            .flat_map(|l| l.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>())
            .collect::<Vec<usize>>();
    println!("Number of fish in d0 {}", v.len());
    let mut current_population = v.clone();
    for _day in 0..80 {
        current_population = stimulate_1day(&current_population)
    }
    println!("Number of fish in d80 {}", current_population.len());
}
