use std::io::{BufReader, BufRead};
use std::fs::File;

const SUM: usize = 2020;

pub fn run() {
    let f = BufReader::new(File::open("./data/input_d01.txt").unwrap());
    
    let mut vec: Vec<usize> = vec![];
    let mut num_1 = 0;
    let mut num_2 = 0;

    for l in f.lines() {
        let num: usize = l.unwrap().parse().unwrap();
        vec.push(num);
    }
    for i in 0..vec.len() {
        let n1 = vec[i];
        for n2 in &vec[i+1..] {
            if n1 + n2 == SUM {
                num_1 = n1 * n2;
                break;
            }
            else {
                if SUM > (n1+n2) {
                    let n3 = SUM - (n1 + n2); 
                    if vec.contains(&n3) {
                        num_2 = n1 * n2 * n3;
                    }    
                }
            }
        }

        if (num_1 != 0) & (num_2 != 0) {
            break;
        }

    }

    println!("Part 1: {}", num_1);
    println!("Part 2: {}", num_2);
}
