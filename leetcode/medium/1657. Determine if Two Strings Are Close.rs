use std::cmp::Ordering;
use std::collections::HashMap;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }
        let mut w1: Vec<char> = word1.chars().collect();
        let mut w2: Vec<char> = word2.chars().collect();
        // w1.sort_by(|a, b| {
        //     if a > b {
        //         return Ordering::Greater;
        //     }
        //     Ordering::Less
        // });
        // w2.sort_by(|a, b| {
        //     if a > b {
        //         return Ordering::Greater;
        //     }
        //     Ordering::Less
        // });
        // println!("{:?}\n{:?}", w1, w2);
        let mut w1_map: HashMap<char, i32> = HashMap::new();
        let mut w2_map: HashMap<char, i32> = HashMap::new();
        for i in 0..w1.len() {
            if let Some(freq) = w1_map.get_mut(&w1[i]) {
                *freq += 1;
            } else {
                w1_map.insert(w1[i], 1);
            }
            if let Some(freq) = w2_map.get_mut(&w2[i]) {
                *freq += 1;
            } else {
                w2_map.insert(w2[i], 1);
            }
        }
        // println!("----");
        // println!("{:?}\n{:?}", w1_map, w2_map);
        if w1_map.len() != w2_map.len() {
            return false;
        }
        let mut w1f_map: HashMap<i32, i32> = HashMap::new();
        let mut w2f_map: HashMap<i32, i32> = HashMap::new();
        for (c, f1) in w1_map.iter() {
            if !w2_map.contains_key(c) {
                return false;
            }
            if let Some(w1f) = w1f_map.get_mut(f1) {
                *w1f += 1;
            } else {
                w1f_map.insert(*f1, 1);
            }
            if let Some(f2) = w2_map.get_mut(c) {
                if let Some(w2f) = w2f_map.get_mut(f2) {
                    *w2f += 1;
                } else {
                    w2f_map.insert(*f2, 1);
                }
            }
        }
        // println!("----");
        // println!("{:?}\n{:?}", w1f_map, w2f_map);
        for (w1f, freq_1) in w1f_map.iter() {
            if let Some(freq_2) = w2f_map.get(w1f) {
                if freq_1 != freq_2 {
                    return false;
                }
                continue;
            }
            return false;
        }
        true
    }
}
