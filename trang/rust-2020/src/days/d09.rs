pub fn run() {
    let stdin = std::fs::read_to_string("./data/input_d09.txt").unwrap();
    let nums: Vec<i64> = stdin.lines()
        .map(|line| line.parse().unwrap())
        .collect(); 
    let window_size: usize = 25;
    for (i, window) in nums.windows(window_size+1).enumerate() {
        if i < (window_size - 1) {
            continue
        }
        let mut valid = 0;
        //println!("{:?}", window);
        for i1 in 0..(window_size-1) {
            for i2 in (1+i1)..window_size {
                if window[i1] + window[i2] == window[window_size] {
                    valid += 1;
                }
            }
        }
        if valid == 0 {
            println!("First number to break: {:?}", window[window_size]);
            break
        }
    }
}