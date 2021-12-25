use std::collections::HashMap;
use pathfinding::directed::dijkstra;

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let mut m: HashMap<(i32, i32), i32> = HashMap::new();

    for (y,l) in f.lines().enumerate() {
        for (x,c) in l.chars().enumerate() {
            m.insert((x as i32,y as i32),c.to_string().parse::<i32>().unwrap());

        }
    };
    let goal = (*m.iter().map(|((x,_),_)| *x).collect::<Vec<i32>>().iter().max().unwrap(), 
                f.lines().count() as i32 - 1);

    println!("Goal: {:?}", goal);
    
    let (_shortest_path, cost) = dijkstra::dijkstra(&(0, 0),
                      |&(x, y)| vec![(x+1,y), (x-1,y), (x,y+1), (x,y-1)]
                                 .into_iter()
                                 .map(|p| (p, match m.get(&p) {
                                     Some(&v) => v,
                                     None => 10,
                                 })),
                      |&p| p == goal)
                      .expect("No shortest path");
    
    println!("Shortest path cost {}", cost);

    // 5x the map    
    let m_old = m.clone();
    for x_i in 0..=4 {
        for y_i in 0..=4 {
            for ((x,y), v) in m_old.iter() {
                let new_v = if (*v + x_i+y_i) %9 ==0 { 9 } else {(*v + x_i+y_i) %9};
                m.insert((*x + x_i*(goal.0 +1) as i32, *y + y_i*(goal.1+1) as i32), new_v);
            }    
        }
    }
    
    let goal = (*m.iter().map(|((x,_),_)| *x).collect::<Vec<i32>>().iter().max().unwrap(), 
                *m.iter().map(|((_,y),_)| *y).collect::<Vec<i32>>().iter().max().unwrap());
    println!("Goal: {:?}", goal);

    let (_shortest_path, cost) = dijkstra::dijkstra(&(0, 0),
                      |&(x, y)| vec![(x+1,y), (x-1,y), (x,y+1), (x,y-1)]
                                 .into_iter()
                                 .map(|p| (p, match m.get(&p) {
                                     Some(&v) => v,
                                     None => 9,
                                 })),
                      |&p| p == goal)
                      .expect("Not found");
    
    println!("Shortest path cost {}", cost);
}
