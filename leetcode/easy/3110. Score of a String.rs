impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut score: i32 = 0;
        let mut temp: Vec<char> = s.chars().collect();

        for i in 1..temp.len() {
            score += (temp[i] as u32).abs_diff(temp[i - 1] as u32) as i32;
        }

        score
    }
}
