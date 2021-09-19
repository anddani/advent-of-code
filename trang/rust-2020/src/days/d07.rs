use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use petgraph::graphmap::DiGraphMap;
use regex::Regex;
use petgraph::algo::has_path_connecting;
use ::petgraph::Direction;


fn count_weight_recursive(g: &DiGraphMap<&str, usize>, root_node: &str) -> usize{
    let mut count = 0;
    for node in g.neighbors_directed(root_node, Direction::Outgoing) {
        let w = g.edge_weight(root_node, node).unwrap();
        count += w + w*count_weight_recursive(g, node);
    }
    return count
}

pub fn run() {
    let time = Instant::now();

    let f = BufReader::new(File::open("./data/input_d07.txt").unwrap()); //read_to_string("./data/test.txt").unwrap();
    let lines: Vec<String> = f.lines().collect::<Result<_, _>>().unwrap();

    // Construct a directed graph where nodes are colors and weights are the number bags a upper node/bag can hold
    let mut g = DiGraphMap::new();
    let re_node0 = Regex::new(r"([\w ]+) bags contain").unwrap();
    let re_node1 = Regex::new(r"(\d+) ([\w ]+) bags?").unwrap();
    for l in &lines {
        let bag0 = re_node0.captures(&l).unwrap().get(1).unwrap().as_str(); //`&str`
        for b in re_node1.captures_iter(&l) {
            let n: usize = b.get(1).unwrap().as_str().parse().unwrap();
            let bag1 = b.get(2).unwrap().as_str(); //`&str`
            g.add_edge(bag0, bag1, n);
            println!{"{}: {} {}", bag0, bag1, n};
        }
    }

    // Part 1
    let mut count1 = 0;
    for node in g.nodes() {
        println!("Node: {:?}", node);
        if node != "shiny gold" {
            if has_path_connecting(&g, node, "shiny gold", None) {
                count1 += 1;
            }
        }
    }
    println!("N bags that can contain at least 1 shiny gold bag {}", count1);
    
    // Part 2
    let count2 = count_weight_recursive(&g, "shiny gold");
    println!("N bags are required inside 1 shiny gold bag {}", count2);

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Elapsed: {:.3} ms", elapsed_ms);
}