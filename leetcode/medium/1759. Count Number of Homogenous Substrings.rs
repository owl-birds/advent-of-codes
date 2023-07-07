impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let mut count: i64 = 0;
        // Sn = n/2 [a1 + an]
        let s_chars: Vec<char> = s.chars().collect();
        let mut idx: usize = 0;
        while idx < s_chars.len() {
            let mut temp_count: i64 = 0;
            let mut curr_char: char = s_chars[idx];
            while idx < s_chars.len() && s_chars[idx] == curr_char {
                temp_count += 1;
                idx += 1;
            }
            // println!("{}" ,temp_count);
            // println!("{}" , (temp_count) * (1 + temp_count)/2);
            let count_appeared: i64 = (temp_count) * (1 + temp_count) / 2;
            count += count_appeared;
        }

        (count % (i64::pow(10, 9) + 7)) as i32
    }
}
