impl Solution {
    pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        let mut temp_p: Vec<i32> = players.to_vec();
        let mut temp_t: Vec<i32> = trainers.to_vec();
        temp_p.sort_by(|a, b| b.cmp(a));
        temp_t.sort_by(|a, b| b.cmp(a));
        // println!("P:{:?}",temp_p);
        // println!("T:{:?}",temp_t);
        let mut i_p: usize = 0;
        let mut i_t: usize = 0;
        let mut count: i32 = 0;

        while i_p < temp_p.len() && i_t < temp_t.len() {
            if temp_p[i_p] > temp_t[i_t] {
                i_p += 1;
                continue;
            }
            if temp_t[i_t] >= temp_p[i_p] {
                i_p += 1;
                i_t += 1;
                count += 1;
                continue;
            }
        }

        count
    }
}
