mod days;

use days::{d01, d02, d03, d05, d06, d07, d08};
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
            2 => d02::run,
            3 => d03::run,
            //4 => d04::run,
            5 => d05::run,
            6 => d06::run,
            7 => d07::run,
            8 => d08::run,
            _ => panic!("Not yet implemented!"),
        };

        println!("\n=== AOC day {}", d);
        func()
    }
}
