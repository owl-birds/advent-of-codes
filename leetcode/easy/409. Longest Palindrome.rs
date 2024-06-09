use std::collections::HashMap;
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut letters_map: HashMap<char, i32> = HashMap::new();
        let mut length: i32 = 0;

        for c in s.chars() {
            if let Some(freq) = letters_map.get_mut(&c) {
                *freq += 1;
            } else {
                letters_map.insert(c, 1);
            }
        }
        // println!("{:?}", letters_map);
        let mut is_middle: bool = false;

        for (letter, freq) in letters_map.iter() {
            if freq % 2 != 0 && !is_middle {
                length += freq;
                is_middle = true;
                continue;
            }
            if freq % 2 == 0 {
                length += freq;
            } else {
                length += (freq - 1);
            }
        }

        length
    }
}
