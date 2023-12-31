// use std::collections::HashSet;
impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut min_time: i32 = 0;
        let mut idx: usize = 0;
        let mut colors_chars: Vec<char> = colors.chars().collect();

        while idx < colors_chars.len() {
            if idx > 0 && idx < colors_chars.len() && colors_chars[idx] == colors_chars[idx - 1] {
                let mut start_idx: usize = idx - 1;
                while idx < colors_chars.len() && colors_chars[idx] == colors_chars[idx - 1] {
                    idx += 1;
                }
                let mut temp_time_vec: Vec<i32> = needed_time[start_idx..idx].to_vec();
                temp_time_vec.sort_by(|a, b| a.cmp(&b));
                let mut count_time: i32 = 0;
                for i in 0..temp_time_vec.len() - 1 {
                    count_time += temp_time_vec[i];
                }
                // println!("{} - {}", start_idx, idx);
                // println!("{:?}", temp_time_vec);
                min_time += count_time;
                continue;
            }

            idx += 1;
        }

        min_time
    }
}
