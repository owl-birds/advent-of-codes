impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut first: usize = 0;
        let mut second: usize = 1;
        let mut score: i32 = (values[first] + first as i32) + (values[second] - second as i32);
        // values[i] + values[j] + i - j
        // (values[i] + i) + (values[j] - j) instead of values[i] + values[j] + i - j
        // for i in 2..values.len() {
        //     if (values[first] + first as i32) > (values[i] + i as i32) {

        //     }
        // }

        while second < values.len() {
            let temp_score = (values[first] + first as i32) + (values[second] - second as i32);
            if temp_score > score {
                score = temp_score;
            }
            if (values[second] + second as i32) > (values[first] + first as i32) {
                first = second;
                second = first + 1;
                continue;
            }
            second += 1;
        }
        score
    }
}
