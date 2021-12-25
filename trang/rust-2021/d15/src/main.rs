use std::collections::HashMap;
use pathfinding::directed::dijkstra;

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let mut m: HashMap<(i32, i32), usize> = HashMap::new();

    for (y,l) in f.lines().enumerate() {
        for (x,c) in l.chars().enumerate() {
            m.insert((x as i32,y as i32),c.to_string().parse::<usize>().unwrap());

        }
    };
    let goal = (*m.iter().map(|((x,_),_)| *x).collect::<Vec<i32>>().iter().max().unwrap(), 
                f.lines().count() as i32 - 1);

    println!("{:?}", goal);
    
    let result = dijkstra::dijkstra(&(0, 0),
                      |&(x, y)| vec![(x+1,y), (x-1,y), (x,y+1), (x,y-1)]
                                 .into_iter()
                                 .map(|p| (p, match m.get(&p) {
                                     Some(&v) => v,
                                     None => 9,
                                 })),
                      |&p| p == goal);
    
    println!("Shortest path {:?}", result);
}
