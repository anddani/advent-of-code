fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let n: Vec<usize> = f.lines().map(|x| x.parse().unwrap()).collect();
    println!("{} lines", n.len());
    let mut n_increase = 0;

    for i in 0..(n.len()-1) {
        if n[i+1]>n[i] {
            n_increase += 1;
        }
    }
    println!("number of increased measurement: {}", n_increase);
}
