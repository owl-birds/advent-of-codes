use std::collections::HashMap;
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut s_map: HashMap<char, i32> = HashMap::new();
        let mut t_map: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            if let Some(freq) = s_map.get_mut(&c) {
                *freq += 1;
                continue;
            }
            s_map.insert(c, 1);
        }
        for c in t.chars() {
            if let Some(freq) = t_map.get_mut(&c) {
                *freq += 1;
                continue;
            }
            t_map.insert(c, 1);
        }
        let mut result: char = '1';
        for (c, c_freq) in t_map.iter() {
            if let Some(s_freq) = s_map.get(&c) {
                if s_freq != c_freq {
                    return *c;
                }
                continue;
            }
            result = *c;
            break;
        }

        result
    }
}
