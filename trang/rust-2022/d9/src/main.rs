use std::collections::HashSet;

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let mut pos_visited: HashSet<(i32,i32)> = HashSet::new();
    let mut h: (i32, i32) = (0,0);
    let mut t: (i32, i32) = (0,0);
    pos_visited.insert(t);

    f.lines().for_each(|l|{
        let items: Vec<&str> = l.split(" ").collect();
        let n_steps = items[1].parse::<i32>().unwrap();
        match items[0] {
            "U" => {
                for _ in 1..=n_steps {
                    h.0 += 1;
                    if (h.0-t.0).abs() > 1 {
                        t.0 += 1;
                        if t.1 != h.1 {t.1 = h.1};
                        pos_visited.insert(t);
                    };
                    //println!("{:?}, {:?}", h, t);
                }
            },
            "D" => {
                for _ in 1..=n_steps {
                    h.0 -= 1;
                    if (h.0-t.0).abs() > 1 {
                        t.0 -= 1;
                        if t.1 != h.1 {t.1 = h.1};
                        pos_visited.insert(t);
                    };
                    //println!("{:?}, {:?}", h, t);
                }
            },
            "L" => {
                for _ in 1..=n_steps {
                    h.1 -= 1;
                    if (h.1-t.1).abs() > 1 {
                        t.1 -= 1;
                        if t.0 != h.0 {t.0 = h.0};
                        pos_visited.insert(t);
                    };
                    //println!("{:?}, {:?}", h, t);
                }
            },
            "R" => {
                for _ in 1..=n_steps {
                    h.1 += 1;
                    if (h.1-t.1).abs() > 1 {
                        t.1 += 1;
                        if t.0 != h.0 {t.0 = h.0};
                        pos_visited.insert(t);
                    };
                    //println!("{:?}, {:?}", h, t);
                }
            },
            _ => panic!("not a direction!"),
        };
    });
    println!("Number of position visited: {}", pos_visited.len());
}
