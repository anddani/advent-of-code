use petgraph::graphmap::DiGraphMap;
use petgraph::algo::all_simple_paths;
//use petgraph::graph::NodeIndex;

fn make_graph(adapters: &Vec<usize>) -> usize{
    let mut g = DiGraphMap::new();
    let m: usize = *adapters.iter().max().unwrap();
    for (i, v) in adapters.iter().enumerate() {
        for ahead in 1..4 { //look upto 3 numbers ahead
            while i+ahead < adapters.len()-1 {
                let diff: usize = adapters[i+ahead] - v;
                if (1..4).contains(&diff) {
                    g.add_edge(v, &adapters[i+ahead], 1);
                }
            }
        }
    }
    let mut count_paths = all_simple_paths::<Vec<&usize>, _>(&g, &0, &m, 2, Some(adapters.len()-1)).count();
    return count_paths
}

pub fn run() {
    let f = std::fs::read_to_string("./data/test.txt").unwrap();
    let mut adapters = f.lines().map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    let buildin: usize = adapters.iter().max().unwrap() + 3;
    adapters.insert(0,0); //insert(index,value)
    adapters.insert(0,buildin);
    adapters.sort();

    let jolts = adapters.windows(2).map(|x| x[1]-x[0]).collect::<Vec<_>>();
    //println!("{:?}", jolts);
    let mut count_1 = jolts.iter().filter(|&n| *n == 1).count();
    let mut count_3 = jolts.iter().filter(|&n| *n == 3).count();
    println!("1s {}, 3s {}, product {}", count_1, count_3, count_1*count_3);

    //let graph = make_graph(&adapters);
    let count2 = make_graph(&adapters);
    println!("All paths: {}", count2);
}   