use std::time::Instant;
use std::collections::HashMap;

pub fn play_n_turn(start: &Vec<usize>, n :usize) -> usize {
    let mut mem = start.to_vec();
    for i in start.len()..n {
        let &last = mem.last().unwrap();
        //println!("{} {} {:?}", i, last, &mem);
        match mem.iter().rev().skip(1).position(|&v| v == last) {
            Some(pos) => {
                mem.push(i - (mem.len() - (pos + 1)))
            },
            None => mem.push(0),
        };
    }
    return *mem.last().unwrap()
}

fn play_n_turn_hash(start: &Vec<usize>, n :usize) -> usize {
    //let mut mem2 = start.to_vec();
    let mut mem: HashMap<usize,usize> = HashMap::new();
    let &(mut last) = start.last().unwrap();
    start.iter().rev().skip(1).rev().enumerate().for_each(|(i, &v)| {mem.insert(v,i);});
    for i in (start.len()-1)..n-1 {
        match mem.get(&last) {
            None => {
                //println!("turn {}, new key {}, next number {}", i+1, last, 0);
                mem.insert(last, i);
                last = 0;
                //mem2.push(0);
            },
            Some(v) => {
                //println!("turn {} last number {} at index last {}, next number {} ", i+1, last, v, i - v );
                let &previous_pos = v;
                mem.insert(last, i);  
                //mem2.push(i - previous_pos);
                last = i - previous_pos;
            },
        }
    }
    //println!("{:?}", mem2);

    return last
}
pub fn run() {
    let time = Instant::now();
    let inputs: Vec<usize> = std::fs::read_to_string("./data/input_d15.txt").unwrap()
        .trim() //trim String: rm whitespace
        .split(",")
        .filter_map(|v| v.parse().ok())
        .collect();
    println!("{:?}",inputs);
    //println!("2020th turn: {:?}", play_n_turn(&inputs, 2020));
    println!("2020th turn: {:?}", play_n_turn_hash(&inputs, 2020));
    println!("30000000th turn {:?}", play_n_turn_hash(&inputs, 30000000));
    
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Elapsed: {:.3} ms", elapsed_ms);
}