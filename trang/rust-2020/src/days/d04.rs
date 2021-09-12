use std::io::{BufRead, BufReader};
use std::fs::File;
use regex::Regex;

pub fn run() {
    let f = BufReader::new(File::open("./data/input_d04.txt").unwrap());
    let re_byr = Regex::new(r"(19\d|20\d)").unwrap();
    let re_iyr = Regex::new(r"").unwrap();
    let re_eyr = Regex::new().unwrap();
    let re_hgt = Regex::new().unwrap();
    let re_hcl = Regex::new().unwrap();
    let re_ecl = Regex::new().unwrap();
    let re_pid = Regex::new().unwrap();
    let re_cid = Regex::new().unwrap();

    let mut vp = 0;
    
    for l in f.lines() {
        let l = l.unwrap();

        let  = match key {
            "byr" => re_byr,
            "iyr" => re_iyr,
            "eyr" => re_eyr,
            "hgt" => re_hgt,
            "hcl" => re_hcl,
            "ecl" => re_ecl,
            "pid" => re_pid,
            "cid" => re_cid,
            _ => panic!("Unknown key!"),
        }
    }
    println!("Valid passports: {}", vp)
}