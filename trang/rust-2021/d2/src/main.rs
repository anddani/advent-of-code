fn part1() {
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
    }
    println!("Manhattan distance: {}", depth*horizontal);
}

fn part2() {    
    let f = std::fs::read_to_string("./input.txt").unwrap();

    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for l in f.lines() {
        let v: Vec<&str> = l.split(' ').collect();
        let stepsize: i32 = v[1].parse().unwrap();
        match v[0] {
            "forward" => {
                horizontal += stepsize;
                depth += aim*stepsize
            },
            "up" => aim -= stepsize,
            "down" => aim += stepsize,
            _ => panic!("Not yet implemented: {:?}", v[0]),
        }
    }
    println!("Manhattan distance with aim: {}", depth*horizontal);
}
fn main() {
    part1();
    part2();
}
