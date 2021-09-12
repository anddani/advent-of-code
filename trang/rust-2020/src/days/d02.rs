use std::io::{BufReader, BufRead};
use std::fs::File;
use std::convert::TryInto;
use regex::Regex;

pub fn run() {
    let f = BufReader::new(File::open("./data/input_d02.txt").unwrap());
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    let mut count_1 = 0;
    let mut count_2 = 0;

    for l in f.lines() {
        let chars = l.unwrap().to_string();
        let caps = re.captures(&chars).unwrap();

        let low: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let high: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let letter = caps.get(3).unwrap().as_str().chars().next().unwrap();
        let chr = caps.get(4).unwrap().as_str();

        println!("low: {}, high: {}, letter: {}, word: {}", low, high, letter, chr);

        let c: i32 = chr.matches(letter).count().try_into().unwrap();
        //println!("match count: {}", c);

        let range = low..high+1;
        if range.contains(&c) {
            count_1 += 1; 
        }

        let pos1 = chr.chars().nth((low-1) as usize).unwrap();
        let pos2 = chr.chars().nth((high-1) as usize).unwrap();
        println!("positions: {} {}", pos1, pos2);
        
        if (pos1 == letter) ^ (pos2 == letter) {
            count_2 += 1;
            println!("Updated: {}", count_2)
        }
    
    }
    println!("Number of valid passwords: {}", count_1);
    println!("Number of valid passwords with new policy: {}", count_2);
}