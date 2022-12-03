
fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let elfs = f.split("\n\n");
    
    // Part 1&2
    let mut counts = elfs.map(|elf| {
            elf.lines().map(|x| x.parse().unwrap()).collect::<Vec<usize>>().iter().sum()
        })
        .collect::<Vec<usize>>();
    
    counts.sort(); //ascending
    counts.reverse();
    println!("Highest calorie {:?}", counts[0]);
    println!("Sum calorie of top 3 elfs: {}", counts[0]+counts[1]+counts[2]);
}
