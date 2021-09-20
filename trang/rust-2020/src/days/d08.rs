fn line_to_instruction(l: &str) -> (&str, i32) {
    let words: Vec<&str> = l.split_whitespace().collect();
    let operation = words[0];//words.get(0).unwrap();
    let argument = words[1].trim().parse::<i32>().unwrap();
    //println!("{:?} {:?}", operation, argument);

    return (operation, argument)
}

fn add(u: usize, i: i32) -> usize {
    // usize and i32 cannot be added directly
    // adding a negative i32 number to an usize variable might/will result in overflow
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}

pub fn run() {
    let stdin = std::fs::read_to_string("./data/input_d08.txt").unwrap();
    
    let instructions: Vec<(&str, i32)> = stdin.lines()
        .map(|line| line_to_instruction(&line))
        .collect();
    //println!("{:?}", instructions[1]);

    let (mut ids, mut id, mut acc) = (vec![], 0, 0);
    while !ids.contains(&id) {// ! is a negative trait implementation
        ids.push(id);            
        println!("current id {}, line {:?}, current acc {}", id, instructions[id], acc);
        match instructions[id] { //slice indices are of type `usize`
            ("acc", n) => acc += n, 
            ("jmp", n) => id = add(id, n) -1, 
            ("nop", _) => acc += 0,
            _ => panic!("Not yet implemented!"),
        };
        id += 1;
    }
    println!("Accumulator of 1st round {}", acc);
}
