use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut chars_count: HashMap<char, i32> = HashMap::new();
        let mut sorted_chars: Vec<char> = vec![];
        for c in s.chars() {
            if let Some(count) = chars_count.get_mut(&c) {
                *count += 1;
                continue;
            }
            sorted_chars.push(c);
            chars_count.insert(c, 1);
        }
        sorted_chars.sort_by(|a, b| a.cmp(&b));
        // println!("{:?}", sorted_chars);

        let mut idx: usize = 0;
        let mut result: String = String::new();
        let mut is_increasing: bool = true;

        while result.len() != s.len() {
            if idx == sorted_chars.len() {
                is_increasing = false;
            }
            if idx == 0 {
                is_increasing = true;
            }
            if !is_increasing {
                idx -= 1;
            }

            // if let Some
            // if let Some(count) = chars_count.get_mut(&sorted_chars[idx]) {
            //     if *count > 0 {
            //         result.push(sorted_chars[idx]);
            //         *count -= 1;
            //     }
            // }
            // if let Some

            // MATCH
            let get_count: Option<&i32> = chars_count.get(&sorted_chars[idx]);

            match get_count {
                Some(count) => {
                    if *count > 0 {
                        result.push(sorted_chars[idx]);
                        chars_count.insert(sorted_chars[idx], *count - 1);
                    }
                }
                None => {}
            }
            // MATCH

            // println!("{}", idx);
            // println!("{:?}", chars_count);
            if is_increasing {
                idx += 1;
                continue;
            }
        }

        result
    }
}
