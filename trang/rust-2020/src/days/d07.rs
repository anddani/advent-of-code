use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use petgraph::graphmap::DiGraphMap;
use regex::Regex;

fn graph(lines: &[String]) -> DiGraphMap<&str, usize> {
    let re_node0 = Regex::new(r"(\w+) bags contain").unwrap();
    let re_node1 = Regex::new(r"(\d+) (\w+) bag").unwrap();
    let mut g = DiGraphMap::new();
    for l in lines {
        let bag0 = re_node0.captures(&l).unwrap().get(1).unwrap().as_str();
        let bags1 = re_node1.captures_iter(&l);
        for b in bags1 {
            let n: usize = b[1].parse().unwrap();
            let bag = b.get(2).unwrap().as_str();
            g.add_edge(bag0, bag, n);
            println!{"{}: {} {}", bag0,bag, n};
        }
    }
    return g;
}

pub fn run() {
    let time = Instant::now();
    let f = BufReader::new(File::open("./data/test.txt").unwrap()); //read_to_string("./data/test.txt").unwrap();
    let lines: Vec<String> = f.lines().map(|l| l.unwrap()).collect();
    let g = graph(&lines);
    assert!(g.contains_edge("dark orange", "shiny gold"));
    let count1 = 0; //g.neighbors_directed("shiny gold")
    println!("N bags that can contain at least 1 shiny gold bag {}", count1);

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Elapsed: {:.3} ms", elapsed_ms);
}