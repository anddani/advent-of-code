use std::fs::File;
use std::io::{BufRead,BufReader};
use itertools::izip;
use std::ops::Range;

pub fn run() {

    let file = BufReader::new(File::open("./data/input_d05.txt").unwrap());
    /* seat ID = row*8 + col 
    iter through lines, map result into a filter map
    The fold method takes an initial value (0 in here) and a closure with two arguments: an ‘accumulator’, and an element
    acc + 1 if half is B or R
    */
    
    let mut seats: Vec<_> = file.lines()
        .filter_map(Result::ok)
        .map(|l| {
            l.chars().fold(0, |acc, half| {
                (acc << 1) | matches!(half, 'B' | 'R') as u16
            })
        })
        .collect();
    seats.sort(); //ascending
    let max_id = seats.last().unwrap().clone();
    println!("Highest seat ID: {}", max_id);
    
    let mut myseat = 0;
    for (i, id) in izip!(Range { start: seats[0], end: max_id }, seats.iter()) {
        //seats at the front and back of the planes are not include
        //println!("{} {}", i, id);
        if i != *id {
            myseat = *id - 1;
            break;
        }
    }
    println!("First seat ID to break pattern: {}", myseat);
}