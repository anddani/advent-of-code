/*
fn part_1(nums: &Vec<i64>) -> i64 {
    let ws: usize = 25;
    for (i, window) in nums.windows(ws+1).enumerate() {
        if i < (ws - 1) {
            continue
        }
        let mut valid = 0;
        println!("{:?}", window);
        for i1 in 0..(ws-1) {
            for i2 in (1+i1)..ws {
                if window[i1] + window[i2] == window[ws] {
                    valid += 1;
                }
            }
        }
        if valid == 0 {
            println!("First number to break: {:?}", window[ws]);
            return *window[ws]
        }
    }
}
*/
pub fn run() {
    let stdin = std::fs::read_to_string("./data/input_d09.txt").unwrap();
    let nums: Vec<i64> = stdin.lines()
        .map(|line| line.parse().unwrap())
        .collect(); 

    //let solution_1 = part_1(&nums);
    //println!("{}", solution_1);
    let mut solution_1: i64 = 0;
    let ws: usize = 25;
    for (i, window) in nums.windows(ws+1).enumerate() {
        if i < (ws - 1) {
            continue
        }
        let mut valid = 0;
        for i1 in 0..(ws-1) {
            for i2 in (1+i1)..ws {
                if window[i1] + window[i2] == window[ws] {
                    valid += 1;
                }
            }
        }
        if valid == 0 {
            println!("First number to break: {:?}", window[ws]);
            solution_1 = window[ws];
            break
        }
    }

    for ws2 in 2.. {
        for (_, window) in nums.windows(ws2+1).enumerate() {
            if window.iter().sum::<i64>() == solution_1 {
                let solution_2 = window.iter().min().unwrap() + window.iter().max().unwrap();
                println!("Encryption weakness: {:?}", solution_2);
                break
            }
        }    
    }
}