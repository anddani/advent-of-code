use regex::Regex;
use std::collections::HashSet;

fn check_block(block_range: &Vec<((i32,i32),(i32,i32),(i32,i32),(i32,i32))>, y: i32) -> HashSet<i32> {
    let mut blocked_pos: HashSet<i32> = HashSet::new();
    // exploiting the square triangle property + rhombus property
    block_range.iter().for_each(|p| {
        if (p.1.1..=p.0.1).contains(&y) {
            let middle = (p.0.0, p.2.1);
            let edge_length = (y - middle.1).abs();
            for i_x in (p.2.0 + edge_length)..=(p.3.0 - edge_length) {
                blocked_pos.insert(i_x); 
            }
        }
    });
    return blocked_pos
}

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let mut block_range: Vec<((i32,i32),(i32,i32),(i32,i32),(i32,i32))> = Vec::new();
    let mut beacon_pos: HashSet<(i32,i32)> = HashSet::new();
    let r = Regex::new(r"Sensor at x=(?P<s_x>-?\d+), y=(?P<s_y>-?\d+): closest beacon is at x=(?P<b_x>-?\d+), y=(?P<b_y>-?\d+)").unwrap();
    f.lines().for_each(|l| {
        let c = r.captures(&l).unwrap();
        let s_x = c.name("s_x").unwrap().as_str().parse::<i32>().unwrap();
        let s_y = c.name("s_y").unwrap().as_str().parse::<i32>().unwrap();
        let b_x = c.name("b_x").unwrap().as_str().parse::<i32>().unwrap();
        let b_y = c.name("b_y").unwrap().as_str().parse::<i32>().unwrap();
        beacon_pos.insert((b_x, b_y));
        let manhattan_distance = (s_x-b_x).abs() + (s_y-b_y).abs();
        // 4 points of the parallelogram (rhombus): up, down, left, right
        let parallelogram = ((s_x, s_y + manhattan_distance),
                             (s_x, s_y - manhattan_distance),
                             (s_x - manhattan_distance, s_y),
                             (s_x + manhattan_distance, s_y));
        
        block_range.push(parallelogram);
    });
    let y = 2000000;
    let blocked_pos = check_block(&block_range, y);
    let b_in_range = beacon_pos.iter().filter(|(b_x, b_y)|{
        (b_y == &y) && blocked_pos.contains(&b_x)
    }).count();
    println!("Number of blocked position in y=2000000: {}", blocked_pos.len()-b_in_range);
}
