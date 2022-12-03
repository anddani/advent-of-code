
use std::collections::HashMap;

fn part1() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let shape_score = HashMap::from([
        ("X", 1), // Rock
        ("Y", 2), // Paper
        ("Z", 3), // Scissors
    ]);

    let mut current_score = 0;
    for line in f.lines() {
        let moves: Vec<&str> = line.split_whitespace().collect();
        // println!("opponent move {}, your move {}", moves[0], moves[1])
        let result_type = match (moves[0],moves[1]) {
            ("A", "X") => 3, // Rock rock
            ("A", "Y") => 6, // Rock paper
            ("A", "Z") => 0, // Rock scissor
            ("B", "X") => 0, // paper rock
            ("B", "Y") => 3, // paper paper
            ("B", "Z") => 6, // paper scissor
            ("C", "X") => 6, // scissor rock
            ("C", "Y") => 0, // scissor paper
            ("C", "Z") => 3, // scissor scissor
            _ => panic!("invalid moves")
        };
        current_score += shape_score[moves[1]] + result_type
    }
    println!("Your score {}", current_score);
}

fn part2() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let shape_score = HashMap::from([
        ("Rock", 1), // Rock
        ("Paper", 2), // Paper
        ("Scissor", 3), // Scissors
    ]);

    let mut current_score = 0;
    for line in f.lines() {
        let moves: Vec<&str> = line.split_whitespace().collect();
        // println!("opponent move {}, your move {}", moves[0], moves[1])
        let result_type = match moves[1] {
            // Lose
            "X" => 0 + match moves[0] {
                "A" => shape_score["Scissor"], 
                "B" => shape_score["Rock"],
                "C" => shape_score["Paper"],
                _ => panic!("invalid moves")
            },
            // Draw
            "Y" => 3 + match moves[0] {
                "A" => shape_score["Rock"], 
                "B" => shape_score["Paper"],
                "C" => shape_score["Scissor"],
                _ => panic!("invalid moves")
            },
            // Win
            "Z" => 6 + match moves[0] {
                "A" => shape_score["Paper"], 
                "B" => shape_score["Scissor"],
                "C" => shape_score["Rock"],
                _ => panic!("invalid moves")
            },
            _ => panic!("invalid moves")
        };
        current_score += result_type
    }
    println!("Your score {}", current_score);
}

fn main() {
    part1();
    part2();
}
