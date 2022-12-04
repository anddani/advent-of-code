use std::collections::HashSet;

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let unionset: HashSet<char> = f.lines().map(|l| {
        let half = l.len() / 2;
        let p1: HashSet<char> = l.chars().take(half).collect::<HashSet<char>>();
        let p2: HashSet<char> = l.chars().rev().take(half).collect::<HashSet<char>>();
        let tmp = p1.intersection(&p2).copied().collect::<HashSet<char>>(); //.collect::<HashSet<&char>>()
        println!("overlap items = {:?}",tmp);
        tmp
    }).fold(HashSet::new(), |acc, x| acc.union(&x).copied().collect());

    let priority = unionset.iter().fold(0, |acc, x| {
        let pri: u32 = if x.is_lowercase() {(*x as u32) - 96} else {(*x as u32) - 38};
        println!("{} = {}", x, pri);
        acc + pri
    });
    
    println!("{:?}", priority);
}
