use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut let_freq: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            if let Some(freq) = let_freq.get_mut(&c) {
                *freq += 1;
                continue;
            }
            let_freq.insert(c, 1);
        }
        let mut count: i32 = 0;
        for c in t.chars() {
            // println!("{:?}-{}-{}", let_freq,c, count);
            if let Some(freq) = let_freq.get_mut(&c) {
                *freq -= 1;
                if *freq == 0 {
                    let_freq.remove(&c);
                }
                count += 1;
                continue;
            }
            break;
        }

        count == s.len() as i32
    }
}
