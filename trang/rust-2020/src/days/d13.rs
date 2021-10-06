fn part1() {
    let f = std::fs::read_to_string("./data/input_d13.txt").unwrap();
    let depart: i32 = f.lines().nth(0).unwrap().parse().unwrap();
    let busses: Vec<i32> = f.lines().nth(1).unwrap().split(",").filter(|&b| b != "x").map(|b| b.parse().unwrap()).collect::<Vec<i32>>();
    println!("{} {:?}", depart, busses);
    
    let mut closest_b = busses[0];
    let mut cwt = depart; 
    for b in busses.iter() {
        let t = depart / b;
        let wt = b*(t+1) - depart;
        if wt < cwt {
            cwt = wt;
            closest_b = *b;
        }
    }
    println!("Shortest waiting time {} from bus {} => {}", cwt, closest_b, cwt*closest_b);
}

fn part2() {
    let f = std::fs::read_to_string("./data/input_d13.txt").unwrap();
    let all_busses: Vec<usize> = f.lines().nth(1).unwrap().split(",").map(|b| b.parse().unwrap_or(0)).collect::<Vec<usize>>();
    let mut busses = all_busses.iter().enumerate().filter(|(i,&b)| b != 0).collect::<Vec<(usize, &usize)>>();
    busses.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    println!("{:?}", busses);
    let mut timestamp = 0;
    let mut inc = *busses[0].1;
    for &(i, &bus) in &busses[1..] {
        // Chinese remainder theorem
        while (timestamp + i) % bus != 0 {
            timestamp += inc;
        }
        inc *= bus;
    }
    println!("Earliest matched time {}", timestamp);
}
pub fn run() {
    let run_part1 = true;
    let run_part2 = true;
    if run_part1 { part1(); }
    if run_part2 { part2(); }

}