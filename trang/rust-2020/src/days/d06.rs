use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;
use std::time::Instant;

fn count_union(group: Vec<&str>) -> usize {
    let mut set: HashSet<char> = HashSet::new();
    for item in group.iter() {
        set.extend(item.chars());
    }
    return set.len()
}

fn count_intersection(group: Vec<&str>) -> usize {
    let mut set: HashSet<char> = HashSet::new();
    for (i, item) in group.iter().enumerate() {
        if i == 0 {
            set.extend(item.chars());
        } else {
            let mut current = HashSet::new();
            current.extend(item.chars());
            set.retain(|&k| current.contains(&&k));    
        }
    }
    return set.len()
}

pub fn run() {    
    // Way 2
    let time = Instant::now();
    let stdin = std::fs::read_to_string("./data/input_d06.txt").unwrap();

    let counts1: usize = stdin.split("\n\n")
        .map(|x| {
            let strings: Vec<&str> = x.split_whitespace().collect();
            count_union(strings)
        })
        .sum();
    println!("{}", counts1);

    let counts2: usize = stdin.split("\n\n")
        .map(|x| {
            let strings: Vec<&str> = x.split_whitespace().collect();
            count_intersection(strings)
        })
        .sum();
    println!("{}", counts2);

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Elapsed: {:.3} ms", elapsed_ms);
    
    // Way 2
    let time = Instant::now();
    let f = BufReader::new(File::open("./data/input_d06.txt").unwrap());

    let mut counts = 0;
    let mut counts2 = 0;
    let mut set = HashSet::new();
    let mut set2 = HashSet::new();
    let mut start_chunk = true;
    for l in f.lines() {
        let l = l.unwrap();
        if l.is_empty() {
            counts += set.len();
            //println!("Line break! Set len: {}", set.len());
            set.clear();  
            
            counts2 += set2.len();
            set2.clear();
            start_chunk = true;
        } else {
            set.extend(l.chars()); //Extends a collection with the contents of an iterator
            //println!("{}",l);
            if start_chunk {
                set2.extend(l.chars());
                start_chunk = false;
            } else {
                let mut current = HashSet::new();
                current.extend(l.chars());
                set2.retain(|&k| current.contains(&&k));
            }
        }
    }
    if set.len() > 0 {
        counts += set.len();
        //println!("Last lines: Set1 len: {}", set.len());
        set.clear();  
    }
    if set2.len() > 0 {
        counts2 += set2.len();
        //println!("Last lines: Set2 len: {}", set2.len());
        set2.clear();  
    }
    println!("Count union {}", counts);
    println!("Count intersection {}", counts2);
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Elapsed: {:.3} ms", elapsed_ms);
}