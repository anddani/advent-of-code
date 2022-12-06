use std::collections::HashSet;

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    // Part 2
    let l = f.chars().count() - 4;
    for i in 0..l {
        let sliding_window = f.chars().skip(i).take(4).collect::<HashSet<char>>();
        if sliding_window.len() == 4 {
            let start_of_pack = f.chars().take(i+4).collect::<String>();
            println!("Start of pack: {}", start_of_pack.chars().count());
            break
        }
    }

    // Part 2    
    let l = f.chars().count() - 14;
    for i in 0..(l+1) {
        let sliding_window = f.chars().skip(i).take(14).collect::<HashSet<char>>();
        if sliding_window.len() == 14 {
            let start_of_message = f.chars().take(i+14).collect::<String>();
            println!("Start of message: {}", start_of_message.chars().count());
            break
        }
    }
}
