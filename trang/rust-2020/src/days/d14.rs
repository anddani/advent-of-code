use regex::Regex;
use std::collections::BTreeMap;

fn parse_mask(s: &str) -> (u64,u64) {
    let mut mask_and = 0xFFFFFFFFF;
    let mut mask_or: u64 = 0;
    let mut not_and: u64 = 0;
    //let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    s.chars().enumerate().for_each(|(i,c)| match c {
        'X' => {},
        '0' => not_and |= 1 << (35 - i), // bitshift by (35 - i) 
        '1' => mask_or |= 1 << (35 - i), // |= is 'or equal'; mask_or = mask_or | 1 << (35 - i),
        _ => panic!("blah"),
    });
    mask_and = !not_and;
    return (mask_and, mask_or)
}

pub fn run() {
    let f = std::fs::read_to_string("./data/input_d14.txt").unwrap();
    let re = Regex::new(r"mask = (?P<mask>[0|1|X]{36})|mem\[(?P<addr>\d+)\] = (?P<value>\d+)").unwrap();
    let mut mem = BTreeMap::new();

    let mut m_and: u64 = 0;
    let mut m_or: u64 = 0;
    for l in f.lines() {
        let c = re.captures(l).unwrap();
        if l.starts_with("mask") {
            let m =  c.name("mask").unwrap().as_str();
            println!("{}", m);
            let (a, o) = parse_mask(m);
            m_and = a;
            m_or = o;
        } else {
            let addr: u64 = c.name("addr").unwrap().as_str().parse().unwrap();
            let value: u64 = c.name("value").unwrap().as_str().parse().unwrap();
            println!("{},{}", addr, value);  
            *mem.entry(addr).or_default() = (value & m_and) | m_or;
        }
    }
    println!("{:?}", mem.values().sum::<u64>());
}