use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn run() {
    let f = BufReader::new(File::open("./data/input_d03.txt").unwrap());

    let mut slope_11 = 0; //process_slope(f, (1,1));
    let mut slope_31 = 0;
    let mut slope_51 = 0;
    let mut slope_71 = 0;
    let mut slope_12 = 0;

    for (row, l) in f.lines().skip(1).enumerate() {
        let l = l.unwrap();
        //println!("current line {}: {}, length {}", row, l,l.len());

        for (c, pos) in l.chars().enumerate() {
            if pos != '#' {continue;}
            if row*1 % l.len() == c { slope_11 += 1;}
            if row*3 % l.len() == c { slope_31 += 1;}
            if row*5 % l.len() == c { slope_51 += 1;}
            if row*7 % l.len() == c { slope_71 += 1;}
            if (row % 2 == 0) && (row/2 % l.len() == c) { slope_12 += 1;}
        } 
    }
    let mul = slope_11 * slope_31 * slope_51 * slope_71 * slope_12;

    println!("Number of trees encountered r1d1: {}", slope_11);
    println!("Number of trees encountered r3d1: {}", slope_31);
    println!("Number of trees encountered r5d1: {}", slope_51);
    println!("Number of trees encountered r7d1: {}", slope_71);
    println!("Number of trees encountered r1d2: {}", slope_12);
    println!("Multiply all slopes: {}", mul);
    
}