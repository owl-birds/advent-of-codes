use std::collections::HashMap;
impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut char_prev_idx: HashMap<char, usize> = HashMap::new();
        let mut longest: i32 = -1;

        let mut idx: usize = 0;
        for c in s.chars() {
            if let Some(prev_idx) = char_prev_idx.get_mut(&c) {
                let substr_len: i32 = (idx - *prev_idx - 1) as i32;
                if substr_len > longest {
                    longest = substr_len
                }
            } else {
                char_prev_idx.insert(c, idx);
            }
            idx += 1;
        }

        longest
    }
}
