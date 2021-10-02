fn rotate_vector(v: (i32,i32), degree: f32) -> (i32,i32) {
    let theta = degree.to_radians();
    println!("Degree: {}, theta: {}, sin: {}", degree, theta, theta.sin());
    //let rot_matrix = [[theta.cos(), -theta.sin()],[theta.sin() + theta.cos()]]
    let new_0 = v.0 * theta.cos() as i32 - v.1 * theta.sin() as i32;
    let new_1 = v.0 * theta.sin() as i32  + v.1 * theta.cos() as i32;
    println!("{:?} turns by {} degree to {:?}", v, degree, (new_0, new_1));
    return (new_0, new_1)
}

pub fn run() {
    let f = std::fs::read_to_string("./data/input_d12.txt").unwrap();
    let mut x: i32 = 0; // West(-) to East(+), ship at 0
    let mut y: i32 = 0; // North(-) to South(+), ship at 0
    let mut v: (i32,i32) = (1,0); // ship facing East
    for l in f.lines() {
        let (direction, stepsize) = l.split_at(1);
        let stepsize: i32 = stepsize.parse().unwrap();
        match direction {
            "N" => y -= stepsize,
            "S" => y += stepsize,
            "E" => x += stepsize,
            "W" => x -= stepsize,
            "L" => v = rotate_vector(v, -stepsize as f32),
            "R" => v = rotate_vector(v, stepsize as f32),
            "F" => {
                x += v.0*stepsize;
                y += v.1*stepsize;
            },
            _ => panic!("Not yet implemented: {:?}", direction),
        }
        println!("{} {}", direction, stepsize);
    }
    println!("Manhattan distance: {}", x.abs() + y.abs());
}