use std::time::Instant;
use regex::Regex;

// 'a is lifetime a
fn count_invalid<'a>(tickets: &'a Vec<usize>, valid_ranges: &'a Vec<(usize, usize, usize, usize)>) -> Vec<&'a usize> {
    let invalids = tickets.iter().filter(|&&t| 
        !valid_ranges
        .iter()
        .any(|r| (t >= r.0 && t <= r.1) || (t >= r.2 && t <= r.3))
    )
    .collect();
    
    return invalids
}

pub fn run() {
    let time = Instant::now();

    let f = std::fs::read_to_string("./data/input_d16.txt").unwrap();
    let re_range = Regex::new(r": (?P<l1>\d+)-(?P<h1>\d+) or (?P<l2>\d+)-(?P<h2>\d+)").unwrap();
    let valid_ranges: Vec<(usize, usize, usize, usize)> = f.lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let c = re_range.captures(&line).unwrap();
            (
                c.name("l1").unwrap().as_str().parse::<usize>().unwrap(),
                c.name("h1").unwrap().as_str().parse::<usize>().unwrap(),
                c.name("l2").unwrap().as_str().parse::<usize>().unwrap(),
                c.name("h2").unwrap().as_str().parse::<usize>().unwrap(),
            )
        })
        .collect();
    println!("{:?}", valid_ranges);

    
    let nearby_tickets: Vec<usize> = f.lines()
        .skip(valid_ranges.len() + 1)
        .skip_while(|line| !line.starts_with("nearby tickets:"))
        .skip(1)
        .flat_map(|line| {
            line.split(',').map(|t| t.parse::<usize>().unwrap())
        })
        .collect();
    //println!("{:?}", nearby_tickets);
    
    let er = count_invalid(&nearby_tickets, &valid_ranges);
    let sum: usize = er.iter().map(|&&t| t).sum::<usize>();
    //println!("Invalid tickets {:?}", er);
    println!("Ticket scanning error rate {}", sum);
    
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Elapsed: {:.3} ms", elapsed_ms);
}
