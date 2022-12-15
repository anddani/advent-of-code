use regex::Regex;
use std::time::Instant;

fn check_block(block_shapes: &Vec<((i32,i32),(i32,i32),(i32,i32),(i32,i32))>, y: i32) -> Vec<(i32, i32)> {
    let mut blocked_ranges: Vec<(i32, i32)> = Vec::new();
    // exploiting the square triangle property + rhombus property
    block_shapes.iter().for_each(|p| {
        if (p.1.1..=p.0.1).contains(&y) {
            let middle = (p.0.0, p.2.1);
            let edge_length = (y - middle.1).abs();
            blocked_ranges.push((p.2.0 + edge_length, p.3.0 - edge_length));
        }
    });
    blocked_ranges.sort_unstable_by_key(|r| r.0); // sort by start of range
    
    // if there are overlapping range, merge them
    let mut merged_ranges = Vec::new();
    merged_ranges.push(blocked_ranges[0]);
    for i_r in 1..blocked_ranges.len() {
        let next_range = blocked_ranges[i_r];
        let i_c = merged_ranges.len()-1;
        let current_range = &merged_ranges[i_c];
        if next_range.0 <= current_range.1  {
            if next_range.1 <= current_range.1 { 
                continue 
            } else {
                merged_ranges[i_c] = (current_range.0, next_range.1);
            }
        } else {
            merged_ranges.push(next_range);
        }
    }
    
    return merged_ranges
}

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let mut block_shapes: Vec<((i32,i32),(i32,i32),(i32,i32),(i32,i32))> = Vec::new();
    let r = Regex::new(r"Sensor at x=(?P<s_x>-?\d+), y=(?P<s_y>-?\d+): closest beacon is at x=(?P<b_x>-?\d+), y=(?P<b_y>-?\d+)").unwrap();
    f.lines().for_each(|l| {
        let c = r.captures(&l).unwrap();
        let s_x = c.name("s_x").unwrap().as_str().parse::<i32>().unwrap();
        let s_y = c.name("s_y").unwrap().as_str().parse::<i32>().unwrap();
        let b_x = c.name("b_x").unwrap().as_str().parse::<i32>().unwrap();
        let b_y = c.name("b_y").unwrap().as_str().parse::<i32>().unwrap();
        let manhattan_distance = (s_x-b_x).abs() + (s_y-b_y).abs();
        // 4 points of the parallelogram (rhombus): up, down, left, right
        let parallelogram = ((s_x, s_y + manhattan_distance),
                             (s_x, s_y - manhattan_distance),
                             (s_x - manhattan_distance, s_y),
                             (s_x + manhattan_distance, s_y));
        
        block_shapes.push(parallelogram);
    });
    
    let time = Instant::now();
    // Part 1
    if true {
        let y = 2000000;
        let blocked_ranges = check_block(&block_shapes, y);
        let n_pos = blocked_ranges.iter().fold(0, |acc, r| acc + (r.1-r.0));
        println!("Number of blocked position in y=2000000: {}",n_pos);    
    };
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Elapsed: {:.3} ms", elapsed_ms);
    
    // Part 2 (exploit the fact that there's only 1 position to find)
    let time = Instant::now();
    if true {
        let length = 4000000;
        for y in 0..=length {
            let blocked_ranges = check_block(&block_shapes, y);
            if blocked_ranges[0].0 < 0 && blocked_ranges[0].1 > length {
                continue
            } else if blocked_ranges.len()>1 {  
                let x = blocked_ranges[1].0 -1;
                let freq: i64 =  x as i64 * 4000000 + y as i64; 
                println!("Beacon {:?}, Tunelling frequency: {}", (x,y), freq);   
                break
            };
        };
    
    };
    let elapsed_s = time.elapsed().as_nanos() as f64 / 1_000_000_000.0;
    println!("Elapsed: {:.3} s", elapsed_s);
    
}

