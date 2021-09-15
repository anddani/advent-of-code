use std::fs::File;
use std::io::{BufRead,BufReader};

/*
fn chartobool(c: char) -> bool {
    // Function not used for this solution
    match c {
        'B' | 'R' => true,
        'F' | 'L' => false,
        _ => panic!("Wrong format"),
    }
}
*/

pub fn run() {
    let a = 1;
    let b = 1;
    println!("Left shift {}", a<<b);
    let file = BufReader::new(File::open("./data/input_d05.txt").unwrap());
    for l in file.lines() {
        let l = l.unwrap();
        let mut id = 1; 
        for (_i, half) in l.chars().enumerate() {
            let character = matches!(half, 'B' | 'R') as u16;
            if character == 1 {
                id += id<<1; // Try fancy bitwise operation, << is Left Shift. This equals id*2
            }
            //println!("{}, {}", half, character);
        }
        println!("{}, {}", l, id);
    }

    let file = BufReader::new(File::open("./data/input_d05.txt").unwrap());
    /* seat ID = row*8 + col 
    iter through lines, map result into a filter map
    The fold method takes an initial value (0 in here) and a closure with two arguments: an ‘accumulator’, and an element
    acc + 1 if half is B or R
    */
    let res = file.lines()
    .filter_map(Result::ok) //pass through a sequence of Results and yield only the “successful” values
    .map(|l| {
        l.chars().fold(0, |acc, half| {
            (acc << 1) | matches!(half, 'B' | 'R') as u16 //u16: integer 0 to 2^16-1
        })
    })
    .max()
    .expect("Not empty");
    println!("Highest seat ID: {}", res);
}