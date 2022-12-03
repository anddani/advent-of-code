
fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let elfs = f.split("\n\n");
    
    let mut highest_calorie = 0;
    for p in elfs {
        let calorie_count = p.lines().map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
        let total_calorie: usize = calorie_count.iter().sum();
        
        if highest_calorie < total_calorie {
            highest_calorie = total_calorie
        }
    }
    println!("calorie_count {:?}", highest_calorie);
}
