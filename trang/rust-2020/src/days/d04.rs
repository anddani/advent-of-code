use regex::Regex;
use std::collections::HashSet;

pub fn run() {
    let stdin = std::fs::read_to_string("./data/input_d04.txt").unwrap();
    let ps = stdin.split("\n\n").collect::<Vec<&str>>();
    let re_p = Regex::new(r"(\w+{3}):(\S+)").unwrap();
    let sets: HashSet<&str> = ["byr","iyr","eyr","hgt","hcl","ecl","pid"].iter().cloned().collect();
    let mut vp = 0;
    for p in ps {
        let caps = re_p.captures_iter(&p);
        let caps_id: HashSet<&str> = caps.map(|c| c.get(1).unwrap().as_str()).collect();
        let intersection: HashSet<&&str> = sets.intersection(&caps_id).collect();
        
        if intersection.len() >= 7 {
            vp += 1;
        }
    }
    
    println!("Valid passports: {}", vp)
}