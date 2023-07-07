use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        if s.len() < 3 {
            return 0;
        }
        let mut count_good: i32 = 0;
        let s_chars: Vec<char> = s.chars().collect();

        let mut idx: usize = 0;
        let mut window_map: HashMap<char, i32> = HashMap::new();
        // let mut window_set: HashSet<char> = HashSet::new();

        while idx < 3 {
            if let Some(count) = window_map.get_mut(&s_chars[idx]) {
                *count += 1;
            } else {
                window_map.insert(s_chars[idx], 1);
            }
            idx += 1;
        }
        if window_map.len() == 3 {
            count_good += 1
        }
        // println!("{:?}", window_map);
        while idx < s_chars.len() {
            if let Some(count) = window_map.get_mut(&s_chars[idx]) {
                *count += 1;
                let mut start_count: i32 = 0;
                if let Some(c_s) = window_map.get(&s_chars[idx - 3]) {
                    start_count = *c_s;
                }
                if start_count == 1 {
                    window_map.remove(&s_chars[idx - 3]);
                } else {
                    window_map.insert(s_chars[idx - 3], start_count - 1);
                }
                if window_map.len() == 3 {
                    count_good += 1;
                }
                // println!("{:?}", window_map);
                idx += 1;
                continue;
            }
            window_map.insert(s_chars[idx], 1);
            let mut start_count: i32 = 0;
            if let Some(c_s) = window_map.get(&s_chars[idx - 3]) {
                start_count = *c_s;
            }
            if start_count == 1 {
                window_map.remove(&s_chars[idx - 3]);
            } else {
                window_map.insert(s_chars[idx - 3], start_count - 1);
            }
            if window_map.len() == 3 {
                count_good += 1
            }
            // println!("{:?}", window_map);
            idx += 1;
        }

        count_good

        // // without sliding window
        // let mut idx: usize = 0;
        // while idx < (s_chars.len()-2) {
        //     let mut window_set: HashSet<char> = HashSet::new();
        //     let mut temp_idx = idx;
        //     while temp_idx < (idx + 3) && !window_set.contains(&s_chars[temp_idx]) {
        //         window_set.insert(s_chars[temp_idx]);
        //         temp_idx += 1;
        //     }
        //     if window_set.len() == 3 {count_good += 1}
        //     idx += 1;
        // }

        // count_good
    }
}
