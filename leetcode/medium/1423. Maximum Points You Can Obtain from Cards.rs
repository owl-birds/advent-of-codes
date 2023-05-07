impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let mut total_points: i32 = 0;
        for i in 0..card_points.len(){
            total_points += card_points[i];
        }
        // println!("{}", total_points);
        if k as usize == card_points.len(){return total_points}

        // start -> end -> start -> end -> ... ||| or can start -> start -> end -> ... ||| end -> end -> end -> start -> ...
        let mut window_points: i32 = 0; // have a length of n-k
        let window_length: usize = card_points.len()-k as usize;

        let mut idx: usize = 0;
        while idx < (card_points.len()-k as usize) {
            window_points += card_points[idx];
            idx += 1;
        }
        // println!("{}, idx {}", window_points, idx);

        let mut score: i32 = total_points - window_points;
        while idx < card_points.len(){
            window_points += card_points[idx]; // new card
            window_points -= card_points[idx - window_length]; // throw the first card
            let temp_score = total_points - window_points;
            // println!("{}", temp_score);
            if temp_score > score{score = temp_score}
            idx += 1;
        }
        


        score
    }
}