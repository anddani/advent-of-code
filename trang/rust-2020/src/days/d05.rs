use std::fs::File;
use std::io::{BufRead,BufReader};

pub fn run() {

    let file = BufReader::new(File::open("./data/input_d05.txt").unwrap());
    /* seat ID = row*8 + col 
    iter through lines, map result into a filter map
    The fold method takes an initial value (0 in here) and a closure with two arguments: an ‘accumulator’, and an element
    acc + 1 if half is B or R
    */
    let max_id = file.lines()
    .filter_map(Result::ok) //pass through a sequence of Results and yield only the “successful” values
    .map(|l| {
        l.chars().fold(0, |acc, half| {
            (acc << 1) | matches!(half, 'B' | 'R') as u16 //u16: integer 0 to 2^16-1
        })
    })
    .max()
    .expect("Not empty");
    println!("Highest seat ID: {}", max_id);

}