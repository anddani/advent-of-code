use std::collections::HashSet;

fn part1() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let unionset: Vec<char> = f.lines().flat_map(|l| {
        let half = l.len() / 2;
        let p1: HashSet<char> = l.chars().take(half).collect::<HashSet<char>>();
        let p2: HashSet<char> = l.chars().rev().take(half).collect::<HashSet<char>>();
        let tmp = p1.intersection(&p2).copied().collect::<HashSet<char>>(); //.collect::<HashSet<&char>>()
        println!("overlap items = {:?}",tmp);
        tmp
    }).collect(); //.fold(HashSet::new(), |acc, x| acc.union(&x).copied().collect());

    let priority = unionset.iter().fold(0, |acc, x| {
        let pri: u32 = if x.is_lowercase() {(*x as u32) - 96} else {(*x as u32) - 38};
        println!("{} = {}", x, pri);
        acc + pri
    });
    
    println!("{:?}", priority);
}

fn part2() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let lines = f.lines().collect::<Vec<_>>();
    
    let unionset: Vec<char>  = lines.chunks(3).flat_map(|chunk| {        
        let p1: HashSet<char> = chunk[0].chars().collect::<HashSet<char>>();
        let p2: HashSet<char> = chunk[1].chars().collect::<HashSet<char>>();
        let p3: HashSet<char> = chunk[2].chars().collect::<HashSet<char>>();
        let p12 = p1.intersection(&p2).copied().collect::<HashSet<char>>();
        let tmp = p12.intersection(&p3).copied().collect::<HashSet<char>>();
        tmp
    }).collect();
    
    let priority = unionset.iter().fold(0, |acc, x| {
        let pri: u32 = if x.is_lowercase() {(*x as u32) - 96} else {(*x as u32) - 38};
        println!("{} = {}", x, pri);
        acc + pri
    });
    
    println!("{:?}", priority);
}

fn main() {
    // part1();
    part2();
}