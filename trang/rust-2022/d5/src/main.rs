use regex::Regex;

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();

    let mut blocks = f.split("\n\n");
    let (mut p1, mut p2, mut p3, mut p4, mut p5, mut p6, mut p7, mut p8, mut p9) = (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());

    blocks.next().unwrap().lines().rev().skip(1).for_each(|line| {
        p1.push(line.chars().nth(1).unwrap());
        p2.push(line.chars().nth(5).unwrap());
        p3.push(line.chars().nth(9).unwrap());
        p4.push(line.chars().nth(13).unwrap());
        p5.push(line.chars().nth(17).unwrap());
        p6.push(line.chars().nth(21).unwrap());
        p7.push(line.chars().nth(25).unwrap());
        p8.push(line.chars().nth(29).unwrap());
        p9.push(line.chars().nth(33).unwrap());
    });
    println!("{:?}", p1);
    println!("{:?}", p2);
    p2.retain(|&x| x != ' ');
    println!("{:?}", p2);
    p4.retain(|&x| x != ' ');
    p5.retain(|&x| x != ' ');
    p7.retain(|&x| x != ' ');
    p8.retain(|&x| x != ' ');
    p8.retain(|&x| x != ' ');
    
    let re_instructions = Regex::new(r"move (?P<n_tomove>\d+) from (?P<p_from>\d+) to (?P<p_to>\d+)").unwrap();
    let instructions = blocks.next().unwrap().lines().for_each(|line| {
        let c = re_instructions.captures(&line).unwrap();
        n_mo = c.name('n_tomove');
    }); 
}
