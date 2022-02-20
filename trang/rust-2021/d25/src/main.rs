fn step(mut current_map: Vec<Vec<char>>) -> (Vec<Vec<char>>, usize) {
    let mut n_move = 0;
    
    // East moves first, then south
    for c_ in ['>','v'] {
        let old_map = current_map.clone();
        for (x, l) in old_map.iter().enumerate() {
            for (y, c) in l.iter().enumerate() {
                match (c_,c) {
                    ('>','>') => if old_map[x][(y+1) % l.len()] == '.' {
                        current_map[x][(y+1) % l.len()] = '>';
                        current_map[x][y] = '.';
                        n_move += 1;
                    },
                    ('v','v') => if old_map[(x+1) % old_map.len()][y] == '.' {
                        current_map[(x+1) % old_map.len()][y] = 'v';
                        current_map[x][y] = '.';
                        n_move += 1;
                    },
                    _ => continue,
                }
            }
        }
    }

    return (current_map, n_move)
}

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let mut sea_map: Vec<Vec<char>> = f.lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    //println!("{:#?}", sea_map);
    //println!("{} {}", sea_map.len(),(4+1) % sea_map.len());
    let mut n_move = 1;
    let mut n_step = 0;
    while n_move > 0 {
        (sea_map, n_move) = step(sea_map);
        n_step += 1;
        //println!("move:{}, step:{}, First pos {:?}",n_move,n_step, sea_map[0][0]);
    }
    
    println!("First step on which no sea cucumbers move: {}", n_step);    
    
}
