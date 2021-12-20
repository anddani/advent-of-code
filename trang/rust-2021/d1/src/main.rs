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

    let ws = 3;
    let mut previous_sum = 0;
    let mut n_increase2 = -1; // compensate for the first comparision, which will result in false positve
    for (i, window) in n.windows(ws).enumerate() {
        let current_sum = window.iter().sum(); 
        if current_sum > previous_sum {
            n_increase2 += 1;
        }
        previous_sum = current_sum;
    }
    println!("number of increased sum of 3: {}", n_increase2);
}
