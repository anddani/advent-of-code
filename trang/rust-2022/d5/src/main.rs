use regex::Regex;

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();

    let mut blocks = f.split("\n\n");
    
    // Read initial stacks blocks
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];
    blocks.next().unwrap().lines().rev().skip(1).for_each(|line| {
        for (i, position) in [1,5,9,13,17,21,25,29,33].iter().enumerate() {
            stacks[i].push(line.chars().nth(*position).unwrap())
        }
    });
    for stack in &mut stacks {
        stack.retain(|&x| x != ' ');
    }
    println!("{:?}", stacks);
    let mut stacks_2 = stacks.clone();
    
    // Move following instructions
    let re_instructions = Regex::new(r"move (?P<n_tomove>\d+) from (?P<p_from>\d+) to (?P<p_to>\d+)").unwrap();
    blocks.next().unwrap().lines().for_each(|line| {
        let c = re_instructions.captures(&line).unwrap();
        let n_tomove = c.name("n_tomove").unwrap().as_str().parse::<usize>().unwrap();
        let from_stack = c.name("p_from").unwrap().as_str().parse::<usize>().unwrap();
        let to_stack = c.name("p_to").unwrap().as_str().parse::<usize>().unwrap();

        // Part 1
        for _ in 0..n_tomove {
            let letter = *stacks[from_stack -1].last().unwrap();
            let _ = &mut stacks[to_stack -1].push(letter);
            let _ = &mut stacks[from_stack -1].pop();
        }

        // Part 2
        let mut letters = Vec::new();
        for _ in 0..n_tomove {
            letters.push(*stacks_2[from_stack -1].last().unwrap());
            let _ = &mut stacks_2[from_stack -1].pop();
        }
        let _ = &mut stacks_2[to_stack -1].extend(letters.iter().rev());
    }); 
        
    let lastletters = stacks.iter().map(|s| *s.last().unwrap()).collect::<String>();
    println!("Part 1: {:?}", lastletters);
    let lastletters_2 = stacks_2.iter().map(|s| *s.last().unwrap()).collect::<String>();
    println!("Part 2: {:?}", lastletters_2);
}
