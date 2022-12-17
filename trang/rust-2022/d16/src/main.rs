use regex::Regex;
//use petgraph::graphmap::DiGraphMap;

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    //let mut g = DiGraphMap::new();
    let r = Regex::new(r"Valve (?P<v_from>[A-Z]+) has flow rate=(?P<flowrate>[0-9]+); tunnels? leads? to valves? (?P<v_to>.+)").unwrap();
    f.lines().for_each(|l| {
        let c = r.captures(&l).unwrap();
        let v_from = c.name("v_from").unwrap().as_str();
        let flowrate = c.name("flowrate").unwrap().as_str().parse::<usize>().unwrap();
        let v_to: Vec<String> = c.name("v_to").unwrap().as_str().split(", ").map(|s| s.to_string()).collect();
        println!("{} {} {:?}", v_from, flowrate,v_to);
        //g.add_edge(node1, node2, 1);
    });
    
    //let paths = all_simple_paths::<Vec<_>, _>(&g, "start", "end", 0, None).collect::<Vec<_>>();
}
