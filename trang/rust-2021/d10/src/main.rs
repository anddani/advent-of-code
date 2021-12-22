enum LineState {
    Complete,
    Incorrect(usize),
    Incomplete(Vec<char>),
}

fn check_line(line: &str) -> LineState {
    let mut stack: Vec<char> = Vec::new();
    let mut illegal_char = None;
    for c in line.chars() {
        match c {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            '<' => stack.push('>'),
            ')' | ']' | '}' | '>' => {
                if stack.last().cloned().unwrap() == c {
                    stack.pop(); 
                } else {
                    illegal_char = Some(c); 
                    break              
                }
            },
            _ => unreachable!(),
        }
    }

    if stack.is_empty() {
        return LineState::Complete
    } else {
        //println!("Print Illegal char {:?} ", illegal_char);
        match illegal_char {
            Some(')') => return LineState::Incorrect(3),
            Some('}') => return LineState::Incorrect(1197),
            Some(']') => return LineState::Incorrect(57),
            Some('>') => return LineState::Incorrect(25137),
            //correct and incomplete
            None => return LineState::Incomplete(stack),

            _ => panic!("illegal character unknown! {:?}", illegal_char),
        }
    }
}

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let mut error_score = 0;
    let mut autocomplete_score: Vec<usize> = Vec::new();
    for l in f.lines() {
        let r = check_line(l);
        match r {
            LineState::Complete => error_score+= 0,
            LineState::Incorrect(score) => error_score+= score,
            LineState::Incomplete(stack) => {
                autocomplete_score.push(
                    stack.iter().rev().fold(0, |acc, c| acc*5 + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    })    
                )
            }
        }
    }
    autocomplete_score.sort();
    let mid = autocomplete_score.len() / 2;
    
    println!("Total scores {}", error_score);
    println!("Middle autocomplete scores {}", autocomplete_score[mid]);
}
