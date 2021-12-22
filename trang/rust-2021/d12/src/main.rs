use petgraph::graphmap::UnGraphMap;
use petgraph::algo::all_simple_paths;
use regex::Regex;
//use std::collections::HashSet;

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let mut g = UnGraphMap::new();
    let re_path = Regex::new(r"(?P<node1>\w+)-(?P<node2>\w+)").unwrap();
    //let mut all_paths: HashSet<Vec<_>> = HashSet::new();

    f.lines().for_each(|l| {
        let c = re_path.captures(&l).unwrap();
        let node1 = c.name("node1").unwrap().as_str();
        let node2 = c.name("node2").unwrap().as_str();
        //println!("{:?}, {:?}", node1, node2);
        g.add_edge(node1, node2, 1);
    });
    let paths = all_simple_paths::<Vec<_>, _>(&g, "start", "end", 0, None).collect::<Vec<_>>();
    //println!("{:?}", paths);
    let mut n_paths = paths.len();
    for p in paths.iter() {
        println!("{}", p.len());
        if p.len() == 4 { //2 middle points
            let mut paths_innter = all_simple_paths::<Vec<_>, _>(&g, p[1], p[2], 1, None)
                                            .filter(|v| !v.contains(&"start") && !v.contains(&"end"))
                                            .collect::<Vec<_>>();
            let c_ = paths_innter.len();
            //println!("{:?} {}", paths_innter, c_);
            n_paths += c_;
        }
        if p.len() == 5 { //3 middle points
            let paths_innter = all_simple_paths::<Vec<_>, _>(&g, p[1], p[2], 1, None)
                                            .filter(|v| !v.contains(&"start") && !v.contains(&"end"))
                                            .collect::<Vec<_>>();
            let c_1 = paths_innter.len();
            println!("{:?} {}", paths_innter, c_1);
            let paths_innter = all_simple_paths::<Vec<_>, _>(&g, p[2], p[3], 1, None)
                                            .filter(|v| !v.contains(&"start") && !v.contains(&"end"))
                                            .collect::<Vec<_>>();
            let c_2 = paths_innter.len();
            println!("{:?} {}", paths_innter, c_2);
            println!("{} {}: {}", c_1,c_2, c_1*c_2);
            n_paths += c_1+c_2 + c_1*c_2;
        }
    }
    //println!("{:?}", paths);
    println!("Number of paths {}", n_paths);
}
