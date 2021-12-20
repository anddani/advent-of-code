use regex::Regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::iter::Zip;
use std::slice::Iter;

fn create_range(a:usize,b:usize) -> RangeInclusive<usize> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let re = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
    let mut covered_points: HashMap<(usize,usize), usize> = HashMap::new();

    for l in f.lines() {
        let c = re.captures(l).unwrap();
        let x1: usize = c.name("x1").unwrap().as_str().parse::<usize>().unwrap();
        let y1: usize = c.name("y1").unwrap().as_str().parse::<usize>().unwrap();
        let x2: usize = c.name("x2").unwrap().as_str().parse::<usize>().unwrap();
        let y2: usize = c.name("y2").unwrap().as_str().parse::<usize>().unwrap();
        //println!("{} {} {} {}", x1, y1, x2, y2);
        if x1 == x2 {
            for y in create_range(y1,y2) {
                //println!("{},{} ", x1,y);
                *covered_points.entry((x1,y)).or_insert(0) += 1;
            }
        }
        if y1 == y2 {
            for x in create_range(x1,x2) {
                //println!("{},{} ", x,y1);
                *covered_points.entry((x,y1)).or_insert(0) += 1;
            }
        }
        if (y1 as i32 - y2 as i32).abs() == (x1 as i32 - x2 as i32).abs() {
            let iter_zip : Zip<Iter<usize>, Iter<usize>>;
            let rx: Vec<usize> = create_range(x1,x2).collect::<Vec<usize>>();
            let ry: Vec<usize> = create_range(y1,y2).collect::<Vec<usize>>();
            let ry_rev: Vec<usize> = ry.iter().rev().map(|v| *v).collect::<Vec<usize>>();
            if (y1 as i32 - y2 as i32)*(x1 as i32 - x2 as i32) > 0 {
                iter_zip = rx.iter().zip(ry.iter());
            } else {
                iter_zip = rx.iter().zip(ry_rev.iter());
            }
            for (x, y) in iter_zip {
                //println!("{},{} ", x,y);
                *covered_points.entry((*x,*y)).or_insert(0) += 1;
            }
        }
        
    }
    //println!("{:?}", covered_points);
    let two = covered_points.values()
                            .cloned()
                            .collect::<Vec<usize>>()
                            .iter()
                            .filter(|&&x| x>=2)
                            .collect::<Vec<&usize>>()
                            .len();
    println!("Number of 2s: {:?}", two);
}
