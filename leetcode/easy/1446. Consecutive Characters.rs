impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut max: i32 = 0;
        let s_chars: Vec<char> = s.chars().collect();

        let mut idx: usize = 0;

        while idx < s_chars.len() {
            let mut temp_count: i32 = 0;
            let mut curr_char: char = s_chars[idx];
            while idx < s_chars.len() && s_chars[idx] == curr_char {
                temp_count += 1;
                idx += 1;
            }

            if temp_count > max {
                max = temp_count;
            }
        }

        max
    }
}
