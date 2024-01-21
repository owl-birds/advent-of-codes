use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut winner_map: HashMap<i32, i32> = HashMap::new();
        let mut loser_map: HashMap<i32, i32> = HashMap::new();
        let mut players_set: HashSet<i32> = HashSet::new();
        for i in 0..matches.len() {
            if let Some(freq) = winner_map.get_mut(&matches[i][0]) {
                *freq += 1;
            } else {
                winner_map.insert(matches[i][0], 1);
                players_set.insert(matches[i][0]);
            }
            if let Some(freq) = loser_map.get_mut(&matches[i][1]) {
                *freq += 1;
            } else {
                loser_map.insert(matches[i][1], 1);
                players_set.insert(matches[i][1]);
            }
        }
        // println!("{:?}\n{:?}\nplayers:{:?}", winner_map, loser_map, players_set);
        let mut result: Vec<Vec<i32>> = vec![vec![], vec![]];
        for player in players_set.iter() {
            if let Some(win_freq) = loser_map.get(player) {
                if *win_freq == 1 {
                    result[1].push(*player);
                }
                continue;
            }
            result[0].push(*player);
        }
        result[0].sort_by(|a, b| a.cmp(&b));
        result[1].sort_by(|a, b| a.cmp(&b));
        result
    }
}
