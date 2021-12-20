fn decode(lines: &Vec<&str>, i: usize) -> (char, char) {
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

fn find_bit(lines_int: &Vec<u32>, i:usize) -> (usize, usize) {
    let v = lines_int.iter().map(|x| (x >> i) & 1).collect::<Vec<u32>>();
    let count_1 = v.iter().filter(|&n| *n == 1).count();
    let count_0 = v.iter().filter(|&n| *n == 0).count();
    return (count_1, count_0)
}

fn find_oxygen_generator_rating(lines_int: &Vec<u32>, number_length:usize) -> u32 {
    let mut lines_remained = lines_int.clone();
    for i in (0..number_length).rev() {
        let (c_1, c_0) = find_bit(&lines_remained, i);
        println!("ones {}, zeroes {}", c_1,c_0);
        lines_remained = lines_remained
                    .into_iter()
                    .filter(|x| {
                        let maj = if c_1 > c_0 || c_1 == c_0 {1} else {0};
                        (x >> i) & 1 == maj
                    })
                    .collect();
                    
        if lines_remained.len() == 1 {
            println!("{:?}", lines_remained);
            return *lines_remained.get(0).unwrap();
        }
    }
    panic!("Aaaaaaaahhhhh")
}


fn find_CO2_scrubber_rating(lines_int: &Vec<u32>, number_length:usize) -> u32 {
    let mut lines_remained = lines_int.clone();
    for i in (0..number_length).rev() {
        let (c_1, c_0) = find_bit(&lines_remained, i);
        println!("ones {}, zeroes {}", c_1,c_0);
        lines_remained = lines_remained
                    .into_iter()
                    .filter(|x| {
                        let keep = if c_1 > c_0 || c_1 == c_0 {0} else {1};
                        (x >> i) & 1 == keep
                    })
                    .collect();

        if lines_remained.len() == 1 {
            println!("{:?}", lines_remained);
            return *lines_remained.get(0).unwrap();
        }
    }
    panic!("Aaaaaaaahhhhh")

}

fn main() {
    let run_part_1 = false;
    let run_part_2 = true;
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<&str> = f.lines().collect::<Vec<&str>>();
    let lines_int: Vec<u32> = lines.iter().map(|x| u32::from_str_radix(&x, 2).unwrap()).collect();
    let number_length: usize = lines.get(0).unwrap().chars().count();

    if run_part_1 {
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

    if run_part_2 {
        let oxygen = find_oxygen_generator_rating(&lines_int, number_length);
        let co2 = find_CO2_scrubber_rating(&lines_int, number_length);
        println!("Oxygen {}, Co2 {}", oxygen, co2);
        println!("Life support {}", oxygen*co2);
    }

}
