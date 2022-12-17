fn check_visible(grid: &Vec<Vec<usize>>, row: usize, col: usize) -> usize {
    let mut n_dir = 4;
    let current_height = grid[row][col];
    for new_r in (0..row).rev() {
        if grid[new_r][col] >= current_height {
            n_dir -= 1;
            break
        }
    };
    for new_r in (row+1)..99 {
        if grid[new_r][col] >= current_height {
            n_dir -= 1;
            break
        }
    }   
    for new_c in (0..col).rev() {
        if grid[row][new_c] >= current_height {
            n_dir -= 1;
            break
        }
    }   
    for new_c in (col+1)..99 {
        if grid[row][new_c] >= current_height {
            n_dir -= 1;
            break
        }
    }   
    return n_dir
}

fn main() {
    let f = std::fs::read_to_string("./input.txt").unwrap();
    let max_r = f.lines().count();
    let max_c = f.lines().next().unwrap().chars().count();
    println!("{}, {}", max_r, max_c);
    let grid: Vec<Vec<usize>> = f.lines().map(|line| {
        line.chars().map(|c| c as usize - '0' as usize).collect()
    }).collect();
    
    println!("{:?}", grid);
    let mut count_visible = 0;
    for r in 0..max_r {
        for c in 0..max_c {
            if r == 0 || r == (max_r-1) || c == 0 || c == (max_c-1) {
                count_visible += 1;
            } else {
                let n_directions = check_visible(&grid, r, c);
                if n_directions > 0 { count_visible += 1 };
                //if n_directions > 0 { println!("found1") };
            }
        }
    };
    println!("Number of trees visible in at least 1 direction {}",count_visible);
}
