use regex::Regex;

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let re_assgnment = Regex::new(r"(?P<l1>\d+)-(?P<h1>\d+),(?P<l2>\d+)-(?P<h2>\d+)").unwrap();
    let assignments: Vec<(usize, usize, usize, usize)> = f.lines().map(|line| {
        let c = re_assgnment.captures(&line).unwrap();
        (
            c.name("l1").unwrap().as_str().parse::<usize>().unwrap(),
            c.name("h1").unwrap().as_str().parse::<usize>().unwrap(),
            c.name("l2").unwrap().as_str().parse::<usize>().unwrap(),
            c.name("h2").unwrap().as_str().parse::<usize>().unwrap()
        )
    }).collect();

    let mut count = 0;
    for x in assignments.iter() {
        if (x.0 <= x.2 && x.1 >= x.3) || (x.2 <= x.0 && x.3 >= x.1) {
            count +=1
        }
    };
    println!("Fully overlap: {}", count);

    
    let mut count2 = 0;
    for x in assignments.iter() {
        let r1 = x.0..=x.1;
        let r2 = x.2..=x.3;
        if r1.contains(&x.2) || r1.contains(&x.3) || r2.contains(&x.0) || r2.contains(&x.1) {
            count2 +=1
        }
    };
    println!("Overlap at all: {}", count2);
}
