use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest: i32 = 0;
        let s_chars: Vec<char> = s.chars().collect();

        // SLIDING WINDOW
        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut window_map: HashMap<char, i32> = HashMap::new();

        while end < s_chars.len() {
            if let Some(count) = window_map.get_mut(&s_chars[end]) {
                *count += 1;
            } else {
                window_map.insert(s_chars[end], 1);
            }
            let mut window_size: i32 = (end - start) as i32 + 1;
            // println!("{:?}", window_map);
            if window_map.len() as i32 == window_size {
                longest = if window_size > longest {
                    window_size
                } else {
                    longest
                };
                end += 1;
            } else {
                while start < s_chars.len() && (window_map.len() as i32) < window_size {
                    let mut count_start: i32 = 0;
                    if let Some(c_s) = window_map.get(&s_chars[start]) {
                        count_start = *c_s;
                    }
                    if count_start == 1 {
                        window_map.remove(&s_chars[start]);
                    } else {
                        window_map.insert(s_chars[start], count_start - 1);
                    }
                    start += 1;
                    window_size = (end - start) as i32 + 1;
                }

                if window_map.len() as i32 == window_size {
                    longest = if window_size > longest {
                        window_size
                    } else {
                        longest
                    };
                }

                end += 1;
            }
        }
        longest

        // // without sliding window
        // let mut idx: usize = 0;
        // let mut window_set: HashSet<char> = HashSet::new();

        // while idx < s_chars.len() {
        //     let mut temp_idx: usize = idx;
        //     while temp_idx < s_chars.len() && !window_set.contains(&s_chars[temp_idx]) {
        //         window_set.insert(s_chars[temp_idx]);
        //         temp_idx += 1;
        //     }
        //     longest = if longest < window_set.len() as i32 {window_set.len() as i32} else {longest};
        //     window_set.clear();
        //     idx += 1;
        // }

        // longest
    }
}
