use regex::Regex;
use std::collections::HashSet;
use std::time::Instant;

fn count_all(ps:&Vec<&str>, sets: HashSet<&str>) -> usize{
    let re_p = Regex::new(r"(\w+{3}):(\S+)").unwrap();
    let mut vp: usize = 0;
    for p in ps {
        let caps = re_p.captures_iter(&p);
        let caps_id: HashSet<&str> = caps.map(|c| c.get(1).unwrap().as_str()).collect();
        let intersection: HashSet<&&str> = sets.intersection(&caps_id).collect();
        
        if intersection.len() >= 7 {
            vp += 1;
        }
    }
    return vp
}

fn check_valid(k: &str, v: &str) -> bool {
    let re_hgt = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    let re_hcl = Regex::new(r"^#([0-9a-f]{6})$").unwrap();
    let re_ecl = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    let re_pid = Regex::new(r"^\d{9}$").unwrap();

    let m = match k {
        "byr" => {
            let range = 1920..2003;
            range.contains(&v.parse().unwrap()) 
        }
        "iyr" => {
            //println!("{:?}",v.parse::<usize>().unwrap());
            let range = 2010..2021;
            range.contains(&v.parse().unwrap()) 
        }
        "eyr" => (2020..2031).contains(&v.parse().unwrap()),
        "hgt" => {
            //println!("{:?}",v);
            if re_hgt.is_match(&v) {
                let num: usize = re_hgt.captures(&v).unwrap().get(1).unwrap().as_str().parse().unwrap();
                let unit: &str = re_hgt.captures(&v).unwrap().get(2).unwrap().as_str();
                if unit == "in" {(59..77).contains(&num)} 
                else {(150..194).contains(&num)}    
            } else { false }
        }
        "hcl" => re_hcl.is_match(&v),
        "ecl" => re_ecl.is_match(&v),
        "pid" => re_pid.is_match(&v),
        "cid" => true,
        _ => panic!("Unknown key!"),
    };
    return m
}

pub fn run() {
    let time = Instant::now();

    let stdin = std::fs::read_to_string("./data/input_d04.txt").unwrap();
    let ps = stdin.split("\n\n").collect::<Vec<&str>>();
    let sets: HashSet<&str> = ["byr","iyr","eyr","hgt","hcl","ecl","pid"].iter().cloned().collect();
    //let vp = count_all(&ps, sets);
    //println!("Valid passports: {}", vp)

    let re_p = Regex::new(r"(\w+{3}):(\S+)").unwrap();
    let mut vp: usize = 0;
    let mut vp2: usize = 0;
    for p in ps {
        let caps = re_p.captures_iter(&p);

        let passport_info: Vec<(&str,&str)> = caps
            .map(|c| (c.get(1).unwrap().as_str(),
                      c.get(2).unwrap().as_str()))
            .collect();
        let caps_id: HashSet<&str> = passport_info.iter().map(|(k, _)| *k).collect();
        let intersection: HashSet<&&str> = sets.intersection(&caps_id).collect();
            
        if intersection.len() >= 7 {
            vp += 1;
            if passport_info.iter().all(|(k,v)| check_valid(k,v)) {
                vp2 += 1;
            }
        }
    }
    println!("Valid passports: {}", vp2);
    
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Elapsed: {:.3} ms", elapsed_ms);
}