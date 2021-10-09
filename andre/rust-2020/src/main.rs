mod days;

use days::{d01};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the day in the right format:");
    }

    let days: Vec<u8> = args.iter()
        .skip(1)
        .map(|x| x.parse().unwrap_or_else(|y| panic!("Not a valid day: {}", y)))
        .collect();

    for d in days{
        let func = match d {
            1 => d01::run,
            _ => panic!("Not yet implemented!"),
        };

        println!("\n=== AOC 2020 day {}", d);
        func()
    }
}