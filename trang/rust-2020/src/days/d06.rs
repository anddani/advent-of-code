use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;

pub fn run() {
    /*
    let f = BufReader::new(File::open("./data/input_d06.txt").unwrap());
    
    let mut batch_iter = f.split(b'\n').map(|l| l.unwrap());

    let mut counts = 0;
    for b in batch_iter.next() {
        for l in b.lines(){
            println!("line: {}", l.unwrap())
        }
        //let s = b.unwrap().split_whitespace();
        //println!("{}", b.unwrap());
        //let mut set = HashSet::new();
        //set.extend(s.chars());
        //println!("{}", set.len());
        //counts += set.len();
    }
    println!("{}", counts);
    */
    let f = BufReader::new(File::open("./data/input_d06.txt").unwrap());

    let mut counts = 0;
    let mut set = HashSet::new();
    for l in f.lines() {
        let l = l.unwrap();
        if l.is_empty() {
            counts += set.len();
            println!("Line break! Set len: {}", set.len());
            set.clear();  
        } else {
            set.extend(l.chars()); //Extends a collection with the contents of an iterator
            println!("{}",l);
        }
    }
    if set.len() > 0 {
        counts += set.len();
        println!("Last lines: Set len: {}", set.len());
        set.clear();  
    }
    println!("Count {}", counts);

    
    
}