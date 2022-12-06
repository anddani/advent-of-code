use std::collections::HashSet;

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let l = f.chars().count() - 4;
    for i in 0..l {
        let sliding_window = f.chars().skip(i).take(4).collect::<HashSet<char>>();
        if sliding_window.len() == 4 {
            let start_of_pack = f.chars().take(i+4).collect::<String>();
            println!("{}", start_of_pack.chars().count());
            break
        }
    }
}
