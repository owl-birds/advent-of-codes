impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let mut c_vec: Vec<char> = customers.chars().collect();
        let mut temp_penalty: i32 = 0;
        let mut close_time: i32 = 0;
        for i in 0..c_vec.len() {
            if c_vec[i] == 'Y' {
                temp_penalty += 1;
                continue;
            }
        }
        // println!("close_time:{}\npenalty:{}", close_time, temp_penalty);

        let mut result: i32 = 0;
        let mut prev_penalty: i32 = temp_penalty;
        // inc close time by one
        for i in 0..c_vec.len() {
            if c_vec[i] == 'Y' {
                temp_penalty -= 1;
            } else {
                temp_penalty += 1;
            }
            close_time += 1;
            if temp_penalty < prev_penalty {
                result = close_time;
                prev_penalty = temp_penalty;
            }
        }

        result
    }
}
