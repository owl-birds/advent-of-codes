use std::collections::HashMap;
impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let mut freq_map: HashMap<char, i32> = HashMap::new();
        for c in s_chars {
            if let Some(freq) = freq_map.get_mut(&c) {
                *freq += 1;
            } else {
                freq_map.insert(c, 1);
            }
        }
        let mut freqs: Vec<i32> = vec![];
        for (_, freq) in freq_map.iter() {
            freqs.push(*freq);
        }
        freqs.sort_by(|a, b| b.cmp(a));
        // println!("{:?}", freq_map);
        // println!("before:{:?}", freqs);

        let mut count: i32 = 0;

        for i in 0..freqs.len() - 1 {
            if freqs[i] == 0 {
                break;
            }
            let mut n_i: usize = i + 1;
            while n_i < freqs.len() && freqs[i] == freqs[n_i] {
                freqs[n_i] -= 1;
                n_i += 1;
                count += 1;
            }
        }
        // println!("after:{:?}", freqs);

        count
    }
}
