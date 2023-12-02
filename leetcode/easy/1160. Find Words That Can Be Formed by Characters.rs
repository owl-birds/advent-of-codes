use std::collections::HashMap;
impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut chars_freq: HashMap<char, i32> = HashMap::new();
        for c in chars.chars() {
            if let Some(freq) = chars_freq.get_mut(&c) {
                *freq += 1;
                continue;
            }
            chars_freq.insert(c, 1);
        }
        let mut count: i32 = 0;
        for word in &words {
            let temp: Vec<char> = word.chars().collect();
            if Self::is_good(&chars_freq, &temp) {
                count += (word.len()) as i32;
            }
        }

        count
    }
    pub fn is_good(chars_freq: &HashMap<char, i32>, word: &Vec<char>) -> bool {
        let mut temp: HashMap<char, i32> = chars_freq.clone();
        let mut count: i32 = 0;
        for i in 0..word.len() {
            if let Some(freq) = temp.get_mut(&word[i]) {
                if *freq == 0 {
                    return false;
                }
                *freq -= 1;
                continue;
            }

            return false;
        }
        true
    }
}
