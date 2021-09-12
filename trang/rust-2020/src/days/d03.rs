use std::io::{BufRead, BufReader};
use std::fs::File;

const STEP: usize = 3;

pub fn run() {
    let f = BufReader::new(File::open("./data/input_d03.txt").unwrap());
    //let mut iter = f.iter().peekable();

    let mut slope_31 = 0;
    let mut c: usize = 3;

    for l in f.lines().skip(1) {
        let l = l.unwrap();
        println!("current line {}, length {}", l,l.len());
        let pos = l.chars().nth(c).unwrap();
        println!("current: {}, position {}", c, pos);
        if pos == '#' {
            slope_31 +=1;
        }
        c = (c + STEP) % l.len();
    }
    println!("Number of trees encountered: {}", slope_31);
}