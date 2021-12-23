//use regex::Regex;
use std::collections::HashMap;
fn step(template: &Vec<char>, rules: &HashMap<(char, char), char>) -> Vec<char>{
    let mut new_template: Vec<char> = Vec::new();
    for w in template.windows(2) {
        new_template.push(w[0]);
        if rules.contains_key(&(w[0], w[1])) {
            new_template.push(*rules.get(&(w[0], w[1])).clone().unwrap());
        }
    }
    new_template.push(*template.last().unwrap());
    //println!("Current template: {:?}", new_template);
    return new_template
}

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();

    let template: Vec<char> = f.lines().nth(0).unwrap().chars().collect::<Vec<char>>();
    //println!("Starting template: {:?}", template);

    let mut rules: HashMap<(char, char), char> = HashMap::new();
    f.lines()
        .skip(2)
        .for_each(|l| {
            let (pattern, insertion) = l.split_once(" -> ").unwrap();
            rules.insert(
                (pattern.chars().collect::<Vec<char>>()[0],pattern.chars().collect::<Vec<char>>()[1]), 
                insertion.chars().collect::<Vec<char>>()[0]);
        });
    //println!("{:?}", rules);
    
    let mut developing_template = template.clone();
    for _i in 1..=10 {
        developing_template = step(&developing_template, &rules);
    }
    //let line: String = developing_template.into_iter().collect();
    //println!("Current template: {}", line);

    let mut uniques: HashMap<char, usize> = HashMap::new();
    developing_template.iter().for_each(|e| {
        *uniques.entry(*e).or_insert(0) +=1;
    });
    println!("{} Unique elements: {:?}", uniques.len(), uniques);

    let mut values = uniques.values().cloned().collect::<Vec<usize>>();
    values.sort();
    println!("Score: {}", values[values.len()-1] - values[0])
}
