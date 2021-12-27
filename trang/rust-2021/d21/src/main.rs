use cached::proc_macro::cached;

fn practice_game() {
    let mut p1_pos = 8;
    let mut p2_pos = 7;

    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut dice = 0;
    loop {
        p1_pos += (dice+1)%100 + (dice+2)%100 + (dice+3) %100; 
        p1_score += if p1_pos%10 == 0 {10} else {p1_pos%10};
        dice +=3;
        if p1_score >= 1000 {
            break
        } 

        p2_pos += (dice+1)%100 + (dice+2)%100 + (dice+3) %100;  
        p2_score += if p2_pos%10 == 0 {10} else {p2_pos%10};
        dice +=3;
        if p2_score >= 1000 {
            break
        } 
    }
    
    let loser_score = if p1_score>p2_score {p2_score} else {p1_score};
    println!("Loser score {}, rolled dice {}, multiplied score: {}", loser_score, dice,loser_score*dice);
}

fn real_game() {
    let p1_pos = 8;
    let p2_pos = 7;

    let (p1_wins, p2_wins) = roll_dice(p1_pos, p2_pos, 0, 0, true);
    println!("{},{}, max: {}", p1_wins, p2_wins, p1_wins.max(p2_wins));
}

#[cached] // stored most recent values returned by this func, so no repeated work
fn roll_dice(p1_pos: usize, p2_pos: usize, 
             p1_score: usize, p2_score: usize,
             p1_turn: bool) -> (i64, i64) {
    if p1_score >= 21 {
        return (1,0)
    }
    if p2_score >= 21 {
        return (0,1)
    }

    let alternatives: Vec<_> = (1..=3)
                        .flat_map(|a| (1..=3).flat_map(move |b| (1..=3).map(move |c| a + b + c)))
                        .collect();

    alternatives.iter()
                .map(|r| {
                    if p1_turn {
                        let p1_pos =  if (p1_pos + r) %10 == 0 {10} else {(p1_pos + r)%10};               
                        let p1_score = p1_score + p1_pos;
                        roll_dice(p1_pos, p2_pos, p1_score, p2_score, false)
                    } else {
                        let p2_pos =  if (p2_pos + r) %10 == 0 {10} else {(p2_pos + r)%10};
                        let p2_score = p2_score + p2_pos;   
                        roll_dice(p1_pos, p2_pos, p1_score, p2_score, true)
                    }
                })
                .fold((0,0), |total_wins, win| (total_wins.0 + win.0, total_wins.1 + win.1))
}

fn main() {
    practice_game();    
    real_game();
}