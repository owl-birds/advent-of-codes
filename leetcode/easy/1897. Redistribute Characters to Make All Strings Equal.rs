use std::collections::HashMap;
impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut chars_count: HashMap<char, i32> = HashMap::new();
        for i in 0..words.len() {
            for c in words[i].chars() {
                if let Some(count) = chars_count.get_mut(&c) {
                    *count += 1;
                    continue;
                }
                chars_count.insert(c, 1);
            }
        }

        for (_, count) in chars_count.iter() {
            if count % (words.len() as i32) != 0 {
                return false;
            }
        }

        true
    }
}
