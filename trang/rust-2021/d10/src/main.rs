fn check_line(line: &str) -> usize {
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
        //fprintln!("{:?} {}", stack, c);
    }

    if stack.is_empty() {
        return 0
    } else {
        //println!("Print Illegal char {:?} ", illegal_char);
        match illegal_char {
            Some(')') => return 3,
            Some('}') => return 1197,
            Some(']') => return 57,
            Some('>') => return 25137,
            Some(_)   => panic!("illegal character unknown! {:?}", illegal_char),

            //correct and incomplete
            None => return 0,
        }
    }
}

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let mut error_score = 0;
    for l in f.lines() {
        let r = check_line(l);
        match r {
            0 => error_score +=0 ,
            3 | 57 | 1197 | 25137 => error_score+= r,
            _ => panic!("return unknown!"),
        }
    }
    println!("Total scores {}", error_score);
}
