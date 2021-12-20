fn decode(lines: &Vec<&str>, i: usize) -> (char, char) {
    //let n_numbers = lines.len();   
    
    let mut zeroes = 0; 
    let mut ones = 0;
    for l in lines.iter() {
        let c = l.chars().nth(i).unwrap();
        match c {
            '0' => zeroes +=1,
            '1' => ones +=1,
            _  => panic!("Not yet implemented: {:?}", c),
        }
    }  
    if ones > zeroes {
        return ('1','0')
    } else {
        return ('0','1')
    } 
}

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<&str> = f.lines().collect::<Vec<&str>>();
    let number_length: usize = lines.get(0).unwrap().chars().count(); 
    println!("{}",number_length);
    println!("{:?}", lines.get(10).unwrap());
    let mut gamma: String = "".to_owned(); 
    let mut epsilon: String = "".to_owned(); 

    for i in 0..number_length {
        let (g,e) = decode(&lines,i);
        gamma.push(g);
        epsilon.push(e);
    }

    let gamma = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon, 2).unwrap();
    println!("Gamma {}, Epsilon {}", gamma,epsilon);
    println!("Power consumption {}", gamma*epsilon);
}
