fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let mut x: i32 = 1;
    let mut cycle: i32 = 0;
    let mut sig_strength: i32 = 0;
    let mut draw_line = Vec::new();
    f.lines().for_each(|l| {
        let m: Vec<&str> = l.split(" ").collect();
        
        match m[0] {
            "noop" => {
                if [x-1, x, x+1].contains(&(cycle%40)) {
                    draw_line.push('#');
                } else {
                    draw_line.push('.');
                };
                cycle+=1;
                if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                    println!("cycle={}, x={}, strength={}",cycle, x, x*cycle);
                    sig_strength += x*cycle;
                }
            },
            "addx" => {
                for _i in 0..=1 {
                    if [x-1, x, x+1].contains(&(cycle%40)) {
                        draw_line.push('#');
                    } else {
                        draw_line.push('.');
                    }
                    cycle +=1;
                    if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                        println!("cycle={}, x={}, strength={}",cycle, x, x*cycle);
                        sig_strength += x*cycle;
                    }
                    x += _i*m[1].parse::<i32>().unwrap();
                }
            },
            _ => panic!("Unknown key!"),
        }; 
    });
    println!("x={}, signal strength={}", x, sig_strength);
    draw_line.chunks(40).for_each(|chunk|{
        println!("{:?}", chunk.iter().collect::<String>());
    });
}