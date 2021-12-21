use std::collections::HashSet;

fn decode_output(signal_strings: &str, output_strings: &str) -> usize {
    let signals = signal_strings.split(' ').collect::<Vec<_>>();
    let outputs = output_strings.split(' ').collect::<Vec<_>>();
    
    let chars_one: HashSet<char> = signals.iter()
                                            .filter(|n| n.len() == 2)
                                            .flat_map(|n| n.chars())
                                            .collect::<HashSet<char>>();

    let chars_four: HashSet<char> = signals.iter()
                                            .filter(|n| n.len() == 4)
                                            .flat_map(|n| n.chars())
                                            .collect::<HashSet<char>>();

    let chars_seven: HashSet<char> = signals.iter()
                                            .filter(|n| n.len() == 3)
                                            .flat_map(|n| n.chars())
                                            .collect::<HashSet<char>>();
    //println!("One:{:?}, Four:{:?}, Seven:{:?}", chars_one, chars_four, chars_seven);

    // (2,3,5) and (0,6,9) can be distinguished by overlaps of (1 and 4) or (4 and 7)
    let output_number = outputs.iter()
            .map(|n|
                match n.len() {
                    2 => 1,
                    4 => 4,
                    3 => 7,
                    7 => 8,
                    6 => {
                        let overlap_4 = n.chars().filter(|c| chars_four.contains(&c)).count();
                        let overlap_7 = n.chars().filter(|c| chars_seven.contains(&c)).count();
                        match (overlap_4, overlap_7) {
                            (3,2) => 6,
                            (4,3) => 9,
                            (3,3) => 0,
                            _ => panic!("Overlap of {} doesn't match any number {} {}", n, overlap_4, overlap_7),
                        }
                    },
                    5 => {
                        let overlap_4 = n.chars().filter(|c| chars_four.contains(&c)).count();
                        let overlap_7 = n.chars().filter(|c| chars_seven.contains(&c)).count();
                        match (overlap_4, overlap_7) {
                            (2,2) => 2,
                            (3,3) => 3,
                            (3,2) => 5,
                            _ => panic!("Overlap of {} doesn't match any number {} {}", n, overlap_4, overlap_7),
                        }
                    }
                    _ => unreachable!(),
                }
            )
            .fold(0, |prod, x| prod*10 + x);
        //println!("Output number: {}", output_number);
        
    return output_number
}

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();

    // part 1: count only 1(2), 4(4), 7(3), and 8(7), because they use unique number of segments
    let count_1478 = f.lines()
                        .filter_map(|l| l.split_once(" | "))
                        .flat_map(|(_signal_strings, output_strings)| output_strings.split(' '))
                        .filter(|n| n.len() == 2 || n.len() == 4 || n.len() == 3 || n.len() == 7)
                        .count();
    println!("Number of 1,4,7 or 8 in output strings: {}", count_1478);
    
    let mut sum:usize = 0;
    for l in f.lines() {
        let (signal_strings, output_strings) = l.split_once(" | ").unwrap();
        let number = decode_output(signal_strings, output_strings);
        sum += number;
    }
    println!("Sum of all output numbers: {}", sum);    
}
