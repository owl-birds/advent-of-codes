use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        // remember we concern only on the middle char, because the length of the
        // palindrome string is only three chars

        // prefix sum on HashMap Array
        let mut pref_sum: Vec<HashMap<char, i32>> = vec![HashMap::new(); s.len()];
        let mut next_sum: Vec<HashMap<char, i32>> = vec![HashMap::new(); s.len()];
        let mut palin_set: HashSet<String> = HashSet::new();
        let mut count: i32 = 0;
        let mut s_chars: Vec<char> = s.chars().collect();
        // println!("{:?}", pref_sum);
        for i in 0..s_chars.len() {
            if i == 0 {
                pref_sum[i].insert(s_chars[i], 1);
                continue;
            }
            let mut temp_map = pref_sum[i - 1].clone();
            if !temp_map.contains_key(&s_chars[i]) {
                temp_map.insert(s_chars[i], 1);
            } else if let Some(freq) = temp_map.get_mut(&s_chars[i]) {
                *freq += 1;
            }
            pref_sum[i] = temp_map;
        }
        // println!("{:?}", pref_sum);
        // println!("{:?}", next_sum);
        let mut i = s.len();
        let mut j: usize = 0;
        while i > 0 {
            i -= 1;
            if i == s.len() - 1 {
                next_sum[j].insert(s_chars[i], 1);
                j += 1;
                continue;
            }
            let mut temp_map = next_sum[j - 1].clone();
            if !temp_map.contains_key(&s_chars[i]) {
                temp_map.insert(s_chars[i], 1);
            } else if let Some(freq) = temp_map.get_mut(&s_chars[i]) {
                *freq += 1;
            }
            next_sum[j] = temp_map;
            j += 1;
        }
        next_sum.reverse();
        // println!("{:?}", next_sum);
        for i in 1..s_chars.len() - 1 {
            let pref: &HashMap<char, i32> = &pref_sum[i - 1];
            let next: &HashMap<char, i32> = &next_sum[i + 1];
            for (letter, freq_p) in pref.iter() {
                if let Some(freq_n) = next.get(&letter) {
                    let mut curr_str = String::new();
                    curr_str.push(*letter);
                    curr_str.push(s_chars[i]);
                    curr_str.push(*letter);
                    if !palin_set.contains(&curr_str) {
                        count += 1;
                    }
                    palin_set.insert(curr_str);
                }
            }
            // println!("--{}", count);
        }
        count
    }
}
