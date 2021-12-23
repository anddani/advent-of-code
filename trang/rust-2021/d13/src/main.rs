use std::collections::HashSet;
use regex::Regex;

fn fold(paper: &HashSet<(usize, usize)>, xy: &str, pos: usize) -> HashSet<(usize, usize)> {
    let mut folded_paper = paper.clone();
    for p in paper.iter() {
        match xy {
            "x" => if p.0 > pos { 
                folded_paper.remove(&p);
                folded_paper.insert((p.0 - (p.0-pos)*2, p.1)); 
            },
            "y" => if p.1 > pos { 
                folded_paper.remove(&p);
                folded_paper.insert((p.0, p.1 - (p.1-pos)*2)); 
            },
            _ => panic!("Pattern not covered {}", xy),
        }
    }
    //println!("{:?}", folded_paper);
    return folded_paper
}

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let mut paper: HashSet<(usize, usize)> = HashSet::new();

    f.lines()
        .take_while(|l| !l.is_empty())
        .for_each(|l| {
            let coord:Vec<usize> = l.split(',').map(|t| t.parse::<usize>().unwrap()).collect();
            paper.insert((coord[0], coord[1]));
        });
    println!("{}", paper.len());

    let re_instruction = Regex::new(r"fold along (?P<xy>\w)=(?P<pos>\d+)").unwrap();
    let mut i = 0;
    f.lines().skip(paper.len()).skip(1)
                        .for_each(|l| {
                            let c = re_instruction.captures(&l).unwrap();
                            let xy = c.name("xy").unwrap().as_str();
                            let pos = c.name("pos").unwrap().as_str().parse::<usize>().unwrap();
                            paper = fold(&paper, xy, pos);
                            i += 1;
                            println!("at fold {}={}, number of visible dots is {}", xy, pos, paper.len());
                        });

}
