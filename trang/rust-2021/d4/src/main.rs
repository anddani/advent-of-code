use std::collections::HashMap;

#[derive(Debug)]
struct Board {
    board_number: usize,
    score: usize,
    d: usize,
}

fn calculate_score(board_map: &HashMap<(usize, usize), usize>, draw: &Vec<usize>) -> (usize, usize) {
    let mut score: usize = board_map.values().cloned().collect::<Vec<usize>>().iter().sum();
    let mut score_board = vec![vec![0; 5];5];
    let old_board_map = board_map.clone();
    for (d_i,d) in draw.iter().enumerate() {
        for (coord, num) in old_board_map.iter() {
            if d == num {
                score_board[coord.0][coord.1] = 1;
                score -= num;
            }
        };
        if d_i>4 {
            let row_sums: Vec<usize> = score_board.iter().map(|row| row.iter().sum::<usize>()).collect();
            let col_sums: Vec<usize> = (0..5).map(|i| score_board.iter().flatten().skip(i).step_by(5).sum::<usize>()).collect();

            if (row_sums.iter().filter(|&r| *r==5).count() > 0) || (col_sums.iter().filter(|&c| *c==5).count() > 0) {
                return (score*d, d_i)
            };
        };
    };
    return (score* draw.len(), draw.len())
}

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    
    let draw: Vec<usize> =  f.lines()
                            .take_while(|line| !line.is_empty())
                            .flat_map(|line| {
                                line.split(',').map(|t| t.parse::<usize>().unwrap())
                            })
                            .collect();
    let mut board_scores: Vec<Board> = Vec::new(); // (Board_number, (score, winning draw order))
    
    let mut board_number = 0;
    let mut current_board_positions: HashMap<(usize, usize), usize> = HashMap::new();
    let mut row_number: usize = 0;
    for l in f.lines().skip(1) {
        if l.is_empty() {
            let (score, d_i) = calculate_score(&current_board_positions, &draw);
            board_number +=1;
            let current_board = Board {
                board_number: board_number,
                score: score,
                d: d_i,
            };
            board_scores.push(current_board);
            current_board_positions.clear();
            row_number=0;
        }; 
        for (col_number, v) in l.split_ascii_whitespace().enumerate() {
            current_board_positions.insert((row_number-1,col_number),v.parse::<usize>().unwrap());
        };
        row_number += 1;
    }
    board_scores.remove(0); // Remove the empty line
    println!("n_boards: {}", board_number);
    //println!("n_boards: {:?}", board_scores);
    
    let winning_board: &Board = board_scores.iter().min_by_key(|b| b.d).unwrap();
    let loosing_board: &Board = board_scores.iter().max_by_key(|b| b.d).unwrap();
    println!("{:?}", winning_board);
    println!("{:?}", loosing_board);
    
}
