use regex::Regex;

struct PasswordSpec {
    min: usize,
    max: usize,
    c: char,
    password: String
}

fn solve1(passwords: &Vec<PasswordSpec>) -> usize {
    return passwords
        .into_iter()
        .filter(|&p| {
            let occurrences: usize = p.password.chars().into_iter()
                .filter(|&c| c == p.c)
                .count();
            (p.min..p.max+1).contains(&occurrences)
        })
        .count();
}

fn solve2(passwords: &Vec<PasswordSpec>) -> usize {
    return passwords
        .into_iter()
        .filter(|&p| {
            let chars: Vec<char> = p.password.chars().collect();
            (chars[p.min-1] == p.c) ^ (chars[p.max-1] == p.c)
        })
        .count();
}

pub fn run() {
    let regex = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<c>\w): (?P<password>\w+)").unwrap();
    let input: Vec<PasswordSpec> = std::fs::read_to_string("./data/d02.txt").unwrap()
        .lines()
        .map(|l| {
            let result = regex.captures(l).unwrap();
            PasswordSpec {
                min: result.name("min").unwrap().as_str().parse().unwrap(),
                max: result.name("max").unwrap().as_str().parse().unwrap(),
                c: result.name("c").unwrap().as_str().chars().next().unwrap(),
                password: String::from(result.name("password").unwrap().as_str()),
            }
        })
        .collect();

    println!("Part 1: {}", solve1(&input));
    println!("Part 2: {}", solve2(&input));
}


