pub fn run() {
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