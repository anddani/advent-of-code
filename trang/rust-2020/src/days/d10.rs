pub fn run() {
    let f = std::fs::read_to_string("./data/input_d10.txt").unwrap();
    let mut adapters = f.lines().map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    let buildin: usize = adapters.iter().max().unwrap() + 3;
    adapters.insert(0,0); //insert(index,value)
    adapters.insert(0,buildin);
    adapters.sort();

    let jolts = adapters.windows(2).map(|x| x[1]-x[0]).collect::<Vec<_>>();
    //println!("{:?}", jolts);
    let count_1 = jolts.iter().filter(|&n| *n == 1).count();
    let count_3 = jolts.iter().filter(|&n| *n == 3).count();
    println!("1s {}, 3s {}, product {}", count_1, count_3, count_1*count_3);

    let mut count2: i64 = 1;
    let mut current_chunk: Vec<_> = Vec::new();
    for j in jolts {
        if j == 3 {
            let count1s = current_chunk.iter().filter(|&n| *n == 1).count();
            match count1s {
                0 => count2 += 0,
                1 => count2 += 0,
                2 => count2 *= 2,
                3 => count2 *= 4,
                4 => count2 *= 7,
                _ => panic!("Uncovered pattern {}", count1s),
            }
            current_chunk.clear();
        }
        else { current_chunk.push(j) }
    }
    println!("All paths: {}", count2);
}   