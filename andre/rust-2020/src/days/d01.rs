
fn solve1(input: &Vec<u32>) -> u32 {
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i != j && input[i] + input[j] == 2020 {
                return input[i] * input[j];
            }
        }
    }
    panic!("Fail");
}

fn solve2(input: &Vec<u32>) -> u32 {
    for i in 0..input.len() {
        for j in 0..input.len() {
            for k in 0..input.len() {
                if i != j && j != k && input[i] + input[j] + input[k] == 2020 {
                    return input[i] * input[j] * input[k];
                }
            }
        }
    }
    panic!("Fail");
}

pub fn run() {
    let input: Vec<u32> = std::fs::read_to_string("./data/d01.txt").unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    println!("Part 1: {}", solve1(&input));
    println!("Part 2: {}", solve2(&input));
}


