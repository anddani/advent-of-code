enum LineState {
    Complete,
    Incomplete(usize),
}

fn check_line(line: &str) -> Result<LineState, <usize, usize>>{
    let mut stack: Vec<char> = Vec::new();
    let mut illegal_char = None;
    for c in line.chars() {
        match c {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            '<' => stack.push('>'),
            ')' | ']' | '}' | '>' => {
                if let Some(last) = stack.last().cloned() {
                    if c == last { stack.pop(); }
                } else {
                    illegal_char = Some(c);
                    println!("{}", c);
                }
            },
            _ => unreachable!(),
        }
    }

    if stack.is_empty() {
        Ok(LineState::Complete)
    } else {
        match illegal_char {
            Some(')') => Ok(LineState::Incomplete(3)),
            Some('}') => Ok(LineState::Incomplete(1197)),
            Some(']') => Ok(LineState::Incomplete(57)),
            Some('>') => Ok(LineState::Incomplete(25137)),
            _ => panic!("illegal character unknown! {:?}", illegal_char),
        }
    }
}

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    //f.lines().map(|l| check_line(l))
    let mut error_score = 0;
    for l in f.lines() {
        let r = check_line(l);
        match r {
            Ok(LineState::Complete) => error_score+= 0,
            Ok(LineState::Incomplete(score)) => error_score+= score,
            _ => panic!("return unknown!"),
        }
    }
    println!("Total scores {}", error_score);
}
