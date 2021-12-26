fn main() {
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
