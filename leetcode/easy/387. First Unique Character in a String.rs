use std::collections::HashMap;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut char_freq: HashMap<char, i32> = HashMap::new();
        let mut idx: i32 = -1;
        for c in s.chars() {
            if let Some(freq) = char_freq.get_mut(&c) {
                *freq += 1;
            } else {
                char_freq.insert(c, 1);
            }
        }
        let mut temp_idx: usize = 0;
        for c in s.chars() {
            if let Some(freq) = char_freq.get(&c) {
                if *freq == 1 {
                    idx = temp_idx as i32;
                    break;
                }
            }
            temp_idx += 1;
        }

        idx
    }
}
