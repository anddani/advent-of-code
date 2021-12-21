fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();

    // part 1: count only 1(2), 4(4), 7(3), and 8(7), because they use unique number of segments
    let count_1478 = f.lines()
                        .filter_map(|l| l.split_once(" | "))
                        .flat_map(|(_digit_strings, output_strings)| output_strings.split(' '))
                        .filter(|n| n.len() == 2 || n.len() == 4 || n.len() == 3 || n.len() == 7)
                        .count();
    println!("Number of 1,4,7 or 8 in output strings: {}", count_1478);
}
