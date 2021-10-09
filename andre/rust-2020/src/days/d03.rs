
fn solve1(forest: &Vec<Vec<char>>, step: (usize, usize), coords: (usize, usize), trees_hit: usize) -> usize {
    let next_pos = (coords.0 + step.0, (coords.1 + step.1) % forest[0].len());
    if next_pos.0 > forest.len() - 1 {
        return trees_hit
    } else if forest[next_pos.0][next_pos.1] == '#' {
        return solve1(forest, step, next_pos, trees_hit + 1)
    } else {
        return solve1(forest, step, next_pos, trees_hit)
    }
}

fn solve2(forest: &Vec<Vec<char>>) -> usize {
    return solve1(forest, (1, 1), (0, 0), 0) * 
        solve1(forest, (1, 3), (0, 0), 0) *
        solve1(forest, (1, 5), (0, 0), 0) *
        solve1(forest, (1, 7), (0, 0), 0) *
        solve1(forest, (2, 1), (0, 0), 0);
}

pub fn run() {
    let forest: Vec<Vec<char>> = std::fs::read_to_string("./data/d03.txt").unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    println!("Part 1: {}", solve1(&forest, (1, 3), (0, 0), 0));
    println!("Part 2: {}", solve2(&forest));
}


