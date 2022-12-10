fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let mut x: i32 = 1;
    let mut cycle: i32 = 0;
    let mut sig_strength: i32 = 0;
    f.lines().for_each(|l| {
        let m: Vec<&str> = l.split(" ").collect();
        //println!("{:?}", m);     
        match m[0] {
            "noop" => cycle+=1,
            "addx" => {
                for _i in 1..=2 {
                    cycle +=1;
                    if [20, 60, 100, 140, 180, 220].contains(&(cycle)) {
                        println!("cycle={}, x={}, strength={}",cycle, x, x*cycle);
                        sig_strength += x*cycle;
                    }
                }
                x += m[1].parse::<i32>().unwrap()},
            _ => panic!("Unknown key!"),
        }; 
    });
    println!("x={}, signal strength={}", x, sig_strength)
}