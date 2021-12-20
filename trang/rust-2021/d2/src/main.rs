fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();

    let mut depth = 0;
    let mut horizontal = 0;
    for l in f.lines() {
        let v: Vec<&str> = l.split(' ').collect();
        let stepsize: i32 = v[1].parse().unwrap();
        match v[0] {
            "forward" => horizontal += stepsize, 
            "up" => depth -= stepsize,
            "down" => depth += stepsize,
            _ => panic!("Not yet implemented: {:?}", v[0]),
        }
        println!("{:?} {}", v[0], stepsize);
    }
    println!("Manhattan distance: {}", depth*horizontal);
}
