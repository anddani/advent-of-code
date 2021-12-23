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
    return new_template
}

fn main() {
    let run_p1 = true;
    let run_p2 = true;
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let template: Vec<char> = f.lines().nth(0).unwrap().chars().collect::<Vec<char>>();
    let mut rules: HashMap<(char, char), char> = HashMap::new();
    f.lines()
        .skip(2)
        .for_each(|l| {
            let (pattern, insertion) = l.split_once(" -> ").unwrap();
            rules.insert(
                (pattern.chars().collect::<Vec<char>>()[0],pattern.chars().collect::<Vec<char>>()[1]), 
                insertion.chars().collect::<Vec<char>>()[0]);
        });
    
    if run_p1 {
        let mut developing_template = template.clone();
        for _i in 1..=10 {
            developing_template = step(&developing_template, &rules);
        }
        
        let mut uniques: HashMap<char, usize> = HashMap::new();
        developing_template.iter().for_each(|e| {
            *uniques.entry(*e).or_insert(0) +=1;
        });
        println!("{} Unique elements: {:?}", uniques.len(), uniques);
    
        let mut values = uniques.values().cloned().collect::<Vec<usize>>();
        values.sort();
        println!("Score: {}", values[values.len()-1] - values[0])    
    }

    if run_p2 {
        let mut uniques: HashMap<char, i64> = HashMap::new();
        template.iter().for_each(|e| {
            *uniques.entry(*e).or_insert(0) +=1;
        });

        let mut pairs: HashMap<(char,char), i64> = HashMap::new();
        template.windows(2).for_each(|w| *pairs.entry((w[0],w[1])).or_insert(0) +=1);
        
        for _i in 1..=40 {
            let old_pairs = pairs.clone();
            for p in old_pairs.iter() {
                let pair_ = p.0;
                let n_pair = *p.1;

                if rules.contains_key(&(pair_.0, pair_.1)) {
                    let mid = rules.get(&(pair_.0, pair_.1)).clone().unwrap();
                    *pairs.entry((pair_.0,*mid)).or_insert(0) += n_pair;
                    *pairs.entry((*mid,pair_.1)).or_insert(0) += n_pair;
                    *pairs.get_mut(pair_).unwrap() -= n_pair;

                    *uniques.entry(*mid).or_insert(0) += n_pair;
                }
            }
        }
        println!("{} Unique elements: {:?}", uniques.len(), uniques);
        let mut values = uniques.values().cloned().collect::<Vec<i64>>();
        values.sort();
        println!("Score: {}", values[values.len()-1] - values[0])
    }

}
