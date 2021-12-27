use regex::Regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::cmp;
//use std::{cmp, fmt};
//use std::iter::FromIterator;
/*
#[derive(Debug, Copy, Clone)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn new(start: i32, end: i32) -> Range {
        Range {
            start: start,
            end: end,
        }
    }

    fn overlaps(&self, other: &Range) -> bool {
        (other.start >= self.start && other.start <= self.end)
        || (other.end >= self.start && other.end <= self.end)
    }

    fn merge(&mut self, other: &Range) {
        self.start = cmp::min(self.start, other.start);
        self.end = cmp::max(self.end, other.end);
    }
}
*/
fn check_overlap_range50(start: i32, end: i32) -> bool {
    (start >= -50 && start <= 50)
    || (end >= -50 && end <= 50)
}

fn overlap_range(start: i32, end: i32) -> RangeInclusive<i32> {
    cmp::max(start, -50)..=cmp::min(end, 50)
}

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let re = Regex::new(r"(?P<switch>\w+) x=(?P<x0>-?\d+)..(?P<x1>-?\d+),y=(?P<y0>-?\d+)..(?P<y1>-?\d+),z=(?P<z0>-?\d+)..(?P<z1>-?\d+)").unwrap();
    let mut cube_map: HashMap<(i32, i32, i32), usize> = HashMap::new();
    
    // Initialization step
    f.lines().for_each(|l| {
        let c = re.captures(&l).unwrap();
        let switch = match c.name("switch").unwrap().as_str() {
            "on" => 1,
            "off" => 0,
            _ => panic!("Instruction not found {}", c.name("switch").unwrap().as_str()),
        };
        let x0 = c.name("x0").unwrap().as_str().parse::<i32>().unwrap();
        let x1 = c.name("x1").unwrap().as_str().parse::<i32>().unwrap();
        let y0 = c.name("y0").unwrap().as_str().parse::<i32>().unwrap();
        let y1 = c.name("y1").unwrap().as_str().parse::<i32>().unwrap();
        let z0 = c.name("z0").unwrap().as_str().parse::<i32>().unwrap();
        let z1 = c.name("z1").unwrap().as_str().parse::<i32>().unwrap();

        if check_overlap_range50(x0,x1) && check_overlap_range50(y0,y1) &&check_overlap_range50(z0,z1) {
            //println!("{} {} {} {} {} {} {}", switch, x0,x1, y0, y1, z0,z1);
            for x in overlap_range(x0,x1) {
                for y in overlap_range(y0,y1) {
                    for z in overlap_range(z0,z1) {
                        cube_map.insert((x,y,z), switch);
                    }
                }
            }
        }  

    });
    let ons: usize = cube_map.values().cloned().collect::<Vec<usize>>().iter().sum();
    println!("{:?}", ons);
}
